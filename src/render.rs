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
                _ => {}
            }
        }

        Ok(())
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
