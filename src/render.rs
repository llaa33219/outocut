use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::parser;
use crate::models::*;
use crate::composition::Composer;
use crate::animation::Animator;

pub struct RenderEngine;

impl RenderEngine {
    pub async fn render(
        project_path: &Path,
        output_path: &Path,
        gpu: bool,
        preset: Option<String>,
        crf: Option<u8>,
    ) -> Result<()> {
        let project = parser::parse_project(project_path)?;

        let cache_dir = project_path.parent().unwrap_or(Path::new(".")).join(".outocut.cache");
        std::fs::create_dir_all(&cache_dir)?;

        let width = project.settings.width;
        let height = project.settings.height;
        let fps = project.settings.fps;
        let total_frames = (project.settings.duration * fps) as u32;

        for frame in 0..total_frames {
            let time = frame as f64 / fps;
            Self::render_frame(&project, time, width, height, &cache_dir, frame)?;
        }

        Self::encode_video(
            &cache_dir,
            output_path,
            width,
            height,
            fps,
            gpu,
            preset,
            crf,
        )?;

        Ok(())
    }

    fn render_frame(
        project: &Project,
        time: f64,
        width: u32,
        height: u32,
        cache_dir: &Path,
        frame: u32,
    ) -> Result<()> {
        let composer = Composer::new(project);

        let comp = project
            .compositions
            .get(&project.mainCompositionId)
            .ok_or_else(|| anyhow::anyhow!("Main composition not found"))?;

        let mut frame_data = vec![0u8; (width * height * 4) as usize];

        let bg_color = hex_to_rgba(&project.settings.backgroundColor);
        for pixel in frame_data.chunks_mut(4) {
            pixel[0] = bg_color.0;
            pixel[1] = bg_color.1;
            pixel[2] = bg_color.2;
            pixel[3] = bg_color.3;
        }

        for layer in &comp.layers {
            if !Animator::is_layer_active(layer, time) {
                continue;
            }

            if !layer.enabled {
                continue;
            }

            let transform = Animator::evaluate_transform(&layer.transform, time);
            let opacity = Animator::evaluate_f64(&layer.opacity, time);

            Self::render_layer(
                layer,
                &transform,
                opacity / 100.0,
                time,
                width,
                height,
                &mut frame_data,
                &composer,
            )?;
        }

        let frame_path = cache_dir.join(format!("{:06}.png", frame));
        Self::save_frame_png(&frame_path, &frame_data, width, height)?;

        Ok(())
    }

    fn render_layer(
        layer: &Layer,
        transform: &crate::animation::ComputedTransform,
        opacity: f64,
        time: f64,
        width: u32,
        height: u32,
        frame_data: &mut [u8],
        composer: &Composer,
    ) -> Result<()> {
        let layer_width = width.min(1920);
        let layer_height = height.min(1080);
        let mut layer_data = vec![0u8; (layer_width * layer_height * 4) as usize];

        match &layer.content {
            Some(LayerContent::Text(text_content)) => {
                Self::render_text(text_content, &mut layer_data, layer_width, layer_height)?;
            }
            Some(LayerContent::Solid(solid_content)) => {
                let color = hex_to_rgba(&solid_content.color);
                for pixel in layer_data.chunks_mut(4) {
                    pixel[0] = color.0;
                    pixel[1] = color.1;
                    pixel[2] = color.2;
                    pixel[3] = color.3;
                }
            }
            _ => {}
        }

        if let Some(shape_contents) = &layer.shapeContents {
            Self::render_shapes(shape_contents, &mut layer_data, layer_width, layer_height)?;
        }

        if let Some(effects) = &layer.effects {
            Self::apply_effects(effects, &mut layer_data, layer_width, layer_height);
        }

        Self::composite_layer(
            frame_data,
            &layer_data,
            transform,
            opacity,
            width,
            height,
            layer_width,
            layer_height,
        );

        Ok(())
    }

