# Effects Reference

Comprehensive reference for OutOcut's layer effects system.

- Author: BLOUplanet
- License: Apache 2.0

## Effects System Overview

Effects are defined per layer in `layer.effects` and evaluated in stack order.

```json
{
  "effects": [
    {
      "id": "fx_01",
      "type": "dropShadow",
      "enabled": true,
      "params": {
        "distance": 12,
        "angle": 135,
        "blur": 20,
        "color": "#000000",
        "opacity": 60
      },
      "keyframes": {
        "distance": [
          { "time": 0, "value": 4 },
          { "time": 1.0, "value": 16, "easing": "easeOutCubic" }
        ]
      }
    }
  ]
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | Yes | Unique effect instance ID in the layer. |
| `type` | string | Yes | Camel-case effect identifier (from `EffectType`). |
| `enabled` | boolean | Yes | Enables/disables effect without removing it. |
| `params` | object | Yes | Effect-specific parameter object. |
| `keyframes` | object or `null` | No | Per-parameter keyframe tracks (by param name). |

## Important Rules and Cautions

1. Effect order matters: effects are applied top-to-bottom in `effects[]`; changing order changes output.
2. Stack cost is cumulative: each additional effect can add full-frame processing work.
3. Keyframes are per param: put tracks under `keyframes.<paramName>`.
4. Not all params are equally keyframe-friendly: scalars are safest; nested objects/arrays may require static values.
5. Multiple blur effects can produce non-linear results and may not visually "stack" as expected.

## Keyframe Structure for Effects

```json
{
  "keyframes": {
    "radius": [
      { "time": 0, "value": 0 },
      { "time": 0.6, "value": 18, "easing": "easeOutCubic" }
    ],
    "opacity": [
      { "time": 0, "value": 20 },
      { "time": 1.0, "value": 75 }
    ]
  }
}
```

## Performance Scale

| Level | Typical Cost |
|---|---|
| Low | Minimal per-pixel math, usually real-time friendly at 1080p. |
| Medium | Noticeable extra processing, especially with many layers. |
| High | Expensive sampling/compositing; can significantly slow frame rendering. |

## Effect Index

The effect registry currently exposes these effect types.

Note: project philosophy docs reference "~30 effects" in v1.0, but `src/models.rs` currently defines 25 concrete `EffectType` identifiers. This reference documents all currently defined identifiers.

| # | Effect | `type` | Keyframes | Perf | Primary Use |
|---|---|---|---|---|---|
| 1 | Drop Shadow | `dropShadow` | Yes | Medium | Depth and separation from background |
| 2 | Inner Shadow | `innerShadow` | Yes | Medium | Engraved/inset styling |
| 3 | Glow | `glow` | Yes | Medium | Soft emitted light |
| 4 | Outer Glow | `outerGlow` | Yes | Medium | Halo around outside edges |
| 5 | Gaussian Blur | `gaussianBlur` | Yes | High | Defocus, atmospheric softness |
| 6 | Directional Blur | `directionalBlur` | Yes | High | Motion streak in one direction |
| 7 | Radial Blur | `radialBlur` | Yes | High | Spin/zoom radiating blur |
| 8 | Crop | `crop` | Yes | Low | Edge trimming and reveal windows |
| 9 | Rotate | `rotate` | Yes | Low | Extra post-transform rotation |
| 10 | Flip | `flip` | Yes | Low | Horizontal/vertical inversion |
| 11 | Mirror | `mirror` | Yes | Medium | Reflection/tile duplication |
| 12 | Color Correction | `colorCorrection` | Yes | Medium | Overall color balancing |
| 13 | Brightness/Contrast | `brightnessContrast` | Yes | Low | Quick tonal tuning |
| 14 | Hue/Saturation | `hueSaturation` | Yes | Medium | Palette shifting and intensity |
| 15 | Levels | `levels` | Yes | Medium | Input/output remapping |
| 16 | Curves | `curves` | Partial | Medium | Filmic tonal shaping |
| 17 | Chroma Key | `chromaKey` | Yes | High | Green/blue screen keying |
| 18 | Noise | `noise` | Yes | Medium | Grain and texture |
| 19 | Vignette | `vignette` | Yes | Low | Edge darkening/focus |
| 20 | Glow Effect | `glowEffect` | Yes | Medium | Alternate glow pipeline |
| 21 | Stroke | `stroke` | Yes | Medium | Outlines around alpha/paths |
| 22 | Fill Gradient | `fillGradient` | Partial | Medium | Gradient recolor/fill |
| 23 | Trim Path | `trimPath` | Yes | Medium | Shape draw-on effects |
| 24 | Wiggle | `wiggle` | Yes | Low | Procedural jitter animation |
| 25 | Text Animator | `textAnimator` | Partial | Medium | Text-specific animated styling |

---

## Shadow Effects

### 1) Drop Shadow (`dropShadow`)

Purpose: Adds an offset shadow behind the layer to improve depth and readability.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `distance` | number | `>= 0` px | Shadow offset distance. |
| `angle` | number | `0-360` deg | Direction of shadow offset. |
| `blur` | number | `>= 0` px | Softness radius. |
| `color` | string | Hex color | Shadow tint. |
| `opacity` | number | `0-100` | Shadow visibility. |

Keyframes: Supported for all params.

```json
{
  "id": "shadow_1",
  "type": "dropShadow",
  "enabled": true,
  "params": {
    "distance": 12,
    "angle": 120,
    "blur": 18,
    "color": "#000000",
    "opacity": 55
  },
  "keyframes": {
    "distance": [
      { "time": 0, "value": 6 },
      { "time": 1.2, "value": 20 }
    ]
  }
}
```

Common use cases: title text separation, UI card depth, logo pop from busy backgrounds.

Performance impact: Medium; blur radius dominates cost.

### 2) Inner Shadow (`innerShadow`)

Purpose: Creates inward shading at layer edges for inset or embossed looks.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `distance` | number | `>= 0` px | Inward offset distance. |
| `angle` | number | `0-360` deg | Direction of inner shading. |
| `blur` | number | `>= 0` px | Edge softness. |
| `color` | string | Hex color | Shadow color. |
| `opacity` | number | `0-100` | Shadow intensity. |
| `choke` | number | `0-100` | Hardens edge before blur. |

Keyframes: Supported for all scalar params.

```json
{
  "id": "inner_1",
  "type": "innerShadow",
  "enabled": true,
  "params": {
    "distance": 6,
    "angle": 315,
    "blur": 10,
    "color": "#000000",
    "opacity": 35,
    "choke": 15
  },
  "keyframes": null
}
```

Common use cases: inset buttons, beveled icons, pseudo-3D panel styling.

Performance impact: Medium.

### 3) Glow (`glow`)

Purpose: Adds soft light around bright or alpha edges.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `radius` | number | `>= 0` px | Glow spread size. |
| `color` | string | Hex color | Glow tint. |
| `opacity` | number | `0-100` | Glow intensity. |
| `threshold` | number | `0-100` | Luma cutoff for selective glow. |

Keyframes: Supported for all params.

```json
{
  "id": "glow_1",
  "type": "glow",
  "enabled": true,
  "params": {
    "radius": 24,
    "color": "#00ccff",
    "opacity": 80,
    "threshold": 35
  },
  "keyframes": {
    "opacity": [
      { "time": 0, "value": 25 },
      { "time": 0.8, "value": 80 }
    ]
  }
}
```

Common use cases: neon text, sci-fi HUDs, magical highlights.

Performance impact: Medium.

### 4) Outer Glow (`outerGlow`)

Purpose: Applies glow strictly outside opaque regions.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `radius` | number | `>= 0` px | Outer halo size. |
| `color` | string | Hex color | Halo color. |
| `opacity` | number | `0-100` | Halo intensity. |
| `spread` | number | `0-100` | Fills glow core before falloff. |

Keyframes: Supported for scalar params.

```json
{
  "id": "outer_glow_1",
  "type": "outerGlow",
  "enabled": true,
  "params": {
    "radius": 30,
    "color": "#ffaa00",
    "opacity": 70,
    "spread": 20
  },
  "keyframes": null
}
```

Common use cases: sticker outline lighting, logo halo, readable subtitle edge glow.

Performance impact: Medium.

---

## Blur Effects

### 5) Gaussian Blur (`gaussianBlur`)

Purpose: Smooth isotropic blur used for defocus and background softening.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `radius` | number | `>= 0` px | Blur radius. |
| `iterations` | integer | `1+` | Additional blur passes. |
| `edgeMode` | string | `clamp`, `wrap`, `mirror` | Edge handling behavior. |

Keyframes: Supported for scalar params.

```json
{
  "id": "gblur_1",
  "type": "gaussianBlur",
  "enabled": true,
  "params": {
    "radius": 14,
    "iterations": 1,
    "edgeMode": "clamp"
  },
  "keyframes": {
    "radius": [
      { "time": 0, "value": 0 },
      { "time": 1, "value": 14 }
    ]
  }
}
```

Common use cases: depth-of-field simulation, frosted glass UI, de-emphasizing background layers.

Performance impact: High, especially at large radius and high resolutions.

### 6) Directional Blur (`directionalBlur`)

Purpose: Blurs pixels along a line to simulate directional motion.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `distance` | number | `>= 0` px | Blur travel length. |
| `angle` | number | `0-360` deg | Blur direction. |
| `samples` | integer | `3+` | Number of sampling taps. |

Keyframes: Supported.

```json
{
  "id": "dblur_1",
  "type": "directionalBlur",
  "enabled": true,
  "params": {
    "distance": 22,
    "angle": 90,
    "samples": 16
  },
  "keyframes": {
    "angle": [
      { "time": 0, "value": 0 },
      { "time": 2, "value": 180 }
    ]
  }
}
```

Common use cases: fast pans, swipe transitions, speed lines.

Performance impact: High.

### 7) Radial Blur (`radialBlur`)

Purpose: Blurs radially from a center point (spin or zoom style).

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `amount` | number | `0-100` | Blur strength. |
| `center` | array<number> | `[x, y]` | Radial origin in pixels. |
| `mode` | string | `spin`, `zoom` | Radial blur mode. |
| `samples` | integer | `4+` | Sampling quality. |

Keyframes: Supported; animate `center` carefully.

```json
{
  "id": "rblur_1",
  "type": "radialBlur",
  "enabled": true,
  "params": {
    "amount": 28,
    "center": [960, 540],
    "mode": "zoom",
    "samples": 20
  },
  "keyframes": null
}
```

Common use cases: impact zooms, rotational energy effects, warp transitions.

Performance impact: High.

---

## Transform Effects

### 8) Crop (`crop`)

Purpose: Trims layer edges after base transforms.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `left` | number | `>= 0` px | Crop from left edge. |
| `top` | number | `>= 0` px | Crop from top edge. |
| `right` | number | `>= 0` px | Crop from right edge. |
| `bottom` | number | `>= 0` px | Crop from bottom edge. |
| `feather` | number | `>= 0` px | Soft crop edge falloff. |

Keyframes: Supported for all edge params.

```json
{
  "id": "crop_1",
  "type": "crop",
  "enabled": true,
  "params": {
    "left": 120,
    "top": 0,
    "right": 120,
    "bottom": 0,
    "feather": 0
  },
  "keyframes": {
    "left": [
      { "time": 0, "value": 500 },
      { "time": 1.0, "value": 120 }
    ]
  }
}
```

Common use cases: reveal animations, safe-area trims, split-screen framing.

Performance impact: Low.

### 9) Rotate (`rotate`)

Purpose: Adds extra rotation as an effect-stage operation.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `angle` | number | any degree | Additional rotation angle. |
| `center` | array<number> | `[x, y]` | Rotation pivot override. |
| `resample` | string | `nearest`, `bilinear` | Sampling filter. |

Keyframes: Supported.

```json
{
  "id": "rotate_fx_1",
  "type": "rotate",
  "enabled": true,
  "params": {
    "angle": 15,
    "center": [960, 540],
    "resample": "bilinear"
  },
  "keyframes": {
    "angle": [
      { "time": 0, "value": -10 },
      { "time": 1.2, "value": 15 }
    ]
  }
}
```

Common use cases: post-transform polish, stylized skew-spin composites.

Performance impact: Low.

### 10) Flip (`flip`)

Purpose: Mirrors content horizontally, vertically, or both.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `horizontal` | boolean | `true/false` | Flip on X axis. |
| `vertical` | boolean | `true/false` | Flip on Y axis. |

Keyframes: Supported (boolean step changes).

```json
{
  "id": "flip_1",
  "type": "flip",
  "enabled": true,
  "params": {
    "horizontal": true,
    "vertical": false
  },
  "keyframes": null
}
```

Common use cases: mirrored motion variants, quick orientation fixes.

Performance impact: Low.

### 11) Mirror (`mirror`)

Purpose: Reflects and repeats content across a configurable axis.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `axis` | string | `horizontal`, `vertical`, `angle` | Mirror axis mode. |
| `angle` | number | `0-360` deg | Axis angle when `axis=angle`. |
| `offset` | number | px | Reflection offset from center. |
| `repeat` | integer | `1+` | Number of repeated reflections. |

Keyframes: Supported.

```json
{
  "id": "mirror_1",
  "type": "mirror",
  "enabled": true,
  "params": {
    "axis": "vertical",
    "angle": 0,
    "offset": 0,
    "repeat": 2
  },
  "keyframes": null
}
```

Common use cases: kaleidoscope look, symmetrical backgrounds, abstract motion.

Performance impact: Medium.

---

## Color Correction Effects

### 12) Color Correction (`colorCorrection`)

Purpose: General-purpose color balancing in one pass.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `temperature` | number | negative to positive | Cool/warm white balance shift. |
| `tint` | number | negative to positive | Green-magenta shift. |
| `exposure` | number | stops | Exposure adjustment. |
| `gamma` | number | `> 0` | Midtone curve shift. |
| `saturation` | number | `0-200` | Global color intensity. |

Keyframes: Supported.

```json
{
  "id": "cc_1",
  "type": "colorCorrection",
  "enabled": true,
  "params": {
    "temperature": 8,
    "tint": -3,
    "exposure": 0.2,
    "gamma": 1.0,
    "saturation": 112
  },
  "keyframes": null
}
```

Common use cases: quick grade pass, shot matching, atmosphere shifts.

Performance impact: Medium.

### 13) Brightness/Contrast (`brightnessContrast`)

Purpose: Fast tonal adjustment for luma separation.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `brightness` | number | `-100` to `100` | Raises/lowers overall luma. |
| `contrast` | number | `-100` to `100` | Expands/compresses contrast. |
| `preserveHighlights` | boolean | `true/false` | Reduces clipping in bright areas. |

Keyframes: Supported.

```json
{
  "id": "bc_1",
  "type": "brightnessContrast",
  "enabled": true,
  "params": {
    "brightness": 6,
    "contrast": 20,
    "preserveHighlights": true
  },
  "keyframes": null
}
```

Common use cases: low-contrast footage recovery, stylized punchier look.

Performance impact: Low.

### 14) Hue/Saturation (`hueSaturation`)

Purpose: Rotates hue and adjusts saturation/lightness globally.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `hue` | number | `-180` to `180` | Hue rotation in degrees. |
| `saturation` | number | `0-200` | Saturation percent. |
| `lightness` | number | `-100` to `100` | Luma shift. |
| `colorize` | boolean | `true/false` | Uses single-hue colorize mode. |
| `colorizeHue` | number | `0-360` | Hue used in colorize mode. |

Keyframes: Supported.

```json
{
  "id": "hs_1",
  "type": "hueSaturation",
  "enabled": true,
  "params": {
    "hue": 15,
    "saturation": 120,
    "lightness": -4,
    "colorize": false,
    "colorizeHue": 0
  },
  "keyframes": {
    "hue": [
      { "time": 0, "value": 0 },
      { "time": 2, "value": 60 }
    ]
  }
}
```

Common use cases: day/night recolor, brand color alignment, psychedelic shifts.

Performance impact: Medium.

### 15) Levels (`levels`)

Purpose: Remaps input and output tonal ranges with gamma control.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `inputBlack` | number | `0-255` | Black point of source input. |
| `inputWhite` | number | `0-255` | White point of source input. |
| `gamma` | number | `> 0` | Midtone distribution. |
| `outputBlack` | number | `0-255` | Black floor of output. |
| `outputWhite` | number | `0-255` | White ceiling of output. |

Keyframes: Supported.

```json
{
  "id": "levels_1",
  "type": "levels",
  "enabled": true,
  "params": {
    "inputBlack": 8,
    "inputWhite": 245,
    "gamma": 1.05,
    "outputBlack": 0,
    "outputWhite": 255
  },
  "keyframes": null
}
```

Common use cases: clipping control, contrast restoration, flat-log footage shaping.

Performance impact: Medium.

### 16) Curves (`curves`)

Purpose: Fine-grained tonal remapping via curve control points.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `master` | array<object> | `[{"x":0-1,"y":0-1}, ...]` | Global RGB/luma curve points. |
| `red` | array<object> | optional | Red channel curve points. |
| `green` | array<object> | optional | Green channel curve points. |
| `blue` | array<object> | optional | Blue channel curve points. |

Keyframes: Partial; scalar toggles keyframe cleanly, full curve-point arrays may be static in many workflows.

```json
{
  "id": "curves_1",
  "type": "curves",
  "enabled": true,
  "params": {
    "master": [
      { "x": 0.0, "y": 0.0 },
      { "x": 0.45, "y": 0.38 },
      { "x": 1.0, "y": 1.0 }
    ],
    "red": [],
    "green": [],
    "blue": []
  },
  "keyframes": null
}
```

Common use cases: filmic S-curves, channel-specific color cast cleanup.

Performance impact: Medium.

---

## Special Effects

### 17) Chroma Key (`chromaKey`)

Purpose: Removes a selected key color (typically green/blue screen).

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `keyColor` | string | Hex color | Target background color to remove. |
| `similarity` | number | `0-100` | Color matching tolerance. |
| `smoothness` | number | `0-100` | Edge blending softness. |
| `spillSuppression` | number | `0-100` | Removes reflected key color spill. |
| `despillColor` | string | Hex color | Preferred replacement tint. |

Keyframes: Supported.

```json
{
  "id": "ck_1",
  "type": "chromaKey",
  "enabled": true,
  "params": {
    "keyColor": "#00ff00",
    "similarity": 35,
    "smoothness": 22,
    "spillSuppression": 50,
    "despillColor": "#808080"
  },
  "keyframes": null
}
```

Common use cases: presenter compositing, virtual backgrounds, product demos.

Performance impact: High.

### 18) Noise (`noise`)

Purpose: Adds procedural noise/grain for texture and anti-banding.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `amount` | number | `0-100` | Noise intensity. |
| `monochrome` | boolean | `true/false` | Uses luma-only grain. |
| `seed` | integer | any | Deterministic noise pattern seed. |
| `animated` | boolean | `true/false` | Changes pattern over time. |

Keyframes: Supported; animate `amount` or `seed` for transitions.

```json
{
  "id": "noise_1",
  "type": "noise",
  "enabled": true,
  "params": {
    "amount": 12,
    "monochrome": true,
    "seed": 42,
    "animated": false
  },
  "keyframes": null
}
```

Common use cases: film emulation, gradient breakup, retro texture overlays.

Performance impact: Medium.

### 19) Vignette (`vignette`)

Purpose: Darkens or colors edges to draw attention to center.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `amount` | number | `-100` to `100` | Vignette strength and direction. |
| `size` | number | `0-100` | Radius/coverage of center region. |
| `roundness` | number | `-100` to `100` | Circle-to-rectangle shape bias. |
| `feather` | number | `0-100` | Edge softness. |
| `color` | string | Hex color | Vignette tint (not only black). |

Keyframes: Supported.

```json
{
  "id": "vignette_1",
  "type": "vignette",
  "enabled": true,
  "params": {
    "amount": 35,
    "size": 65,
    "roundness": 25,
    "feather": 70,
    "color": "#000000"
  },
  "keyframes": null
}
```

Common use cases: focus guidance, cinematic framing, period look.

Performance impact: Low.

### 20) Glow Effect (`glowEffect`)

Purpose: Alternate glow variant for stylized bloom pipelines.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `intensity` | number | `0-100` | Overall bloom strength. |
| `radius` | number | `>= 0` px | Bloom spread. |
| `blendMode` | string | `screen`, `add`, `softLight` | Composite mode for glow pass. |
| `color` | string | Hex color | Glow tint. |

Keyframes: Supported.

```json
{
  "id": "glow_fx_1",
  "type": "glowEffect",
  "enabled": true,
  "params": {
    "intensity": 72,
    "radius": 20,
    "blendMode": "screen",
    "color": "#88ddff"
  },
  "keyframes": null
}
```

Common use cases: bloom-heavy motion graphics, dreamy title treatments.

Performance impact: Medium.

---

## Styling Effects

### 21) Stroke (`stroke`)

Purpose: Draws an outline around alpha boundaries or vector paths.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `width` | number | `>= 0` px | Stroke thickness. |
| `color` | string | Hex color | Stroke color. |
| `opacity` | number | `0-100` | Stroke intensity. |
| `position` | string | `inside`, `center`, `outside` | Stroke alignment. |
| `join` | string | `miter`, `round`, `bevel` | Corner join style. |

Keyframes: Supported.

```json
{
  "id": "stroke_1",
  "type": "stroke",
  "enabled": true,
  "params": {
    "width": 6,
    "color": "#ffffff",
    "opacity": 100,
    "position": "outside",
    "join": "round"
  },
  "keyframes": {
    "width": [
      { "time": 0, "value": 0 },
      { "time": 0.5, "value": 6 }
    ]
  }
}
```

Common use cases: text legibility, logo stickers, shape accents.

Performance impact: Medium.

### 22) Fill Gradient (`fillGradient`)

Purpose: Replaces/overlays fill with linear or radial gradient.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `type` | string | `linear`, `radial` | Gradient mode. |
| `start` | array<number> | `[x, y]` | Gradient start point. |
| `end` | array<number> | `[x, y]` | Gradient end point. |
| `stops` | array<object> | `[{"pos":0-1,"color":"#..."}, ...]` | Color stop list. |
| `opacity` | number | `0-100` | Blend amount of gradient fill. |

Keyframes: Partial; points and opacity animate well, complex `stops` arrays are commonly static.

```json
{
  "id": "gradient_1",
  "type": "fillGradient",
  "enabled": true,
  "params": {
    "type": "linear",
    "start": [0, 0],
    "end": [1920, 1080],
    "stops": [
      { "pos": 0.0, "color": "#ff6600" },
      { "pos": 1.0, "color": "#2222ff" }
    ],
    "opacity": 100
  },
  "keyframes": null
}
```

Common use cases: brand gradients, depth tint overlays, modern title cards.

Performance impact: Medium.

### 23) Trim Path (`trimPath`)

Purpose: Reveals/hides shape path segments over time.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `start` | number | `0-100` | Start percentage of visible path. |
| `end` | number | `0-100` | End percentage of visible path. |
| `offset` | number | any degree/percent | Rotational/phase offset. |
| `mode` | string | `simultaneous`, `individual` | Multi-path handling mode. |

Keyframes: Supported and commonly used.

```json
{
  "id": "trim_1",
  "type": "trimPath",
  "enabled": true,
  "params": {
    "start": 0,
    "end": 0,
    "offset": 0,
    "mode": "simultaneous"
  },
  "keyframes": {
    "end": [
      { "time": 0, "value": 0 },
      { "time": 1.5, "value": 100 }
    ]
  }
}
```

Common use cases: line draw-on animation, logo tracing, UI path reveals.

Performance impact: Medium.

---

## Animation Effects

### 24) Wiggle (`wiggle`)

Purpose: Procedural pseudo-random animation for jitter and organic motion.

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `frequency` | number | `>= 0` Hz | How often value changes. |
| `amplitude` | number | `>= 0` | Maximum offset magnitude. |
| `seed` | integer | any | Random seed for deterministic motion. |
| `dimensions` | string | `x`, `y`, `xy`, `rotation` | Target channel(s). |

Keyframes: Supported; often static with animated `amplitude` ramps.

```json
{
  "id": "wiggle_1",
  "type": "wiggle",
  "enabled": true,
  "params": {
    "frequency": 3.5,
    "amplitude": 18,
    "seed": 7,
    "dimensions": "xy"
  },
  "keyframes": {
    "amplitude": [
      { "time": 0, "value": 0 },
      { "time": 0.4, "value": 18 }
    ]
  }
}
```

Common use cases: handheld camera feel, subtle text shake, glitch motion base layer.

Performance impact: Low.

### 25) Text Animator (`textAnimator`)

Purpose: Text-targeted animation controls (tracking, offset, opacity, per-range styling).

| Param | Type | Range / Expected | Description |
|---|---|---|---|
| `property` | string | `position`, `opacity`, `scale`, `rotation`, `tracking` | Text property to animate. |
| `amount` | number or array<number> | depends on `property` | Animation value/offset. |
| `selector` | object | range/expression selector | Defines affected character range. |
| `units` | string | `percent`, `index` | Selector interpretation mode. |
| `ease` | string | easing name | Selector falloff shape. |

Keyframes: Partial in v1.0-era workflows; supported in schema but per-character behavior may be limited.

```json
{
  "id": "text_anim_1",
  "type": "textAnimator",
  "enabled": true,
  "params": {
    "property": "opacity",
    "amount": -100,
    "selector": {
      "start": 0,
      "end": 100,
      "basedOn": "characters"
    },
    "units": "percent",
    "ease": "easeOutCubic"
  },
  "keyframes": {
    "selector.end": [
      { "time": 0, "value": 0 },
      { "time": 1.2, "value": 100 }
    ]
  }
}
```

Common use cases: type-on reveals, per-character fades, kinetic typography.

Performance impact: Medium; increases with long text and complex selectors.

---

## Effect Stack Strategy

Recommended ordering pattern (general):

1. Utility and geometry effects: `crop`, `flip`, `mirror`, `rotate`
2. Matte and cleanup effects: `chromaKey`
3. Color and tonal effects: `colorCorrection`, `levels`, `curves`, `hueSaturation`
4. Detail styling: `stroke`, `fillGradient`, `trimPath`
5. Blur and light wraps: `gaussianBlur`, `directionalBlur`, `radialBlur`, `glow`
6. Final mood effects: `vignette`, `noise`

Practical caution points:

- Reordering blur and glow can drastically change perceived edge quality.
- Place `chromaKey` early; blurring before keying usually worsens matte quality.
- Avoid stacking multiple heavy blurs on the same layer when possible.
- Consider precomposing expensive stacks to isolate and reuse results.

## Notes on Support Scope

- OutOcut's schema accepts `params` as flexible JSON and `keyframes` as optional JSON tracks.
- Use scalar param tracks for safest interpolation behavior.
- Complex object/array param animation should be tested per project before relying on it at scale.
