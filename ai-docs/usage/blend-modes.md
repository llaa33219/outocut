# Blend Modes

Complete reference for OutOcut layer blend modes.

Author: BLOUplanet  
License: Apache 2.0

## Overview

Blend modes control how a layer (the **top** or blend layer) combines with pixels already in the frame (the **bottom** or base layer).

- `blendMode` is an optional layer field.
- If omitted (`null` or missing), behavior defaults to `normal`.
- Layer order matters: swapping two layers usually changes the result.
- Blend math is evaluated per channel (`R`, `G`, `B`) on normalized values in `[0, 1]`.

### Layer Field

```json
{
  "id": "highlight",
  "type": "solid",
  "blendMode": "screen",
  "opacity": { "value": 65, "keyframes": null }
}
```

### Notation Used in Formulas

- `B`: base channel value (bottom layer)
- `F`: blend channel value (top layer)
- `R`: result channel before final alpha compositing
- `clamp(x)`: limit to `[0, 1]`
- `eps`: very small number to avoid divide-by-zero (for practical implementations)

## Global Performance and Behavior Notes

- Fastest modes are usually `normal`, `multiply`, `screen`, `darken`, `lighten`, `add`, `subtract`.
- Mid-cost modes include `overlay`, `hardLight`, `difference`, `exclusion`, `divide`.
- More expensive modes are typically `colorDodge`, `colorBurn`, `softLight`, and HSL-based modes (`hue`, `saturation`, `color`, `luminosity`) because they require conditional math and/or color-space conversion.
- Heavy stacks of blend modes can increase render time, especially at 4K+ resolutions.
- Semi-transparent edges can look different from expectations when many blended layers overlap.
- For predictable output, precompose complex stacks and test with real footage.

---

## 1) Normal

- **Name**: Normal
- **JSON value**: `"normal"`
- **Visual effect**: Standard over compositing. Top layer simply appears over bottom layer.
- **Formula**: `R = F` (then alpha compositing with layer opacity)
- **Common use cases**: Most layers, UI elements, titles, cutout assets, default behavior.
- **Example JSON**:

```json
{
  "id": "title",
  "type": "text",
  "blendMode": "normal"
}
```

- **Performance notes**: Lowest cost baseline mode.

## 2) Multiply

- **Name**: Multiply
- **JSON value**: `"multiply"`
- **Visual effect**: Darkens image by multiplying base and blend. White has little effect; black forces black.
- **Formula**: `R = B * F`
- **Common use cases**: Shadows, dirt/grain overlays, texture integration, darkening bright footage.
- **Example JSON**:

```json
{
  "id": "shadow_overlay",
  "type": "solid",
  "blendMode": "multiply",
  "content": { "color": "#5a4a3a" }
}
```

- **Performance notes**: Very fast (simple multiply per channel).

## 3) Screen

- **Name**: Screen
- **JSON value**: `"screen"`
- **Visual effect**: Brightens by inverting both layers, multiplying, then inverting back. Black has little effect.
- **Formula**: `R = 1 - (1 - B) * (1 - F)`
- **Common use cases**: Light leaks, glow passes, lens flares, fire/sparks, additive-looking brightening with softer clipping.
- **Example JSON**:

```json
{
  "id": "light_pass",
  "type": "image",
  "blendMode": "screen"
}
```

- **Performance notes**: Very fast; common real-time friendly lightening mode.

## 4) Overlay

- **Name**: Overlay
- **JSON value**: `"overlay"`
- **Visual effect**: Combines multiply in dark regions and screen in bright regions, increasing contrast.
- **Formula**:
  - `R = 2 * B * F` if `B < 0.5`
  - `R = 1 - 2 * (1 - B) * (1 - F)` otherwise
- **Common use cases**: Contrast pop, texture-on-footage, cinematic punch, stylized grade layers.
- **Example JSON**:

```json
{
  "id": "contrast_texture",
  "type": "image",
  "blendMode": "overlay",
  "opacity": { "value": 35, "keyframes": null }
}
```

- **Performance notes**: Low-to-medium cost due to branching.

## 5) Darken

- **Name**: Darken
- **JSON value**: `"darken"`
- **Visual effect**: Keeps the darker value from base or blend per channel.
- **Formula**: `R = min(B, F)`
- **Common use cases**: Replace bright background noise, merge shadows, suppress highlights.
- **Example JSON**:

```json
{
  "id": "shadow_merge",
  "type": "video",
  "blendMode": "darken"
}
```

- **Performance notes**: Very fast (`min` comparison only).

## 6) Lighten

- **Name**: Lighten
- **JSON value**: `"lighten"`
- **Visual effect**: Keeps the lighter value from base or blend per channel.
- **Formula**: `R = max(B, F)`
- **Common use cases**: Glow/luma extraction style merges, preserving bright particles or highlights.
- **Example JSON**:

```json
{
  "id": "spark_pass",
  "type": "video",
  "blendMode": "lighten"
}
```

