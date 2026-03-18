use crate::models::*;
use std::collections::HashMap;

pub struct Composer {
    compositions: HashMap<String, Composition>,
    assets: Vec<Asset>,
}

impl Composer {
    pub fn new(project: &Project) -> Self {
        Self {
            compositions: project.compositions.clone(),
            assets: project.assets.clone(),
        }
    }

    pub fn get_layer(&self, comp_id: &str, layer_id: &str) -> Option<&Layer> {
        self.compositions
            .get(comp_id)
            .and_then(|c| c.layers.iter().find(|l| l.id == layer_id))
    }

    pub fn get_layer_tree(&self, comp_id: &str) -> Vec<LayerNode> {
        let composition = match self.compositions.get(comp_id) {
            Some(c) => c,
            None => return vec![],
        };

        let nodes: Vec<LayerNode> = composition
            .layers
            .iter()
            .map(|layer| LayerNode {
                layer: layer.clone(),
                children: vec![],
            })
            .collect();

        nodes
    }

    pub fn resolve_asset(&self, asset_id: &str) -> Option<&Asset> {
        self.assets.iter().find(|a| a.id == asset_id)
    }

    pub fn evaluate_blend_mode(
        &self,
        base: &[u8; 4],
        blend: &[u8; 4],
        mode: &BlendMode,
    ) -> [u8; 4] {
        match mode {
            BlendMode::Normal => *blend,
            _ => self.apply_blend(base, blend, mode),
        }
    }

    fn apply_blend(&self, base: &[u8; 4], blend: &[u8; 4], mode: &BlendMode) -> [u8; 4] {
        let b = [
            base[0] as f64 / 255.0,
            base[1] as f64 / 255.0,
            base[2] as f64 / 255.0,
            base[3] as f64 / 255.0,
        ];
        let f = [
            blend[0] as f64 / 255.0,
            blend[1] as f64 / 255.0,
            blend[2] as f64 / 255.0,
            blend[3] as f64 / 255.0,
        ];

        let result: [f64; 4] = match mode {
            BlendMode::Multiply => [b[0] * f[0], b[1] * f[1], b[2] * f[2], f[3]],
            BlendMode::Screen => [
                1.0 - (1.0 - b[0]) * (1.0 - f[0]),
                1.0 - (1.0 - b[1]) * (1.0 - f[1]),
                1.0 - (1.0 - b[2]) * (1.0 - f[2]),
                f[3],
            ],
            BlendMode::Overlay => [
                if b[0] < 0.5 {
                    2.0 * b[0] * f[0]
                } else {
                    1.0 - 2.0 * (1.0 - b[0]) * (1.0 - f[0])
                },
                if b[1] < 0.5 {
                    2.0 * b[1] * f[1]
                } else {
                    1.0 - 2.0 * (1.0 - b[1]) * (1.0 - f[1])
                },
                if b[2] < 0.5 {
                    2.0 * b[2] * f[2]
                } else {
                    1.0 - 2.0 * (1.0 - b[2]) * (1.0 - f[2])
                },
                f[3],
            ],
            BlendMode::Add => [
                (b[0] + f[0]).min(1.0),
                (b[1] + f[1]).min(1.0),
                (b[2] + f[2]).min(1.0),
                f[3],
            ],
            BlendMode::Lighten => [b[0].max(f[0]), b[1].max(f[1]), b[2].max(f[2]), f[3]],
            BlendMode::Darken => [b[0].min(f[0]), b[1].min(f[1]), b[2].min(f[2]), f[3]],
            BlendMode::Difference => [
                (b[0] - f[0]).abs(),
                (b[1] - f[1]).abs(),
                (b[2] - f[2]).abs(),
                f[3],
            ],
            _ => [f[0], f[1], f[2], f[3]],
        };

        [
            (result[0] * 255.0) as u8,
            (result[1] * 255.0) as u8,
            (result[2] * 255.0) as u8,
            (result[3] * 255.0) as u8,
        ]
    }

    pub fn apply_track_matte(
        &self,
        layer: &Layer,
        matte_layer: &Layer,
        time: f64,
    ) -> TrackMatteResult {
        let track_mattee = match &layer.trackMatte {
            Some(m) => m,
            None => return TrackMatteResult::NoMatte,
        };

        let matte_opacity = crate::animation::Animator::evaluate_f64(&matte_layer.opacity, time);

        if matte_opacity < 1.0 {
            return TrackMatteResult::PartialAlpha(matte_opacity);
        }

        match track_mattee {
            TrackMatte::Alpha => TrackMatteResult::Alpha,
            TrackMatte::AlphaInverted => TrackMatteResult::AlphaInverted,
            TrackMatte::Luma => TrackMatteResult::Luma,
            TrackMatte::LumaInverted => TrackMatteResult::LumaInverted,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerNode {
    pub layer: Layer,
    pub children: Vec<LayerNode>,
}

#[derive(Debug, Clone)]
pub enum TrackMatteResult {
    NoMatte,
    Alpha,
    AlphaInverted,
    Luma,
    LumaInverted,
    PartialAlpha(f64),
}