    fn render_text(
        content: &TextContent,
        data: &mut [u8],
        width: u32,
        height: u32,
    ) -> Result<()> {
        let color = hex_to_rgba(&content.color);
        let font_size = content.fontSize as u32;

        let center_x = width / 2;
        let center_y = height / 2;

        let chars = content.text.chars().collect::<Vec<_>>();
        let total_width = chars.len() as u32 * font_size / 2;
        let start_x = center_x.saturating_sub(total_width / 2);

        for (i, _c) in chars.iter().enumerate() {
            let x = start_x + (i as u32 * font_size / 2);
            let y = center_y.saturating_sub(font_size / 2);

            for dy in 0..font_size {
                for dx in 0..(font_size / 2) {
                    let px = x + dx;
                    let py = y + dy;
                    if px < width && py < height {
                        let idx = ((py * width + px) * 4) as usize;
                        if idx + 3 < data.len() {
                            data[idx] = color.0;
                            data[idx + 1] = color.1;
                            data[idx + 2] = color.2;
                            data[idx + 3] = color.3;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn render_shapes(
        shapes: &[ShapeContent],
        data: &mut [u8],
        width: u32,
        height: u32,
    ) -> Result<()> {
        for shape in shapes {
            match shape.shape_type {
                ShapeType::Rect => {
                    let size = shape.size.as_ref().map(|s| (s[0] as u32, s[1] as u32)).unwrap_or((100, 100));
                    let pos = shape.position.as_ref().map(|p| (p[0] as i32, p[1] as i32)).unwrap_or((0, 0));
                    let roundness = shape.roundness.unwrap_or(0.0) as u32;

                    let color = shape.color.as_ref().map(|c| hex_to_rgba(c)).unwrap_or((255, 255, 255, 255));

                    for y in 0..size.1 {
                        for x in 0..size.0 {
                            let mut in_shape = true;
                            if roundness > 0 {
                                let r = roundness as i32;
                                if x < roundness && y < roundness {
                                    let dx = r as i32 - x as i32 - 1;
                                    let dy = r as i32 - y as i32 - 1;
                                    if dx * dx + dy * dy > r * r {
                                        in_shape = false;
                                    }
                                }
                                if x >= size.0 - roundness && y < roundness {
                                    let dx = x as i32 - (size.0 - roundness) as i32;
                                    let dy = r as i32 - y as i32 - 1;
                                    if dx * dx + dy * dy > r * r {
                                        in_shape = false;
                                    }
                                }
                                if x < roundness && y >= size.1 - roundness {
                                    let dx = r as i32 - x as i32 - 1;
                                    let dy = y as i32 - (size.1 - roundness) as i32;
                                    if dx * dx + dy * dy > r * r {
                                        in_shape = false;
                                    }
                                }
                                if x >= size.0 - roundness && y >= size.1 - roundness {
                                    let dx = x as i32 - (size.0 - roundness) as i32;
                                    let dy = y as i32 - (size.1 - roundness) as i32;
                                    if dx * dx + dy * dy > r * r {
                                        in_shape = false;
                                    }
                                }
                            }

                            if in_shape {
                                let px = pos.0 + x as i32;
                                let py = pos.1 + y as i32;
                                if px >= 0 && (px as u32) < width && py >= 0 && (py as u32) < height {
                                    let idx = ((py as u32 * width + px as u32) * 4) as usize;
                                    if idx + 3 < data.len() {
                                        data[idx] = color.0;
                                        data[idx + 1] = color.1;
                                        data[idx + 2] = color.2;
                                        data[idx + 3] = color.3;
                                    }
                                }
                            }
                        }
                    }
                }
                ShapeType::Ellipse => {
                    let size = shape.size.as_ref().map(|s| (s[0] as f64, s[1] as f64)).unwrap_or((100.0, 100.0));
                    let pos = shape.position.as_ref().map(|p| (p[0] as f64, p[1] as f64)).unwrap_or((0.0, 0.0));
                    let color = shape.color.as_ref().map(|c| hex_to_rgba(c)).unwrap_or((255, 255, 255, 255));

                    let cx = size.0 / 2.0;
                    let cy = size.1 / 2.0;
                    let rx = cx;
                    let ry = cy;

                    for y in 0..size.1 as u32 {
                        for x in 0..size.0 as u32 {
                            let dx = x as f64 - rx;
                            let dy = y as f64 - ry;
                            // Ellipse equation: (x/a)^2 + (y/b)^2 <= 1
                            let normalized = (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry);
                            if normalized <= 1.0 {
                                let px = pos.0 as i32 + x as i32;
                                let py = pos.1 as i32 + y as i32;
                                if px >= 0 && (px as u32) < width && py >= 0 && (py as u32) < height {
                                    let idx = ((py as u32 * width + px as u32) * 4) as usize;
                                    if idx + 3 < data.len() {
                                        data[idx] = color.0;
                                        data[idx + 1] = color.1;
                                        data[idx + 2] = color.2;
                                        data[idx + 3] = color.3;
                                    }
                                }
                            }
                        }
                    }
                }
                ShapeType::Star => {
                    let size = shape.size.as_ref().map(|s| (s[0] as f64, s[1] as f64)).unwrap_or((100.0, 100.0));
                    let pos = shape.position.as_ref().map(|p| (p[0] as f64, p[1] as f64)).unwrap_or((0.0, 0.0));
                    let color = shape.color.as_ref().map(|c| hex_to_rgba(c)).unwrap_or((255, 255, 255, 255));
                    let points = shape.copies.unwrap_or(5) as i32;

                    let cx = size.0 / 2.0;
                    let cy = size.1 / 2.0;
                    let outer_rx = cx;
                    let outer_ry = cy;
                    let inner_rx = cx * 0.4;
                    let inner_ry = cy * 0.4;

                    let mut star_pixels: Vec<(i32, i32)> = Vec::new();
                    for i in 0..points {
                        let angle1 = (i as f64 * 2.0 * std::f64::consts::PI / points as f64) - std::f64::consts::FRAC_PI_2;
                        let angle2 = ((i as f64 + 0.5) * 2.0 * std::f64::consts::PI / points as f64) - std::f64::consts::FRAC_PI_2;

                        let outer_x1 = cx + outer_rx * angle1.cos();
                        let outer_y1 = cy + outer_ry * angle1.sin();
                        let inner_x1 = cx + inner_rx * angle2.cos();
                        let inner_y1 = cy + inner_ry * angle2.sin();

                        let min_x = 0.0_f64.max(outer_x1.min(inner_x1) - 1.0);
                        let max_x = size.0.min(outer_x1.max(inner_x1) + 1.0);
                        let min_y = 0.0_f64.max(outer_y1.min(inner_y1) - 1.0);
                        let max_y = size.1.min(outer_y1.max(inner_y1) + 1.0);

                        for py in min_y as u32..max_y as u32 {
                            for px in min_x as u32..max_x as u32 {
                                let px_f = px as f64 + 0.5;
                                let py_f = py as f64 + 0.5;
                                let v0x = inner_x1 - outer_x1;
                                let v0y = inner_y1 - outer_y1;
                                let v1x = cx - outer_x1;
                                let v1y = cy - outer_y1;
                                let v2x = px_f - outer_x1;
                                let v2y = py_f - outer_y1;

                                let dot00 = v0x * v0x + v0y * v0y;
                                let dot01 = v0x * v1x + v0y * v1y;
                                let dot02 = v0x * v2x + v0y * v2y;
                                let dot11 = v1x * v1x + v1y * v1y;
                                let dot12 = v1x * v2x + v1y * v2y;

                                let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
                                let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
                                let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

                                if u >= 0.0 && v >= 0.0 && (u + v) <= 1.0 {
                                    star_pixels.push((px as i32, py as i32));
                                }
                            }
                        }
                    }

                    star_pixels.sort();
                    star_pixels.dedup();

                    for (px, py) in star_pixels {
                        let final_x = pos.0 as i32 + px;
                        let final_y = pos.1 as i32 + py;
                        if final_x >= 0 && (final_x as u32) < width && final_y >= 0 && (final_y as u32) < height {
                            let idx = ((final_y as u32 * width + final_x as u32) * 4) as usize;
                            if idx + 3 < data.len() {
                                data[idx] = color.0;
                                data[idx + 1] = color.1;
                                data[idx + 2] = color.2;
                                data[idx + 3] = color.3;
                            }
                        }
                    }
                }
                ShapeType::Polygon => {
                    let size = shape.size.as_ref().map(|s| (s[0] as f64, s[1] as f64)).unwrap_or((100.0, 100.0));
                    let pos = shape.position.as_ref().map(|p| (p[0] as f64, p[1] as f64)).unwrap_or((0.0, 0.0));
                    let color = shape.color.as_ref().map(|c| hex_to_rgba(c)).unwrap_or((255, 255, 255, 255));
                    let sides = shape.copies.unwrap_or(6) as i32;

                    let cx = size.0 / 2.0;
                    let cy = size.1 / 2.0;
                    let r = cx.min(cy) * 0.9;

                    let mut poly_pixels: Vec<(i32, i32)> = Vec::new();

                    for i in 0..sides {
                        let angle1 = (i as f64 * 2.0 * std::f64::consts::PI / sides as f64) - std::f64::consts::FRAC_PI_2;
                        let angle2 = ((i as f64 + 1.0) * 2.0 * std::f64::consts::PI / sides as f64) - std::f64::consts::FRAC_PI_2;

                        let x1 = cx + r * angle1.cos();
                        let y1 = cy + r * angle1.sin();
                        let x2 = cx + r * angle2.cos();
                        let y2 = cy + r * angle2.sin();

                        let min_x = 0.0_f64.max(x1.min(x2).min(cx) - 1.0);
                        let max_x = size.0.min(x1.max(x2).max(cx) + 1.0);
                        let min_y = 0.0_f64.max(y1.min(y2).min(cy) - 1.0);
                        let max_y = size.1.min(y1.max(y2).max(cy) + 1.0);

                        for py in min_y as u32..max_y as u32 {
                            for px in min_x as u32..max_x as u32 {
                                let px_f = px as f64 + 0.5;
                                let py_f = py as f64 + 0.5;
                                let v0x = x2 - x1;
                                let v0y = y2 - y1;
                                let v1x = cx - x1;
                                let v1y = cy - y1;
                                let v2x = px_f - x1;
                                let v2y = py_f - y1;

                                let dot00 = v0x * v0x + v0y * v0y;
                                let dot01 = v0x * v1x + v0y * v1y;
                                let dot02 = v0x * v2x + v0y * v2y;
                                let dot11 = v1x * v1x + v1y * v1y;
                                let dot12 = v1x * v2x + v1y * v2y;

                                let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
                                let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
                                let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

                                if u >= 0.0 && v >= 0.0 && (u + v) <= 1.0 {
                                    poly_pixels.push((px as i32, py as i32));
                                }
                            }
                        }
                    }

                    poly_pixels.sort();
                    poly_pixels.dedup();

                    for (px, py) in poly_pixels {
                        let final_x = pos.0 as i32 + px;
                        let final_y = pos.1 as i32 + py;
                        if final_x >= 0 && (final_x as u32) < width && final_y >= 0 && (final_y as u32) < height {
                            let idx = ((final_y as u32 * width + final_x as u32) * 4) as usize;
                            if idx + 3 < data.len() {
                                data[idx] = color.0;
                                data[idx + 1] = color.1;
                                data[idx + 2] = color.2;
                                data[idx + 3] = color.3;
                            }
                        }
                    }
                }
                ShapeType::Stroke => {
                    let size = shape.size.as_ref().map(|s| (s[0] as u32, s[1] as u32)).unwrap_or((100, 100));
                    let pos = shape.position.as_ref().map(|p| (p[0] as i32, p[1] as i32)).unwrap_or((0, 0));
                    let stroke_width = shape.width.unwrap_or(1.0) as i32;
                    let color = shape.color.as_ref().map(|c| hex_to_rgba(c)).unwrap_or((255, 255, 255, 255));

                    for y in 0..size.1 {
                        for x in 0..size.0 {
                            let is_border = x < stroke_width as u32
                                || x >= size.0 - stroke_width as u32
                                || y < stroke_width as u32
                                || y >= size.1 - stroke_width as u32;

                            if is_border {
                                let px = pos.0 + x as i32;
                                let py = pos.1 + y as i32;
                                if px >= 0 && (px as u32) < width && py >= 0 && (py as u32) < height {
                                    let idx = ((py as u32 * width + px as u32) * 4) as usize;
                                    if idx + 3 < data.len() {
                                        data[idx] = color.0;
                                        data[idx + 1] = color.1;
                                        data[idx + 2] = color.2;
                                        data[idx + 3] = color.3;
                                    }
                                }
                            }
                        }
                    }
                }
                ShapeType::Fill => {
                    let size = shape.size.as_ref().map(|s| (s[0] as u32, s[1] as u32)).unwrap_or((width, height));
                    let pos = shape.position.as_ref().map(|p| (p[0] as i32, p[1] as i32)).unwrap_or((0, 0));
                    let color = shape.color.as_ref().map(|c| hex_to_rgba(c)).unwrap_or((255, 255, 255, 255));

                    for y in 0..size.1.min(height) {
                        for x in 0..size.0.min(width) {
                            let px = pos.0 + x as i32;
                            let py = pos.1 + y as i32;
                            if px >= 0 && (px as u32) < width && py >= 0 && (py as u32) < height {
                                let idx = ((py as u32 * width + px as u32) * 4) as usize;
                                if idx + 3 < data.len() {
                                    data[idx] = color.0;
                                    data[idx + 1] = color.1;
                                    data[idx + 2] = color.2;
                                    data[idx + 3] = color.3;
                                }
                            }
                        }
                    }
                }
                ShapeType::Repeater => {
                    let copies = shape.copies.unwrap_or(1).max(1) as usize;
                    let offset = shape.offset.as_ref().map(|o| (o[0] as i32, o[1] as i32)).unwrap_or((50, 50));
                    let base_shape_size = shape.size.as_ref().map(|s| (s[0] as u32, s[1] as u32)).unwrap_or((100, 100));
                    let color = shape.color.as_ref().map(|c| hex_to_rgba(c)).unwrap_or((255, 255, 255, 255));

                    for copy_idx in 0..copies {
                        let copy_x = offset.0 * copy_idx as i32;
                        let copy_y = offset.1 * copy_idx as i32;

                        for y in 0..base_shape_size.1 {
                            for x in 0..base_shape_size.0 {
                                let px = copy_x + x as i32;
                                let py = copy_y + y as i32;
                                if px >= 0 && (px as u32) < width && py >= 0 && (py as u32) < height {
                                    let idx = ((py as u32 * width + px as u32) * 4) as usize;
                                    if idx + 3 < data.len() {
                                        data[idx] = color.0;
                                        data[idx + 1] = color.1;
                                        data[idx + 2] = color.2;
                                        data[idx + 3] = color.3;
                                    }
                                }
                            }
                        }
                    }
                }
                ShapeType::Group | ShapeType::Path => {
                }
            }
        }

        Ok(())
    }

    fn apply_effects(
        effects: &[Effect],
        data: &mut [u8],
        width: u32,
        height: u32,
    ) {
        for effect in effects {
            if !effect.enabled {
                continue;
            }

            match effect.effect_type {
                EffectType::Glow => {
                    let radius = get_f64_param(&effect.params, "radius", 20.0) as i32;
                    let color = get_color_param(&effect.params, "color", "#ffffff");
                    let opacity = get_f64_param(&effect.params, "opacity", 50.0) / 100.0;
                    let threshold = get_f64_param(&effect.params, "threshold", 0.0) / 100.0;

                    apply_glow(data, width, height, radius, color, opacity, threshold);
                }
                EffectType::DropShadow => {
                    let distance = get_f64_param(&effect.params, "distance", 10.0) as i32;
                    let angle = get_f64_param(&effect.params, "angle", 45.0);
                    let blur = get_f64_param(&effect.params, "blur", 5.0) as i32;
                    let color = get_color_param(&effect.params, "color", "#000000");
                    let opacity = get_f64_param(&effect.params, "opacity", 50.0) / 100.0;

                    apply_drop_shadow(data, width, height, distance, angle, blur, color, opacity);
                }
                EffectType::GaussianBlur => {
                    let radius = get_f64_param(&effect.params, "radius", 5.0) as i32;
                    let iterations = get_i32_param(&effect.params, "iterations", 1);

                    apply_gaussian_blur(data, width, height, radius, iterations);
                }
                EffectType::Vignette => {
                    let amount = get_f64_param(&effect.params, "amount", 30.0) / 100.0;
                    let size = get_f64_param(&effect.params, "size", 50.0) / 100.0;
                    let feather = get_f64_param(&effect.params, "feather", 50.0) / 100.0;
                    let color = get_color_param(&effect.params, "color", "#000000");

                    apply_vignette(data, width, height, amount, size, feather, color);
                }
                EffectType::BrightnessContrast => {
                    let brightness = get_f64_param(&effect.params, "brightness", 0.0) / 100.0;
                    let contrast = get_f64_param(&effect.params, "contrast", 0.0) / 100.0;

                    apply_brightness_contrast(data, width, height, brightness, contrast);
                }
                EffectType::HueSaturation => {
                    let hue = get_f64_param(&effect.params, "hue", 0.0);
                    let saturation = get_f64_param(&effect.params, "saturation", 0.0) / 100.0;

                    apply_hue_saturation(data, width, height, hue, saturation);
                }
                EffectType::ColorCorrection => {
                    let exposure = get_f64_param(&effect.params, "exposure", 0.0) / 100.0;
                    let gamma = get_f64_param(&effect.params, "gamma", 1.0);
                    let saturation = get_f64_param(&effect.params, "saturation", 0.0) / 100.0;

                    apply_color_correction(data, width, height, exposure, gamma, saturation);
                }
                EffectType::Levels => {
                    let black = get_f64_param(&effect.params, "black", 0.0) / 255.0;
                    let white = get_f64_param(&effect.params, "white", 255.0) / 255.0;
                    let gamma = get_f64_param(&effect.params, "gamma", 1.0);

                    apply_levels(data, width, height, black, white, gamma);
                }
                EffectType::Noise => {
                    let amount = get_f64_param(&effect.params, "amount", 10.0) / 100.0;
                    let seed = get_i32_param(&effect.params, "seed", 0);

                    apply_noise(data, width, height, amount, seed);
                }
                EffectType::Flip => {
                    let horizontal = get_bool_param(&effect.params, "horizontal", false);
                    let vertical = get_bool_param(&effect.params, "vertical", false);

                    apply_flip(data, width, height, horizontal, vertical);
                }
                _ => {}
            }
        }
    }

    fn composite_layer(
        base: &mut [u8],
        layer: &[u8],
        transform: &crate::animation::ComputedTransform,
        opacity: f64,
        base_w: u32,
        base_h: u32,
        layer_w: u32,
        layer_h: u32,
    ) {
        let offset_x = transform.position[0] as i32;
        let offset_y = transform.position[1] as i32;

        let scale_x = transform.scale[0] / 100.0;
        let scale_y = transform.scale[1] / 100.0;

        for y in 0..layer_h {
            for x in 0..layer_w {
                let src_idx = ((y * layer_w + x) * 4) as usize;
                if src_idx + 3 >= layer.len() {
                    continue;
                }

                let dst_x = offset_x + (x as f64 * scale_x) as i32;
                let dst_y = offset_y + (y as f64 * scale_y) as i32;

                if dst_x < 0 || (dst_x as u32) >= base_w || dst_y < 0 || (dst_y as u32) >= base_h {
                    continue;
                }

                let dst_idx = ((dst_y as u32 * base_w + dst_x as u32) * 4) as usize;
                if dst_idx + 3 >= base.len() {
                    continue;
                }

                let src_a = (layer[src_idx + 3] as f64 / 255.0) * opacity;
                let dst_a = base[dst_idx + 3] as f64 / 255.0;
                let out_a = src_a + dst_a * (1.0 - src_a);

                if out_a > 0.0 {
                    for i in 0..3 {
                        let src_c = layer[src_idx + i] as f64;
                        let dst_c = base[dst_idx + i] as f64;
                        let blended = (src_c * src_a + dst_c * dst_a * (1.0 - src_a)) / out_a;
                        base[dst_idx + i] = blended as u8;
                    }
                    base[dst_idx + 3] = (out_a * 255.0) as u8;
                }
            }
        }
    }

    fn save_frame_png(path: &Path, data: &[u8], width: u32, height: u32) -> Result<()> {
        let img = image::RgbaImage::from_raw(width, height, data.to_vec())
            .ok_or_else(|| anyhow::anyhow!("Failed to create image"))?;
        img.save(path)?;
        Ok(())
    }

    fn encode_video(
        cache_dir: &Path,
        output_path: &Path,
        width: u32,
        height: u32,
        fps: f64,
        gpu: bool,
        preset: Option<String>,
        crf: Option<u8>,
    ) -> Result<()> {
        let input_pattern = cache_dir.join("%06d.png");
        let crf_val = crf.unwrap_or(23);
        let preset_val = preset.unwrap_or_else(|| "medium".to_string());

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y")
            .arg("-framerate")
            .arg(fps.to_string())
            .arg("-i")
            .arg(input_pattern.to_str().unwrap())
            .arg("-c:v");

        if gpu {
            cmd.arg("h264_nvenc");
        } else {
            cmd.arg("libx264");
        }

        cmd.arg("-preset")
            .arg(&preset_val)
            .arg("-crf")
            .arg(crf_val.to_string())
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg(output_path.to_str().unwrap());

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("FFmpeg failed: {}", stderr);
        }

        Ok(())
    }
}

fn hex_to_rgba(hex: &str) -> (u8, u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
        } else {
            255
        };
        (r, g, b, a)
    } else {
        (0, 0, 0, 255)
    }
}

fn get_f64_param(params: &serde_json::Value, name: &str, default: f64) -> f64 {
    params.get(name).and_then(|v| v.as_f64()).unwrap_or(default)
}

fn get_i32_param(params: &serde_json::Value, name: &str, default: i32) -> i32 {
    params.get(name).and_then(|v| v.as_i64()).unwrap_or(default as i64) as i32
}

fn get_bool_param(params: &serde_json::Value, name: &str, default: bool) -> bool {
    params.get(name).and_then(|v| v.as_bool()).unwrap_or(default)
}

fn get_color_param(params: &serde_json::Value, name: &str, default: &str) -> String {
    params.get(name).and_then(|v| v.as_str()).unwrap_or(default).to_string()
}

fn apply_glow(
    data: &mut [u8],
    width: u32,
    height: u32,
    radius: i32,
    color: String,
    opacity: f64,
    threshold: f64,
) {
    let (cr, cg, cb, _) = hex_to_rgba(&color);
    let mut blurred = data.to_vec();

    box_blur(&data, &mut blurred, width, height, radius);

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 >= data.len() {
                continue;
            }

            let alpha = data[idx + 3] as f64 / 255.0;
            if alpha < threshold {
                continue;
            }

            let blend = opacity * alpha;
            data[idx] = (data[idx] as f64 * (1.0 - blend) + cr as f64 * blend) as u8;
            data[idx + 1] = (data[idx + 1] as f64 * (1.0 - blend) + cg as f64 * blend) as u8;
            data[idx + 2] = (data[idx + 2] as f64 * (1.0 - blend) + cb as f64 * blend) as u8;
        }
    }
}

fn apply_drop_shadow(
    data: &mut [u8],
    width: u32,
    height: u32,
    distance: i32,
    angle: f64,
    blur: i32,
    color: String,
    opacity: f64,
) {
    let (cr, cg, cb, _) = hex_to_rgba(&color);
    let angle_rad = angle * std::f64::consts::PI / 180.0;
    let offset_x = (distance as f64 * angle_rad.cos()) as i32;
    let offset_y = (distance as f64 * angle_rad.sin()) as i32;

    let mut shadow = vec![0u8; data.len()];

    for y in 0..height {
        for x in 0..width {
            let src_idx = ((y * width + x) * 4) as usize;
            if src_idx + 3 >= data.len() {
                continue;
            }

            let alpha = data[src_idx + 3] as f64 / 255.0;
            if alpha > 0.0 {
                let dst_x = x as i32 + offset_x;
                let dst_y = y as i32 + offset_y;

                if dst_x >= 0 && (dst_x as u32) < width && dst_y >= 0 && (dst_y as u32) < height {
                    let dst_idx = ((dst_y as u32 * width + dst_x as u32) * 4) as usize;
                    if dst_idx + 3 < shadow.len() {
                        shadow[dst_idx] = cr;
                        shadow[dst_idx + 1] = cg;
                        shadow[dst_idx + 2] = cb;
                        shadow[dst_idx + 3] = (alpha * opacity * 255.0) as u8;
                    }
                }
            }
        }
    }

    if blur > 0 {
        let mut blurred = vec![0u8; shadow.len()];
        box_blur(&shadow, &mut blurred, width, height, blur);

        for i in (0..data.len()).step_by(4) {
            if i + 3 >= data.len() {
                break;
            }
            if blurred[i + 3] > 0 {
                let src_alpha = blurred[i + 3] as f64 / 255.0;
                let dst_alpha = data[i + 3] as f64 / 255.0;
                let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);
                if out_alpha > 0.0 {
                    data[i] = ((blurred[i] as f64 * src_alpha + data[i] as f64 * dst_alpha * (1.0 - src_alpha)) / out_alpha) as u8;
                    data[i + 1] = ((blurred[i + 1] as f64 * src_alpha + data[i + 1] as f64 * dst_alpha * (1.0 - src_alpha)) / out_alpha) as u8;
                    data[i + 2] = ((blurred[i + 2] as f64 * src_alpha + data[i + 2] as f64 * dst_alpha * (1.0 - src_alpha)) / out_alpha) as u8;
                    data[i + 3] = (out_alpha * 255.0) as u8;
                }
            }
        }
    } else {
        for y in 0..height {
            for x in 0..width {
                let src_idx = idx_of(&shadow, x, y, width);
                let dst_idx = idx_of(data, x, y, width);
                if shadow[src_idx + 3] > 0 && data[dst_idx + 3] == 0 {
                    data[dst_idx] = shadow[src_idx];
                    data[dst_idx + 1] = shadow[src_idx + 1];
                    data[dst_idx + 2] = shadow[src_idx + 2];
                    data[dst_idx + 3] = shadow[src_idx + 3];
                }
            }
        }
    }
}