- **Performance notes**: Very fast (`max` comparison only).

## 7) Color Dodge

- **Name**: Color Dodge
- **JSON value**: `"colorDodge"`
- **Visual effect**: Strongly brightens and saturates highlights; can blow out whites quickly.
- **Formula**:
  - `R = 1` if `F = 1`
  - `R = min(1, B / (1 - F))` otherwise
- **Common use cases**: Intense light rays, magical glints, specular boost, stylized bloom accent.
- **Example JSON**:

```json
{
  "id": "ray_boost",
  "type": "solid",
  "blendMode": "colorDodge",
  "content": { "color": "#ffd080" },
  "opacity": { "value": 25, "keyframes": null }
}
```

- **Performance notes**: Medium-to-high cost (division + clamp + edge handling).

## 8) Color Burn

- **Name**: Color Burn
- **JSON value**: `"colorBurn"`
- **Visual effect**: Aggressive darkening with strong contrast/saturation in darker tones.
- **Formula**:
  - `R = 0` if `F = 0`
  - `R = 1 - min(1, (1 - B) / F)` otherwise
- **Common use cases**: Gritty stylization, heavy shadow shaping, distressed film looks.
- **Example JSON**:

```json
{
  "id": "grit_grade",
  "type": "image",
  "blendMode": "colorBurn",
  "opacity": { "value": 20, "keyframes": null }
}
```

- **Performance notes**: Medium-to-high cost (division + branch logic).

## 9) Hard Light

- **Name**: Hard Light
- **JSON value**: `"hardLight"`
- **Visual effect**: Similar to overlay but controlled by blend layer; punchier contrast.
- **Formula**:
  - `R = 2 * B * F` if `F < 0.5`
  - `R = 1 - 2 * (1 - B) * (1 - F)` otherwise
- **Common use cases**: Dramatic graphic treatment, energetic promo styles, high-impact typography.
- **Example JSON**:

```json
{
  "id": "punch_layer",
  "type": "solid",
  "blendMode": "hardLight",
  "content": { "color": "#ff6a00" }
}
```

- **Performance notes**: Low-to-medium cost; similar class to overlay.

## 10) Soft Light

- **Name**: Soft Light
- **JSON value**: `"softLight"`
- **Visual effect**: Gentle contrast/light shaping; subtler than overlay/hard light.
- **Formula** (W3C-style piecewise):
  - If `F <= 0.5`: `R = B - (1 - 2F) * B * (1 - B)`
  - If `F > 0.5`: `R = B + (2F - 1) * (D(B) - B)`
  - Where `D(B) = ((16B - 12)B + 4)B` when `B <= 0.25`, else `sqrt(B)`
- **Common use cases**: Beauty pass, subtle grade unification, skin/portrait-friendly contrast.
- **Example JSON**:

```json
{
  "id": "soft_grade",
  "type": "image",
  "blendMode": "softLight",
  "opacity": { "value": 40, "keyframes": null }
}
```

- **Performance notes**: Medium-to-high cost (piecewise polynomial/sqrt operations).

## 11) Difference

- **Name**: Difference
- **JSON value**: `"difference"`
- **Visual effect**: Inverts where layers differ; identical colors become black.
- **Formula**: `R = abs(B - F)`
- **Common use cases**: Glitch look, alignment checking, inversion effects, transition tricks.
- **Example JSON**:

```json
{
  "id": "glitch_pass",
  "type": "video",
  "blendMode": "difference"
}
```

- **Performance notes**: Low cost (`abs` per channel).

## 12) Exclusion

- **Name**: Exclusion
- **JSON value**: `"exclusion"`
- **Visual effect**: Similar to difference but softer and lower-contrast.
- **Formula**: `R = B + F - 2 * B * F`
- **Common use cases**: Mild negative look, moody overlays, color interplay without harsh inversion.
- **Example JSON**:

```json
{
  "id": "soft_glitch",
  "type": "solid",
  "blendMode": "exclusion",
  "content": { "color": "#66ccff" }
}
```

- **Performance notes**: Low-to-medium cost.

## 13) Hue

- **Name**: Hue
- **JSON value**: `"hue"`
- **Visual effect**: Takes hue from blend layer while preserving saturation and luminosity from base.
- **Formula/algorithm**:
  1. Convert `B` and `F` from RGB to HSL.
  2. Build result as `H = H(F), S = S(B), L = L(B)`.
  3. Convert back to RGB.
- **Common use cases**: Recolor objects while keeping original shading/detail.
- **Example JSON**:

```json
{
  "id": "recolor_hue",
  "type": "solid",
  "blendMode": "hue",
  "content": { "color": "#00b3ff" },
  "opacity": { "value": 60, "keyframes": null }
}
```

- **Performance notes**: Higher cost due to RGB<->HSL conversion.

## 14) Saturation

