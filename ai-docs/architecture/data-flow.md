# Data Flow

This document describes how data flows through the OutoCut system.

## Data Flow Diagram

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  .outocut   │────►│   Parser     │────►│   Project   │
│    File     │     │  (JSON +     │     │   Struct    │
│  (UTF-8)    │     │  Comments)   │     │             │
└─────────────┘     └──────────────┘     └──────┬──────┘
                                                │
                                                ▼
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   FFmpeg    │◄────│   Render    │◄────│  Composer   │
│  (Output)   │     │   Engine     │     │             │
│             │     │              │     └──────┬──────┘
└─────────────┘     └──────┬───────┘            │
                           │                     ▼
                           ▼              ┌─────────────┐
                    ┌──────────────┐      │   Animator  │
                    │ Frame Buffer │◄─────┤             │
                    │   (RGBA)     │      │  Keyframe   │
                    └──────────────┘      │  Evaluation │
                                            └─────────────┘
```

## Pipeline Stages

### 1. File Input

```
.outocut File (JSON with comments)
        │
        ▼
┌─────────────────────────────┐
│  parser::parse_project()    │
│  ├─ Read file as UTF-8     │
│  ├─ strip_comments()       │
│  └─ serde_json::from_str() │
└─────────────────────────────┘
        │
        ▼
   Project struct
```

**Comment Stripping Algorithm**:
1. Iterate through characters
2. Track string context (inside quotes)
3. Track escape sequences
4. On `/`:
   - If next char is `/`, skip to newline
   - If next char is `*`, skip to `*/`
5. Preserve all other characters

### 2. Project Validation

```
   Project struct
        │
        ▼
┌─────────────────────────────┐
│  parser::validate_project() │
│  ├─ Check dimensions > 0   │
│  ├─ Check fps > 0          │
│  ├─ Check duration > 0     │
│  └─ Check main composition │
│      exists & matches      │
└─────────────────────────────┘
        │
        ▼
   Validated Project
```

### 3. Composition Building

```
 Validated Project
        │
        ▼
┌─────────────────────────────┐
│  Composer::new()           │
│  ├─ Store compositions     │
│  ├─ Store assets           │
│  └─ Build layer index      │
└─────────────────────────────┘
        │
        ▼
   Composer Instance
```

**Layer Tree Construction**:
- All layers at root level initially
- Parent references (`parentId`) link layers
- Layers with same parent form child group
- Evaluation order: children inherit parent transforms

### 4. Animation Evaluation

```
 Composer + Time
        │
        ▼
┌─────────────────────────────┐
│  Animator::evaluate_*()     │
│  For each animated property: │
│  ├─ Sort keyframes by time │
│  ├─ Find surrounding keys  │
│  ├─ Calculate progress     │
│  ├─ Apply easing           │
│  └─ Interpolate value      │
└─────────────────────────────┘
        │
        ▼
   ComputedTransform
   + Opacity value
```

**Keyframe Lookup**:
```
Time: t
Keyframes: [k0, k1, k2, k3] (sorted)

if t <= k0.time: return k0.value
if t >= k3.time: return k3.value
if k_i.time <= t <= k_{i+1}.time:
    progress = (t - k_i.time) / (k_{i+1}.time - k_i.time)
    eased = apply_easing(progress, k_i.easing)
    return interpolate(k_i.value, k_{i+1}.value, eased)
```

### 5. Frame Rendering

```
 Frame Number: n
 Time: t = n / fps
        │
        ▼
┌─────────────────────────────┐
│  RenderEngine::render_frame │
│  1. Create buffer (width*    │
│     height*4 RGBA)           │
│  2. Fill with background    │
│  3. For each layer (bottom   │
│     to top):                 │
│     ├─ Check active(t)      │
│     ├─ Render content       │
│     ├─ Apply transform      │
│     ├─ Apply opacity        │
│     └─ Composite            │
│  4. Save as PNG             │
└─────────────────────────────┘
        │
        ▼
   Frame PNG File
```

**Layer Compositing**:
```
For each pixel in layer:
    src_alpha = layer_pixel.a * layer_opacity
    dst_alpha = base_pixel.a
    out_alpha = src_alpha + dst_alpha * (1 - src_alpha)
    
    For each RGB channel:
        out_color = (src * src_alpha + dst * dst_alpha * (1 - src_alpha)) / out_alpha
```

### 6. Video Encoding

```
 Frame PNG Sequence
 (.outocut.cache/*.png)
        │
        ▼
┌─────────────────────────────┐
│  FFmpeg Command            │
│  ffmpeg -framerate FPS     │
│    -i %06d.png             │
│    -c:v libx264           │
│    -preset PRESET          │
│    -crf CRF                │
│    output.mp4              │
└─────────────────────────────┘
        │
        ▼
   Final Video File
```

## State Management

### Transient State (Per-Frame)
- Computed transform values
- Current opacity
- Active layer list
- Rendered frame buffer

### Persistent State (Across Frames)
- Project structure (loaded once)
- Composer instance
- Cache directory

### Output State
- PNG sequence (temporary)
- Final video file

## Error Propagation

```
Error in Stage N
        │
        ▼
┌─────────────────────────────┐
│   ?Result<T>                │
│   ├─ Ok(value) ──► Stage N+1│
│  Err(e) ───► Early Return  │
└─────────────────────────────┘
        │
        ▼
   CLI Error Display
```

All functions return `Result<T>` using `anyhow` for easy error propagation.
