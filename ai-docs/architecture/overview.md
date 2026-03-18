# Architecture Overview

This document describes the high-level architecture of OutoCut.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLI (clap)                                │
│  outocut render | preview | validate | export-json | watch      │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Parser Module                             │
│  - JSON parsing with comment support                             │
│  - Project validation                                            │
│  - JSON export (pretty/minified)                                 │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                       Models Module                              │
│  - Project, Composition, Layer                                   │
│  - Transform, Keyframe, Effect                                   │
│  - Asset, ExportPreset                                           │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Composition Module                            │
│  - Layer tree management                                        │
│  - Parenting resolution                                          │
│  - Blend mode evaluation                                         │
│  - Track matte processing                                        │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Animation Module                             │
│  - Keyframe evaluation                                           │
│  - Easing function application                                    │
│  - Transform computation                                          │
│  - Layer visibility checking                                       │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Render Module                                │
│  - Frame-by-frame rendering                                      │
│  - Layer compositing                                             │
│  - PNG sequence generation                                       │
│  - FFmpeg video encoding                                         │
└─────────────────────────────────────────────────────────────────┘
```

## Module Dependencies

```
main.rs (entry point)
    │
    ├── cli.rs (command-line interface)
    │       └── clap (CLI parsing)
    │
    ├── parser.rs (JSON processing)
    │       ├── serde_json (JSON parsing)
    │       └── std::fs (file I/O)
    │
    ├── models.rs (data structures)
    │       ├── serde (serialization)
    │       ├── chrono (timestamps)
    │       └── uuid (ID generation)
    │
    ├── composition.rs (layer composition)
    │       └── models.rs
    │
    ├── animation.rs (keyframe system)
    │       └── models.rs
    │
    └── render.rs (rendering engine)
            ├── parser.rs
            ├── models.rs
            ├── composition.rs
            ├── animation.rs
            └── std::process::Command (FFmpeg)
```

## Key Design Decisions

### 1. Deterministic Rendering

Every render produces identical output for the same input:
- No random number generation during render
- Frame-accurate timing based on project fps
- Consistent keyframe interpolation

### 2. Layer-Based Composition

All visual elements are layers with:
- Transform (position, scale, rotation, anchor)
- Opacity (with keyframe support)
- Blend modes
- Track matte
- Effects stack
- Masks

### 3. Expression System (Planned)

Future versions will support JavaScript-like expressions:
```javascript
position.x = time * 50
opacity = Math.sin(time) * 100
scale = [100 + Math.sin(time * 2) * 20, 100]
```

## Data Flow

1. **Parse**: Read `.outocut` JSON file
2. **Validate**: Check project structure and settings
3. **Compose**: Build layer tree, resolve parenting
4. **Animate**: Evaluate keyframes for current frame
5. **Render**: Composite layers to frame buffer
6. **Encode**: Use FFmpeg to create final video

## Extension Points

The architecture supports extensions at multiple levels:

### New Layer Types
Add to `LayerType` enum in models.rs:
```rust
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
    // Add new types here
}
```

### New Effects
Add to `EffectType` enum in models.rs and implement in render.rs

### New Blend Modes
Add to `BlendMode` enum in models.rs and implement in composition.rs

### New Easing Functions
Add to `Easing` enum in models.rs and implement in animation.rs