- **Name**: Saturation
- **JSON value**: `"saturation"`
- **Visual effect**: Uses saturation from blend while preserving base hue and luminosity.
- **Formula/algorithm**:
  1. Convert both layers to HSL.
  2. Build result as `H = H(B), S = S(F), L = L(B)`.
  3. Convert back to RGB.
- **Common use cases**: Selective saturation control, stylized desaturation/oversaturation passes.
- **Example JSON**:

```json
{
  "id": "sat_control",
  "type": "solid",
  "blendMode": "saturation",
  "content": { "color": "#ff0000" },
  "opacity": { "value": 30, "keyframes": null }
}
```

- **Performance notes**: Higher cost (HSL conversion path).

## 15) Color

- **Name**: Color
- **JSON value**: `"color"`
- **Visual effect**: Uses hue and saturation from blend while preserving base luminosity.
- **Formula/algorithm**:
  1. Convert both layers to HSL.
  2. Build result as `H = H(F), S = S(F), L = L(B)`.
  3. Convert back to RGB.
- **Common use cases**: Color grading tint layers while retaining scene brightness structure.
- **Example JSON**:

```json
{
  "id": "grade_tint",
  "type": "solid",
  "blendMode": "color",
  "content": { "color": "#ffb347" },
  "opacity": { "value": 20, "keyframes": null }
}
```

- **Performance notes**: Higher cost; common in grading but heavier than simple RGB modes.

## 16) Luminosity

- **Name**: Luminosity
- **JSON value**: `"luminosity"`
- **Visual effect**: Uses luminosity from blend while preserving hue and saturation from base.
- **Formula/algorithm**:
  1. Convert both layers to HSL.
  2. Build result as `H = H(B), S = S(B), L = L(F)`.
  3. Convert back to RGB.
- **Common use cases**: Contrast/detail transfer, luma-driven texture merge, tone remapping.
- **Example JSON**:

```json
{
  "id": "luma_transfer",
  "type": "image",
  "blendMode": "luminosity",
  "opacity": { "value": 45, "keyframes": null }
}
```

- **Performance notes**: Higher cost (HSL conversion + luma replacement).

## 17) Add

- **Name**: Add (Linear Dodge)
- **JSON value**: `"add"`
- **Visual effect**: Simple additive brighten; quickly reaches white (clamped).
- **Formula**: `R = min(1, B + F)`
- **Common use cases**: Light accumulation, energy beams, glows, hot highlights.
- **Example JSON**:

```json
{
  "id": "energy_pass",
  "type": "solid",
  "blendMode": "add",
  "content": { "color": "#33ccff" },
  "opacity": { "value": 55, "keyframes": null }
}
```

- **Performance notes**: Very fast; watch for clipping in bright regions.

## 18) Subtract

- **Name**: Subtract
- **JSON value**: `"subtract"`
- **Visual effect**: Removes blend brightness from base, darkening toward black.
- **Formula**: `R = max(0, B - F)`
- **Common use cases**: Stylized darkening, mask-like subtraction looks, posterized mood passes.
- **Example JSON**:

```json
{
  "id": "dark_cut",
  "type": "solid",
  "blendMode": "subtract",
  "content": { "color": "#203050" },
  "opacity": { "value": 35, "keyframes": null }
}
```

- **Performance notes**: Very fast; may crush blacks quickly.

## 19) Divide

- **Name**: Divide
- **JSON value**: `"divide"`
- **Visual effect**: Divides base by blend; can brighten strongly where blend is dark.
- **Formula**: `R = clamp(B / max(F, eps))`
- **Common use cases**: Technical correction passes, unusual stylization, inverted shading effects.
- **Example JSON**:

```json
{
  "id": "divide_fx",
  "type": "image",
  "blendMode": "divide",
  "opacity": { "value": 22, "keyframes": null }
}
```

- **Performance notes**: Medium cost (division + zero-protection logic).

---

## Common Blend Combinations

- `multiply` + low-opacity color solid: quick shadow tinting.
- `screen` or `add` + blurred duplicate: practical glow/bloom stack.
- `overlay` followed by `softLight`: contrast first, gentle tonal polish second.
- `color` + `luminosity`: separate color tint from brightness shaping.
- `difference` animated with transform/offset: glitch transition base.

## Cautions and Best Practices

- Some blend modes can look unstable on semi-transparent edges, antialiased text, or thin lines.
- Results are order-dependent: moving a layer up/down can completely change color output.
- High-intensity modes (`add`, `colorDodge`, `colorBurn`, `divide`) can clip; reduce opacity first.
- HSL modes (`hue`, `saturation`, `color`, `luminosity`) are powerful but heavier for long/high-res renders.
- For large projects, precompose expensive blend stacks and reuse the result.

## Practical Default Strategy

1. Start with `normal`.
2. Try `multiply` for dark integration or `screen` for light integration.
3. Use `overlay`/`softLight` for contrast shaping.
4. Reserve `colorDodge`, `colorBurn`, `divide`, and HSL modes for targeted shots.
