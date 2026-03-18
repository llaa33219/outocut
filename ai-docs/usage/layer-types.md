# OutOcut Layer Types

Author: BLOUplanet  
License: Apache 2.0

This guide documents all OutOcut layer types in practical detail.

It covers:
- Purpose and typical use cases
- JSON structure (required vs optional fields)
- Full JSON examples for each layer type
- Common pitfalls and cautions
- Performance considerations

---

## 1) Common Layer Model (applies to all layer types)

Every layer in `composition.layers` follows the same base structure. Type-specific data is stored in `content` and/or `shapeContents`.

```json
{
  "id": "layer_unique_id",
  "type": "text",
  "name": "Layer Name",
  "enabled": true,
  "startTime": 0.0,
  "duration": 5.0,
  "inPoint": 0.0,
  "outPoint": 5.0,
  "parentId": null,
  "trackMatte": null,
  "blendMode": null,
  "opacity": {
    "value": 100,
    "keyframes": null
  },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": { "value": [960, 540], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {},
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common fields

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | Yes | Unique layer identifier in the composition. |
| `type` | enum | Yes | One of: `video`, `audio`, `image`, `text`, `shape`, `solid`, `null`, `adjustment`, `composition`. |
| `name` | string or null | No | Human-readable label. |
| `enabled` | boolean | Yes | If false, layer is skipped. |
| `startTime` | number | Yes | Layer start time in composition timeline seconds. |
| `duration` | number | Yes | Layer span length in seconds. |
| `inPoint` | number or null | No | Trim-in relative to layer start. |
| `outPoint` | number or null | No | Trim-out relative to layer start. |
| `parentId` | string or null | No | Parent layer id for transform hierarchy/grouping. |
| `trackMatte` | enum or null | No | `alpha`, `alphaInverted`, `luma`, `lumaInverted`. |
| `blendMode` | enum or null | No | Blend mode for compositing. |
| `opacity` | object | Yes | `AnimatedProperty<f64>` (0-100 typical). |
| `transform` | object | Yes | Anchor, position, scale, rotation, skew, skewAxis; each animated. |
| `content` | object or null | Depends on type | Type-specific payload (`text`, `solid`, media asset refs, etc.). |
| `shapeContents` | array or null | Shape layers | Shape primitives/operators. |
| `effects` | array or null | No | Layer effect stack. |
| `masks` | array or null | No | Layer masks. |

### Active-time behavior (`startTime`, `duration`, `inPoint`, `outPoint`)

A layer is active when all conditions are true:
- `time >= startTime`
- `time < startTime + duration`
- `time >= startTime + (inPoint or 0)`
- `time < startTime + (outPoint or duration)`

Practical rule:
- `startTime` moves the whole layer block.
- `inPoint`/`outPoint` trim inside that block.

### Animation object pattern (`AnimatedProperty`)

All animatable properties share this shape:

```json
{
  "value": 100,
  "keyframes": [
    { "time": 0.0, "value": 0, "easing": "linear" },
    { "time": 1.0, "value": 100, "easing": "easeOutCubic" }
  ]
}
```

Notes:
- `keyframes` can be `null` for static values.
- Keyframes are sorted by `time` internally.
- `easing` is optional; if omitted, interpolation is linear.

### Parent/child relationships (`parentId`)

Use `parentId` to build hierarchies:
- Child layers inherit logical relationship to parent (grouping/rigging intent).
- Use a `null` layer as a clean controller/anchor for many children.
- Keep parent and children active over overlapping time ranges.

Practical parent rig pattern:
1. Add one `null` layer at center.
2. Set multiple layers `parentId` to that null layer id.
3. Animate parent transform once instead of each child.

### Layer ordering and z-index

In a composition, layers are processed in array order:
- Earlier entries render first (back).
- Later entries render on top (front).

So for `layers: [A, B, C]`:
- `A` is bottom-most
- `C` is top-most

When using track mattes or blend interactions, adjacency/order matters. Place related layers close together and verify visually.

### Common enums used by many types

`TextAlignment`:
- `left`
- `center`
- `right`
- `justify`

`ShapeType` values in `shapeContents`:
- `rect`
- `ellipse`
- `star`
- `polygon`
- `path`
- `fill`
- `stroke`
- `repeater`
- `group`

---

## 2) `video` layer

### Purpose and use case

Use `video` layers for footage clips (camera shots, B-roll, screen captures, pre-rendered animations). Typically used as:
- Primary visual source
- Background plate
- Overlay footage (with blending/masks)

### JSON structure

Type-specific payload is `content.assetId` referencing an `assets[]` item of type `video`.

Required:
- Common required fields
- `content.assetId`

Optional:
- All common optional fields (`name`, `inPoint`, `outPoint`, `parentId`, `trackMatte`, `blendMode`, `effects`, `masks`)

```json
{
  "type": "video",
  "content": {
    "assetId": "vid_intro"
  }
}
```

### Full example JSON

```json
{
  "id": "layer_video_intro",
  "type": "video",
  "name": "Intro Footage",
  "enabled": true,
  "startTime": 0.0,
  "duration": 8.0,
  "inPoint": 0.5,
  "outPoint": 7.5,
  "parentId": null,
  "trackMatte": null,
  "blendMode": "normal",
  "opacity": { "value": 100, "keyframes": null },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": { "value": [960, 540], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {
    "assetId": "vid_intro"
  },
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- `assetId` must reference an existing asset in top-level `assets`.
- Ensure the asset type is actually `video`, not `image`/`audio`.
- `duration` and `outPoint` should match intended clip length logic.
- Large footage with heavy effects can become very slow in software rendering.