fn idx_of(data: &[u8], x: u32, y: u32, width: u32) -> usize {
    ((y * width + x) * 4) as usize
}

fn box_blur(input: &[u8], output: &mut [u8], width: u32, height: u32, radius: i32) {
    let r = radius.min(50);

    for y in 0..height {
        for x in 0..width {
            let mut sum_r: f64 = 0.0;
            let mut sum_g: f64 = 0.0;
            let mut sum_b: f64 = 0.0;
            let mut sum_a: f64 = 0.0;
            let mut count: f64 = 0.0;

            for dy in -r..=r {
                for dx in -r..=r {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && (nx as u32) < width && ny >= 0 && (ny as u32) < height {
                        let idx = ((ny as u32 * width + nx as u32) * 4) as usize;
                        if idx + 3 < input.len() {
                            sum_r += input[idx] as f64;
                            sum_g += input[idx + 1] as f64;
                            sum_b += input[idx + 2] as f64;
                            sum_a += input[idx + 3] as f64;
                            count += 1.0;
                        }
                    }
                }
            }

            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 < output.len() && count > 0.0 {
                output[idx] = (sum_r / count) as u8;
                output[idx + 1] = (sum_g / count) as u8;
                output[idx + 2] = (sum_b / count) as u8;
                output[idx + 3] = (sum_a / count) as u8;
            }
        }
    }
}

