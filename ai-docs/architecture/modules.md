# Module Breakdown

Detailed description of each module in OutOcut.

## CLI Module (`src/cli.rs`)

**Purpose**: Command-line interface using clap

**Responsibilities**:
- Parse command-line arguments
- Route to appropriate handlers
- File watching with notify crate

**Commands**:
- `render` - Video rendering
- `preview` - Frame preview
- `validate` - Project validation
- `export-json` - JSON export
- `watch` - Auto-reload on file change

**Key Functions**:
```rust
pub struct Args {
    pub command: Commands,
}

pub enum Commands {
    Render { project, output, gpu, preset, crf },
    Preview { project, time, duration },
    Validate { project },
    ExportJson { project, pretty },
    Watch { project },
}
```

## Parser Module (`src/parser.rs`)

**Purpose**: JSON file parsing and validation

**Responsibilities**:
- Read and parse `.outocut` files
- Strip comments from JSON
- Validate project structure
- Export JSON (pretty/minified)

**Key Functions**:
```rust
pub fn parse_project(path: &Path) -> Result<Project>
pub fn validate_project(path: &Path) -> Result<Project>
pub fn export_json(path: &Path, pretty: bool) -> Result<String>
```

**Comment Support**:
- Line comments: `// comment`
- Block comments: `/* multi-line comment */`
- Preserves content inside strings

## Models Module (`src/models.rs`)

**Purpose**: All data structures for the project file format

**Core Structures**:

### Project
Root object containing all project data:
- version: File format version
- metadata: Name, author, tags
- settings: Resolution, fps, duration
- assets: External file references
- compositions: All compositions
- mainCompositionId: Entry point composition
- exportPresets: Export configurations

### Composition
A timeline containing layers:
- id: Unique identifier
- duration: Timeline length
- dimensions: Width and height
- layers: Ordered list of layers

### Layer
Visual element on the timeline:
- id: Unique identifier
- type: Layer type (video, audio, text, shape, etc.)
- name: Optional display name
- enabled: Visibility toggle
- timing: startTime, duration, inPoint, outPoint
- parentId: Parenting reference
- trackMatte: Alpha/luma matte
- blendMode: Compositing mode
- opacity: Animated property
- transform: Position, scale, rotation, etc.
- content: Type-specific data
- shapeContents: Vector shape definitions
- effects: Effect stack
- masks: Mask definitions

### AnimatedProperty<T>
Property with optional keyframes:
- value: Default/static value
- keyframes: Optional array of keyframes

### Keyframe
Single animation point:
- time: Position on timeline
- value: Value at this time
- easing: Transition to next keyframe

## Composition Module (`src/composition.rs`)

**Purpose**: Layer composition and compositing

**Responsibilities**:
- Layer tree management
- Parent-child relationships
- Blend mode evaluation
- Track matte processing
- Asset resolution

**Key Functions**:
```rust
pub struct Composer {
    compositions: HashMap<String, Composition>,
    assets: Vec<Asset>,
}

impl Composer {
    pub fn new(project: &Project) -> Self
    pub fn get_layer(&self, comp_id: &str, layer_id: &str) -> Option<&Layer>
    pub fn get_layer_tree(&self, comp_id: &str) -> Vec<LayerNode>
    pub fn resolve_asset(&self, asset_id: &str) -> Option<&Asset>
    pub fn evaluate_blend_mode(&self, base, blend, mode) -> [u8; 4]
    pub fn apply_track_matte(&self, layer, matte_layer, time) -> TrackMatteResult
}
```

## Animation Module (`src/animation.rs`)

**Purpose**: Keyframe evaluation and interpolation

**Responsibilities**:
- Evaluate animated properties at any time
- Apply easing functions
- Compute transform values
- Check layer visibility

**Easing Functions Supported**:
- Linear
- EaseIn, EaseOut, EaseInOut
- EaseInCubic, EaseOutCubic, EaseInOutCubic
- EaseInBack, EaseOutBack, EaseInOutBack
- EaseInElastic, EaseOutElastic, EaseInOutElastic
- EaseInBounce, EaseOutBounce, EaseInOutBounce
- CubicBezier(x1, y1, x2, y2)

**Key Functions**:
```rust
pub struct Animator;

impl Animator {
    pub fn evaluate_f64(property: &AnimatedProperty<f64>, time: f64) -> f64
    pub fn evaluate_vec_f64(property: &AnimatedProperty<Vec<f64>>, time: f64) -> Vec<f64>
    pub fn evaluate_transform(transform: &Transform, time: f64) -> ComputedTransform
    pub fn is_layer_active(layer: &Layer, time: f64) -> bool
}
```

## Render Module (`src/render.rs`)

**Purpose**: Frame rendering and video encoding

**Responsibilities**:
- Frame-by-frame rendering
- Layer compositing
- Text and shape rendering
- PNG sequence generation
- FFmpeg video encoding

**Key Functions**:
```rust
pub struct RenderEngine;

impl RenderEngine {
    pub async fn render(project_path, output_path, gpu, preset, crf) -> Result<()>
    fn render_frame(project, time, width, height, cache_dir, frame) -> Result<()>
    fn render_layer(layer, transform, opacity, time, width, height, frame_data, composer) -> Result<()>
    fn composite_layer(base, layer, transform, opacity, ...) -> ()
    fn encode_video(cache_dir, output_path, width, height, fps, gpu, preset, crf) -> Result<()>
}
```

**Render Pipeline**:
1. Create frame buffer with background color
2. For each layer (back to front):
   - Check if layer is active at this time
   - Render layer content to temporary buffer
   - Apply transform (position, scale, rotation)
   - Apply opacity
   - Composite onto frame buffer
3. Save frame as PNG to cache directory
4. Use FFmpeg to encode PNG sequence to video

## Module Interaction

```
User Command
    │
    ▼
cli::Args::parse()
    │
    ├──► parser::parse_project() ──► models::Project
    │                                      │
    │                                      ▼
    │                              composition::Composer::new()
    │                                      │
    │                                      ▼
    │                              animation::Animator::evaluate_*(time)
    │                                      │
    │                                      ▼
    │                              render::RenderEngine::render()
    │                                      │
    │                                      ▼
    │                              FFmpeg encoding
    │
    └──► Handle result / errors
```