### Performance considerations

- Prefer pre-trimmed source files for long footage.
- Use lower-resolution proxies for development iterations.
- Keep stacked effects minimal on full-resolution video layers.

---

## 3) `audio` layer

### Purpose and use case

Use `audio` layers for music, voice-over, SFX, and ambience. Common patterns:
- One main music bed
- Separate dialogue/VO tracks
- Spot SFX layers aligned to events

### JSON structure

Type-specific payload is `content.assetId` referencing an `assets[]` item of type `audio`.

Required:
- Common required fields
- `content.assetId`

Optional:
- Common optional fields

```json
{
  "type": "audio",
  "content": {
    "assetId": "aud_music_main"
  }
}
```

### Full example JSON

```json
{
  "id": "layer_audio_bgm",
  "type": "audio",
  "name": "Background Music",
  "enabled": true,
  "startTime": 0.0,
  "duration": 30.0,
  "inPoint": 0.0,
  "outPoint": 30.0,
  "parentId": null,
  "trackMatte": null,
  "blendMode": null,
  "opacity": { "value": 100, "keyframes": null },
  "transform": {
    "anchor": { "value": [0, 0], "keyframes": null },
    "position": { "value": [0, 0], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {
    "assetId": "aud_music_main"
  },
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- `audio` layers are timeline items; visual transform fields usually have no audible effect.
- Avoid mismatching audio sample rates/channels across source assets when possible.
- Keep audio layer timing explicit with `startTime`, `inPoint`, and `outPoint`.

### Performance considerations

- Long uncompressed WAV files can increase I/O pressure.
- Many concurrent audio layers raise decode/mix cost.
- Pre-mix stems externally if project grows very large.

---

## 4) `image` layer

### Purpose and use case

Use `image` for logos, photos, still overlays, and UI captures.

### JSON structure

Type-specific payload is `content.assetId` referencing an `assets[]` item of type `image`.

Required:
- Common required fields
- `content.assetId`

Optional:
- Common optional fields

```json
{
  "type": "image",
  "content": {
    "assetId": "img_logo"
  }
}
```

### Full example JSON

```json
{
  "id": "layer_image_logo",
  "type": "image",
  "name": "Brand Logo",
  "enabled": true,
  "startTime": 1.0,
  "duration": 9.0,
  "inPoint": 0.0,
  "outPoint": 9.0,
  "parentId": "ctrl_brand_group",
  "trackMatte": null,
  "blendMode": "normal",
  "opacity": {
    "value": 0,
    "keyframes": [
      { "time": 1.0, "value": 0, "easing": "linear" },
      { "time": 1.5, "value": 100, "easing": "easeOutCubic" }
    ]
  },
  "transform": {
    "anchor": { "value": [100, 100], "keyframes": null },
    "position": { "value": [1800, 980], "keyframes": null },
    "scale": { "value": [60, 60], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {
    "assetId": "img_logo"
  },
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- Check image dimensions and intended anchor/position math.
- Very large PNGs can use substantial memory.
- Transparent images over many layers increase compositing load.

### Performance considerations

- Resize oversized images before import.
- Prefer compressed formats where acceptable.
- Avoid stacking many full-resolution transparent overlays.

---

## 5) `text` layer

### Purpose and use case

Use `text` layers for titles, lower-thirds, captions, CTAs, labels, and kinetic typography.

### JSON structure

`content` must be a text payload.

Required text fields:
- `text` (string)
- `fontSize` (number)
- `color` (string, typically hex)

Optional text fields:
- `fontId` (string or null, references a font asset)
- `tracking` (number or null)
- `leading` (number or null)
- `alignment` (`left` | `center` | `right` | `justify`)
- `baselineShift` (number or null)

```json
{
  "type": "text",
  "content": {
    "text": "HELLO",
    "fontId": "font_inter_bold",
    "fontSize": 96,
    "color": "#ffffff",
    "tracking": 20,
    "leading": 110,
    "alignment": "center",
    "baselineShift": 0
  }
}
```

### Full example JSON

```json
{
  "id": "layer_text_title",
  "type": "text",
  "name": "Main Title",
  "enabled": true,
  "startTime": 0.0,
  "duration": 6.0,
  "inPoint": 0.0,
  "outPoint": 6.0,
  "parentId": "ctrl_title_group",
  "trackMatte": null,
  "blendMode": "normal",
  "opacity": {
    "value": 0,
    "keyframes": [
      { "time": 0.0, "value": 0, "easing": "linear" },
      { "time": 0.8, "value": 100, "easing": "easeOutCubic" },
      { "time": 5.2, "value": 100, "easing": "linear" },
      { "time": 6.0, "value": 0, "easing": "easeInCubic" }
    ]
  },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": {
      "value": [960, 540],
      "keyframes": [
        { "time": 0.0, "value": [960, 590], "easing": "easeOutBack" },
        { "time": 0.8, "value": [960, 540], "easing": "easeOutCubic" }
      ]
    },
    "scale": {
      "value": [100, 100],
      "keyframes": [
        { "time": 0.0, "value": [90, 90], "easing": "easeOutCubic" },
        { "time": 0.8, "value": [100, 100], "easing": "easeOutBack" }
      ]
    },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {
    "text": "OUTOCUT LAYER SYSTEM",
    "fontId": "font_heading_semibold",
    "fontSize": 84,
    "color": "#f7f7f7",
    "tracking": 18,
    "leading": 96,
    "alignment": "center",
    "baselineShift": 0
  },
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- Missing `fontSize` or `color` makes text content invalid.
- `fontId` should map to an existing font asset when used.
- Be consistent with `alignment` enum values (`left`, `center`, `right`, `justify`).
- Track readability: high tracking + small font sizes can become unreadable.

### Performance considerations

- Many animated text layers with many keyframes can become costly.
- Prefer fewer long-lived text layers over many short duplicated ones.
- For extremely complex typography, pre-render to video when appropriate.

---

## 6) `shape` layer

### Purpose and use case

Use `shape` layers for vector graphics, backgrounds, bars, badges, icons, motion design elements, and procedural geometry rigs.

### JSON structure

`shape` layers generally use `shapeContents` (array). `content` is typically `null`.

`shapeContents[]` item fields:
- `type` (required): `rect`, `ellipse`, `star`, `polygon`, `path`, `fill`, `stroke`, `repeater`, `group`
- Optional (depends on item): `name`, `size`, `position`, `roundness`, `color`, `width`, `copies`, `offset`

```json
{
  "type": "shape",
  "content": null,
  "shapeContents": [
    {
      "type": "rect",
      "name": "Card",
      "size": [640, 240],
      "position": [320, 120],
      "roundness": 24,
      "color": "#1f2937"
    },
    {
      "type": "stroke",
      "name": "CardStroke",
      "width": 4,
      "color": "#ffffff"
    }
  ]
}
```

### Full example JSON

```json
{
  "id": "layer_shape_panel",
  "type": "shape",
  "name": "Info Panel",
  "enabled": true,
  "startTime": 0.0,
  "duration": 10.0,
  "inPoint": 0.0,
  "outPoint": 10.0,
  "parentId": null,
  "trackMatte": null,
  "blendMode": "normal",
  "opacity": { "value": 90, "keyframes": null },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": { "value": [960, 540], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": null,
  "shapeContents": [
    {
      "type": "group",
      "name": "PanelGroup",
      "size": null,
      "position": null,
      "roundness": null,
      "color": null,
      "width": null,
      "copies": null,
      "offset": null
    },
    {
      "type": "rect",
      "name": "PanelBody",
      "size": [1400, 520],
      "position": [260, 280],
      "roundness": 28,
      "color": "#111827ee",
      "width": null,
      "copies": null,
      "offset": null
    },
    {
      "type": "fill",
      "name": "PanelFill",
      "size": null,
      "position": null,
      "roundness": null,
      "color": "#111827ee",
      "width": null,
      "copies": null,
      "offset": null
    },
    {
      "type": "stroke",
      "name": "PanelStroke",
      "size": null,
      "position": null,
      "roundness": null,
      "color": "#e5e7eb",
      "width": 3,
      "copies": null,
      "offset": null
    },
    {
      "type": "repeater",
      "name": "DecorRepeater",
      "size": null,
      "position": null,
      "roundness": null,
      "color": null,
      "width": null,
      "copies": 5,
      "offset": [16, 0]
    }
  ],
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- Do not confuse layer `type: "shape"` with shape item `shapeContents[].type`.
- `shapeContents` order can affect visual results in shape stacks.
- If using `stroke`, set a practical `width`.
- Keep geometry values in composition coordinate space for predictability.

### Performance considerations

- Many complex shape items across many layers increase rasterization cost.
- Repeater-heavy setups can multiply draw operations quickly.
- Pre-compose dense vector scenes when reused repeatedly.

---

## 7) `solid` layer

### Purpose and use case

Use `solid` for flat color backplates, wipes, overlays, and simple color mattes.

### JSON structure

Type-specific payload is `content.color`.

Required:
- Common required fields
- `content.color`

Optional:
- Common optional fields

```json
{
  "type": "solid",
  "content": {
    "color": "#0f172a"
  }
}
```

### Full example JSON

```json
{
  "id": "layer_solid_bg",
  "type": "solid",
  "name": "Background Solid",
  "enabled": true,
  "startTime": 0.0,
  "duration": 12.0,
  "inPoint": 0.0,
  "outPoint": 12.0,
  "parentId": null,
  "trackMatte": null,
  "blendMode": "normal",
  "opacity": { "value": 100, "keyframes": null },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": { "value": [960, 540], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {
    "color": "#0b1020"
  },
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- Solid layers cover full render target area; ordering is critical.
- Put base background solids near start of `layers` array (bottom).
- Use alpha in color (`#RRGGBBAA`) for overlays when needed.

### Performance considerations

- Solids are cheap to render compared to media-heavy layers.
- Many full-frame translucent solids still add compositing cost.

---

## 8) `null` layer

### Purpose and use case

Use `null` layers as non-visual controllers:
- Parent controller for grouped motion
- Shared anchor for coordinated transforms
- Organizational rigging tool for complex timelines

### JSON structure

`null` layers typically omit meaningful `content` and `shapeContents`.

Required:
- Common required fields

Optional:
- Common optional fields

```json
{
  "type": "null",
  "content": null,
  "shapeContents": null
}
```

### Full example JSON

```json
{
  "id": "ctrl_title_group",
  "type": "null",
  "name": "Title Controller",
  "enabled": true,
  "startTime": 0.0,
  "duration": 10.0,
  "inPoint": 0.0,
  "outPoint": 10.0,
  "parentId": null,
  "trackMatte": null,
  "blendMode": null,
  "opacity": { "value": 100, "keyframes": null },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": {
      "value": [960, 540],
      "keyframes": [
        { "time": 0.0, "value": [960, 600], "easing": "easeOutBack" },
        { "time": 1.0, "value": [960, 540], "easing": "easeOutCubic" }
      ]
    },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": null,
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- Do not expect visible pixels from a `null` layer.
- Broken `parentId` references silently break rigs.
- Keep ids stable; changing a parent id requires child updates.

### Performance considerations

- `null` layers are lightweight and excellent for managing complexity.
- Overusing deep parent chains can make troubleshooting harder.

---

## 9) `adjustment` layer

### Purpose and use case

Use `adjustment` for effect-only correction passes that conceptually affect layers below (grade, glow pass, blur pass, etc.).

Typical pattern:
- Place adjustment above target layers.
- Add effects in `effects` stack.
- Keep `content` empty/null.

### JSON structure

Required:
- Common required fields

Optional:
- `effects` is usually where the useful payload lives.

```json
{
  "type": "adjustment",
  "content": null,
  "effects": [
    {
      "id": "fx_cc_main",
      "type": "colorCorrection",
      "enabled": true,
      "params": { "saturation": 0.9, "contrast": 1.1 },
      "keyframes": null
    }
  ]
}
```

### Full example JSON

```json
{
  "id": "layer_adj_grade",
  "type": "adjustment",
  "name": "Global Grade",
  "enabled": true,
  "startTime": 0.0,
  "duration": 10.0,
  "inPoint": 0.0,
  "outPoint": 10.0,
  "parentId": null,
  "trackMatte": null,
  "blendMode": "normal",
  "opacity": { "value": 100, "keyframes": null },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": { "value": [960, 540], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": null,
  "shapeContents": null,
  "effects": [
    {
      "id": "fx_color",
      "type": "colorCorrection",
      "enabled": true,
      "params": {
        "brightness": 0.02,
        "contrast": 1.08,
        "saturation": 0.95
      },
      "keyframes": null
    },
    {
      "id": "fx_vignette",
      "type": "vignette",
      "enabled": true,
      "params": {
        "amount": 0.2,
        "roundness": 0.6
      },
      "keyframes": null
    }
  ],
  "masks": null
}
```

### Common pitfalls and cautions

- Adjustment behavior depends on render-engine support for effect routing.
- Put adjustment layers above the layers they should influence.
- If effect result looks wrong, verify z-order first before parameter tuning.

### Performance considerations

- Full-frame effects are expensive, especially blur/glow chains.
- Stack fewer heavy effects; combine where possible.
- Scope with masks when possible to reduce useful processing area.

---

## 10) `composition` layer

### Purpose and use case

Use `composition` layers for nested/pre-composed scenes:
- Reuse animated modules in multiple places.
- Keep main timeline clean.
- Build complex sequences from smaller compositions.

### JSON structure

Type-specific payload is `content.compositionId` referencing another key in top-level `compositions`.

Required:
- Common required fields
- `content.compositionId`

Optional:
- Common optional fields

```json
{
  "type": "composition",
  "content": {
    "compositionId": "title_comp"
  }
}
```

### Full example JSON

```json
{
  "id": "layer_comp_title",
  "type": "composition",
  "name": "Title Precomp",
  "enabled": true,
  "startTime": 0.0,
  "duration": 8.0,
  "inPoint": 0.0,
  "outPoint": 8.0,
  "parentId": null,
  "trackMatte": null,
  "blendMode": "normal",
  "opacity": { "value": 100, "keyframes": null },
  "transform": {
    "anchor": { "value": [960, 540], "keyframes": null },
    "position": { "value": [960, 320], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {
    "compositionId": "title_comp"
  },
  "shapeContents": null,
  "effects": null,
  "masks": null
}
```

### Common pitfalls and cautions

- `compositionId` must exist in top-level `compositions`.
- Avoid accidental recursive references (A includes B includes A).
- Keep nested composition durations intentional; trim at parent layer if needed.

### Performance considerations

- Deep nesting multiplies layer/effect evaluation work.
- Reusing a heavy precomp many times can be expensive without caching.
- Flatten static precomps to footage for final renders when practical.

---

## 11) Track Matte, Blend Mode, Effects, Masks (cross-cutting)

### `trackMatte` enum

- `alpha`
- `alphaInverted`
- `luma`
- `lumaInverted`

Use for stencil-style visibility control via a matte source layer.

### `blendMode` enum

- `normal`, `multiply`, `screen`, `overlay`
- `darken`, `lighten`, `colorDodge`, `colorBurn`
- `hardLight`, `softLight`, `difference`, `exclusion`
- `hue`, `saturation`, `color`, `luminosity`
- `add`, `subtract`, `divide`

Use blend modes to control color interaction between the current layer and already-rendered result beneath it.

### `effects`

Each effect item structure:

```json
{
  "id": "fx_001",
  "type": "gaussianBlur",
  "enabled": true,
  "params": { "radius": 12 },
  "keyframes": null
}
```

### `masks`

Mask structure:

```json
{
  "name": "Mask 1",
  "mode": "add",
  "path": [
    { "x": 100, "y": 100, "handleIn": null, "handleOut": null },
    { "x": 500, "y": 100, "handleIn": null, "handleOut": null },
    { "x": 500, "y": 400, "handleIn": null, "handleOut": null },
    { "x": 100, "y": 400, "handleIn": null, "handleOut": null }
  ],
  "feather": 0,
  "opacity": 100
}
```

---

## 12) Practical build patterns

### Pattern A: Standard motion-graphics stack

1. Bottom: `solid` background
2. Mid: `shape` panels and `image` assets
3. Top: `text` titles
4. Controller: `null` parent for grouped moves
5. Optional top pass: `adjustment` for global grade

### Pattern B: Modular precomp workflow

1. Build reusable module in separate composition
2. Reference with `composition` layers in main timeline
3. Reposition/retime each instance independently

### Pattern C: Footage + graphics

1. Base `video` clip
2. Overlay graphics (`shape`, `image`, `text`)
3. Audio bed via `audio` layer
4. Optional grading via `adjustment`

---

## 13) Validation checklist for layer JSON

- Layer `type` matches payload shape (`content` and/or `shapeContents`).
- IDs are unique in each composition.
- `assetId` / `compositionId` references resolve.
- `startTime`, `duration`, `inPoint`, `outPoint` produce intended active range.
- Parent chains are valid (`parentId` points to real layer id).
- Layer order reflects intended z-index.
- Enum strings are exact and case-sensitive.

---

## 14) Implementation-status caution (important)

The schema supports all layer types above, but renderer behavior can evolve by version. In current core rendering code, direct rasterization paths are strongest for:
- `solid`
- `text`
- `shape` (basic rectangle path currently explicit)

Treat advanced behavior (complex shape operators, adjustment routing, nested precomp rendering, full media playback path, complete matte/effect stacks) as version-dependent and verify with `outocut preview` / `outocut render` in your target release.