fn apply_gaussian_blur(data: &mut [u8], width: u32, height: u32, radius: i32, iterations: i32) {
    let r = (radius as f64 * 0.3).round() as i32;
    if r < 1 {
        return;
    }

    let mut temp = vec![0u8; data.len()];

    for _ in 0..iterations {
        box_blur(data, &mut temp, width, height, r);
        box_blur(&temp, data, width, height, r);
    }
}

fn apply_vignette(
    data: &mut [u8],
    width: u32,
    height: u32,
    amount: f64,
    size: f64,
    feather: f64,
    color: String,
) {
    let (cr, cg, cb, _) = hex_to_rgba(&color);
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;
    let max_dist = ((cx * cx + cy * cy).sqrt() * size).max(1.0);
    let feather_width = max_dist * feather;

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 >= data.len() {
                continue;
            }

            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            let vignette;
            if dist < max_dist - feather_width {
                vignette = 0.0;
            } else if dist > max_dist {
                vignette = amount;
            } else {
                let t = (dist - (max_dist - feather_width)) / feather_width;
                vignette = amount * t;
            }

            if vignette > 0.0 {
                data[idx] = (data[idx] as f64 * (1.0 - vignette) + cr as f64 * vignette) as u8;
                data[idx + 1] = (data[idx + 1] as f64 * (1.0 - vignette) + cg as f64 * vignette) as u8;
                data[idx + 2] = (data[idx + 2] as f64 * (1.0 - vignette) + cb as f64 * vignette) as u8;
            }
        }
    }
}

