use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub version: String,
    pub metadata: ProjectMetadata,
    pub settings: ProjectSettings,
    pub assets: Vec<Asset>,
    pub compositions: HashMap<String, Composition>,
    #[serde(rename = "mainCompositionId")]
    pub mainCompositionId: String,
    #[serde(rename = "exportPresets")]
    pub exportPresets: Vec<ExportPreset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub created: DateTime<Utc>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub duration: f64,
    #[serde(rename = "backgroundColor")]
    pub backgroundColor: String,
    #[serde(rename = "pixelAspect")]
    pub pixelAspect: f64,
    #[serde(rename = "sampleRate")]
    pub sampleRate: u32,
    #[serde(rename = "audioChannels")]
    pub audioChannels: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    pub path: String,
    #[serde(rename = "trimStart")]
    pub trimStart: Option<f64>,
    #[serde(rename = "trimEnd")]
    pub trimEnd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Video,
    Audio,
    Image,
    Font,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    pub id: String,
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    #[serde(rename = "type")]
    pub layer_type: LayerType,
    pub name: Option<String>,
    pub enabled: bool,
    #[serde(rename = "startTime")]
    pub startTime: f64,
    pub duration: f64,
    #[serde(rename = "inPoint")]
    pub inPoint: Option<f64>,
    #[serde(rename = "outPoint")]
    pub outPoint: Option<f64>,
    #[serde(rename = "parentId")]
    pub parentId: Option<String>,
    #[serde(rename = "trackMatte")]
    pub trackMatte: Option<TrackMatte>,
    #[serde(rename = "blendMode")]
    pub blendMode: Option<BlendMode>,
    pub opacity: AnimatedProperty<f64>,
    pub transform: Transform,
    pub content: Option<LayerContent>,
    #[serde(rename = "shapeContents")]
    pub shapeContents: Option<Vec<ShapeContent>>,
    pub effects: Option<Vec<Effect>>,
    pub masks: Option<Vec<Mask>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerType {
    Video,
    Audio,
    Image,
    Text,
    Shape,
    Solid,
    Null,
    Adjustment,
    Composition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrackMatte {
    Alpha,
    #[serde(rename = "alphaInverted")]
    AlphaInverted,
    Luma,
    #[serde(rename = "lumaInverted")]
    LumaInverted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    Add,
    Subtract,
    Divide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub anchor: AnimatedProperty<Vec<f64>>,
    pub position: AnimatedProperty<Vec<f64>>,
    pub scale: AnimatedProperty<Vec<f64>>,
    pub rotation: AnimatedProperty<f64>,
    pub skew: AnimatedProperty<Vec<f64>>,
    #[serde(rename = "skewAxis")]
    pub skewAxis: AnimatedProperty<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedProperty<T> {
    pub value: T,
    pub keyframes: Option<Vec<Keyframe>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f64,
    pub value: serde_json::Value,
    pub easing: Option<Easing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    CubicBezier(f64, f64, f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayerContent {
    Text(TextContent),
    Video(VideoContent),
    Image(ImageContent),
    Audio(AudioContent),
    Solid(SolidContent),
    Composition(CompositionContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub text: String,
    #[serde(rename = "fontId")]
    pub fontId: Option<String>,
    #[serde(rename = "fontSize")]
    pub fontSize: f64,
    pub color: String,
    pub tracking: Option<f64>,
    pub leading: Option<f64>,
    pub alignment: Option<TextAlignment>,
    #[serde(rename = "baselineShift")]
    pub baselineShift: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoContent {
    #[serde(rename = "assetId")]
    pub assetId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    #[serde(rename = "assetId")]
    pub assetId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioContent {
    #[serde(rename = "assetId")]
    pub assetId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidContent {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionContent {
    #[serde(rename = "compositionId")]
    pub compositionId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeContent {
    #[serde(rename = "type")]
    pub shape_type: ShapeType,
    pub name: Option<String>,
    pub size: Option<Vec<f64>>,
    pub position: Option<Vec<f64>>,
    pub roundness: Option<f64>,
    pub color: Option<String>,
    pub width: Option<f64>,
    #[serde(rename = "copies")]
    pub copies: Option<i32>,
    pub offset: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShapeType {
    Rect,
    Ellipse,
    Star,
    Polygon,
    Path,
    Fill,
    Stroke,
    Repeater,
    Group,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: String,
    #[serde(rename = "type")]
    pub effect_type: EffectType,
    pub enabled: bool,
    pub params: serde_json::Value,
    pub keyframes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EffectType {
    DropShadow,
    InnerShadow,
    Glow,
    OuterGlow,
    GaussianBlur,
    DirectionalBlur,
    RadialBlur,
    Crop,
    Rotate,
    Flip,
    Mirror,
    ColorCorrection,
    BrightnessContrast,
    HueSaturation,
    Levels,
    Curves,
    ChromaKey,
    Noise,
    Vignette,
    GlowEffect,
    Stroke,
    FillGradient,
    TrimPath,
    Wiggle,
    TextAnimator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mask {
    pub name: String,
    pub mode: MaskMode,
    pub path: Vec<MaskPoint>,
    pub feather: f64,
    pub opacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MaskMode {
    Add,
    Subtract,
    Intersect,
    Lighten,
    Darken,
    Difference,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPoint {
    pub x: f64,
    pub y: f64,
    #[serde(rename = "handleIn")]
    pub handle_in: Option<Vec<f64>>,
    #[serde(rename = "handleOut")]
    pub handle_out: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPreset {
    pub name: String,
    pub codec: String,
    pub crf: Option<u8>,
    pub preset: Option<String>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            version: "1.0".to_string(),
            metadata: ProjectMetadata {
                name,
                created: Utc::now(),
                author: None,
                description: None,
                tags: vec![],
            },
            settings: ProjectSettings {
                width: 1920,
                height: 1080,
                fps: 30.0,
                duration: 10.0,
                backgroundColor: "#000000".to_string(),
                pixelAspect: 1.0,
                sampleRate: 48000,
                audioChannels: 2,
            },
            assets: vec![],
            compositions: HashMap::new(),
            mainCompositionId: "comp_main".to_string(),
            exportPresets: vec![ExportPreset {
                name: "youtube".to_string(),
                codec: "h264".to_string(),
                crf: Some(18),
                preset: Some("slow".to_string()),
            }],
        }
    }
}

impl Layer {
    pub fn new(layer_type: LayerType, duration: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            layer_type,
            name: None,
            enabled: true,
            startTime: 0.0,
            duration,
            inPoint: None,
            outPoint: None,
            parentId: None,
            trackMatte: None,
            blendMode: None,
            opacity: AnimatedProperty {
                value: 100.0,
                keyframes: None,
            },
            transform: Transform::default(),
            content: None,
            shapeContents: None,
            effects: None,
            masks: None,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            anchor: AnimatedProperty {
                value: vec![0.0, 0.0],
                keyframes: None,
            },
            position: AnimatedProperty {
                value: vec![0.0, 0.0],
                keyframes: None,
            },
            scale: AnimatedProperty {
                value: vec![100.0, 100.0],
                keyframes: None,
            },
            rotation: AnimatedProperty {
                value: 0.0,
                keyframes: None,
            },
            skew: AnimatedProperty {
                value: vec![0.0, 0.0],
                keyframes: None,
            },
            skewAxis: AnimatedProperty {
                value: 0.0,
                keyframes: None,
            },
        }
    }
}