fn apply_brightness_contrast(
    data: &mut [u8],
    width: u32,
    height: u32,
    brightness: f64,
    contrast: f64,
) {
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 >= data.len() {
                continue;
            }

            for i in 0..3 {
                let v = data[idx + i] as f64 / 255.0;
                let v2 = (v - 0.5) * (1.0 + contrast) + 0.5 + brightness;
                data[idx + i] = (v2.clamp(0.0, 1.0) * 255.0) as u8;
            }
        }
    }
}

fn apply_hue_saturation(
    data: &mut [u8],
    width: u32,
    height: u32,
    hue_shift: f64,
    saturation: f64,
) {
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 >= data.len() {
                continue;
            }

            let r = data[idx] as f64 / 255.0;
            let g = data[idx + 1] as f64 / 255.0;
            let b = data[idx + 2] as f64 / 255.0;

            let (h, s, v) = rgb_to_hsv(r, g, b);
            let h2 = (h + hue_shift / 360.0) % 1.0;
            let s2 = (s * (1.0 + saturation)).clamp(0.0, 1.0);

            let (r2, g2, b2) = hsv_to_rgb(h2, s2, v);
            data[idx] = (r2 * 255.0) as u8;
            data[idx + 1] = (g2 * 255.0) as u8;
            data[idx + 2] = (b2 * 255.0) as u8;
        }
    }
}

fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        ((g - b) / delta) % 6.0 / 6.0
    } else if max == g {
        ((b - r) / delta + 2.0) / 6.0
    } else {
        ((r - g) / delta + 4.0) / 6.0
    };

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h.abs(), s, v)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r + m, g + m, b + m)
}

fn apply_color_correction(
    data: &mut [u8],
    width: u32,
    height: u32,
    exposure: f64,
    gamma: f64,
    saturation: f64,
) {
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 >= data.len() {
                continue;
            }

            for i in 0..3 {
                let v = data[idx + i] as f64 / 255.0;
                let v2 = v * (1.0 + exposure);
                let v3 = if v2 > 0.0 { v2.powf(1.0 / gamma) } else { 0.0 };
                data[idx + i] = (v3.clamp(0.0, 1.0) * 255.0) as u8;
            }

            let r = data[idx] as f64 / 255.0;
            let g = data[idx + 1] as f64 / 255.0;
            let b = data[idx + 2] as f64 / 255.0;
            let (_, s, v) = rgb_to_hsv(r, g, b);
            let s2 = (s * (1.0 + saturation)).clamp(0.0, 1.0);
            let (r2, g2, b2) = hsv_to_rgb(0.0, s2, v);
            data[idx] = (r2 * 255.0) as u8;
            data[idx + 1] = (g2 * 255.0) as u8;
            data[idx + 2] = (b2 * 255.0) as u8;
        }
    }
}

fn apply_levels(
    data: &mut [u8],
    width: u32,
    height: u32,
    black: f64,
    white: f64,
    gamma: f64,
) {
    let scale = if white > black { 1.0 / (white - black) } else { 1.0 };

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 >= data.len() {
                continue;
            }

            for i in 0..3 {
                let v = data[idx + i] as f64 / 255.0;
                let v2 = ((v - black) * scale).max(0.0);
                let v3 = if v2 > 0.0 { v2.powf(1.0 / gamma) } else { 0.0 };
                data[idx + i] = (v3.clamp(0.0, 1.0) * 255.0) as u8;
            }
        }
    }
}

fn apply_noise(data: &mut [u8], width: u32, height: u32, amount: f64, seed: i32) {
    let mut rng = SimpleRng::new(seed as u64);

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if idx + 3 >= data.len() {
                continue;
            }

            let noise = (rng.next_f64() - 0.5) * 2.0 * amount * 255.0;
            for i in 0..3 {
                let v = data[idx + i] as f64 + noise;
                data[idx + i] = (v.clamp(0.0, 255.0)) as u8;
            }
        }
    }
}

fn apply_flip(data: &mut [u8], width: u32, height: u32, horizontal: bool, vertical: bool) {
    if horizontal {
        let mut flipped = data.to_vec();
        for y in 0..height {
            for x in 0..width {
                let src_x = width - 1 - x;
                let src_idx = ((y * width + src_x) * 4) as usize;
                let dst_idx = ((y * width + x) * 4) as usize;
                if src_idx + 3 < data.len() && dst_idx + 3 < data.len() {
                    flipped[dst_idx..dst_idx + 4].copy_from_slice(&data[src_idx..src_idx + 4]);
                }
            }
        }
        data.copy_from_slice(&flipped);
    }

    if vertical {
        let mut flipped = data.to_vec();
        for y in 0..height {
            let src_y = height - 1 - y;
            for x in 0..width {
                let src_idx = ((src_y * width + x) * 4) as usize;
                let dst_idx = ((y * width + x) * 4) as usize;
                if src_idx + 3 < data.len() && dst_idx + 3 < data.len() {
                    flipped[dst_idx..dst_idx + 4].copy_from_slice(&data[src_idx..src_idx + 4]);
                }
            }
        }
        data.copy_from_slice(&flipped);
    }
}

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(1234567890) }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (u64::MAX >> 11) as f64
    }
}

pub async fn render_project(
    project: &PathBuf,
    output: &PathBuf,
    gpu: bool,
    preset: Option<String>,
    crf: Option<u8>,
) -> Result<()> {
    RenderEngine::render(project, output, gpu, preset, crf).await
}

pub async fn preview_project(
    project: &PathBuf,
    time: f64,
    duration: Option<f64>,
) -> Result<()> {
    let proj = parser::parse_project(project)?;
    let composer = Composer::new(&proj);

    println!("Preview at time: {}s", time);
    println!("Project: {} ({}x{} @ {}fps)", proj.metadata.name, proj.settings.width, proj.settings.height, proj.settings.fps);

    let comp = proj.compositions.get(&proj.mainCompositionId).unwrap();
    let active_layers: Vec<_> = comp.layers.iter().filter(|l| Animator::is_layer_active(l, time)).collect();

    println!("Active layers: {}", active_layers.len());
    for layer in active_layers {
        let transform = Animator::evaluate_transform(&layer.transform, time);
        let opacity = Animator::evaluate_f64(&layer.opacity, time);
        println!("  - {} ({:?}) at ({:.1}, {:.1}) scale {:.1}% opacity {:.0}%",
            layer.name.as_deref().unwrap_or(&layer.id),
            layer.layer_type,
            transform.position[0],
            transform.position[1],
            transform.scale[0],
            opacity
        );
    }

    Ok(())
}
