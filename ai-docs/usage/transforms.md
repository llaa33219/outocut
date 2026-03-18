# Transform System

Comprehensive reference for OutOcut layer transforms.

- Author: BLOUplanet
- License: Apache 2.0

## Overview

Every layer has a `transform` object. Each transform field is an `AnimatedProperty`, so every value can be static or keyframed over time.

Transform fields:

1. `anchor` - `Vec<f64>` (`[x, y]`) - point around which transform occurs
2. `position` - `Vec<f64>` (`[x, y]`) - layer position in composition space
3. `scale` - `Vec<f64>` (`[x%, y%]`) - scale factors (`100` means 100%, negative values flip)
4. `rotation` - `f64` - rotation in degrees (`360` = full turn)
5. `skew` - `Vec<f64>` (`[x, y]`) - skew angles
6. `skewAxis` - `f64` - skew axis angle in degrees

## Default Values

When a transform is omitted or created with defaults:

- `anchor`: `[0, 0]`
- `position`: `[0, 0]`
- `scale`: `[100, 100]`
- `rotation`: `0`
- `skew`: `[0, 0]`
- `skewAxis`: `0`

Default JSON:

```json
"transform": {
  "anchor": { "value": [0, 0], "keyframes": null },
  "position": { "value": [0, 0], "keyframes": null },
  "scale": { "value": [100, 100], "keyframes": null },
  "rotation": { "value": 0, "keyframes": null },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```

## Transform Structure and AnimatedProperty

All transform fields follow the same `AnimatedProperty<T>` shape:

```json
{
  "value": [960, 540],
  "keyframes": [
    { "time": 0.0, "value": [0, 540], "easing": "easeOutCubic" },
    { "time": 1.2, "value": [960, 540] }
  ]
}
```

- `value`: static fallback value
- `keyframes`: `null` or array of keyframes
- keyframe `time` is seconds
- interpolation and easing behavior are defined by the animation system

Full transform structure example:

```json
"transform": {
  "anchor": { "value": [0, 0], "keyframes": null },
  "position": {
    "value": [960, 540],
    "keyframes": [
      { "time": 0.0, "value": [960, 660], "easing": "easeOutCubic" },
      { "time": 0.8, "value": [960, 540] }
    ]
  },
  "scale": { "value": [100, 100], "keyframes": null },
  "rotation": { "value": 0, "keyframes": null },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```

## How Transforms Are Applied (Order Matters)

OutOcut transform evaluation is order-sensitive. The conceptual transform stack is:

1. `anchor`
2. `position`
3. `scale`
4. `rotation`
5. `skew`

Think of this as matrix composition where each step builds on the previous step. Changing one property can change how later properties feel.

Practical implications:

- moving `anchor` changes pivot for scale and rotation
- scaling first and then rotating does not look the same as rotating first and then scaling
- skew is applied last, so it shears the already-positioned/scaled/rotated result

Visual intuition:

- `anchor` chooses the pin location inside the layer
- `position` places that pinned layer in comp space
- `scale` expands/shrinks relative to anchor
- `rotation` spins around anchor
- `skew` slants final geometry along `skewAxis`

## Anchor (Transform Origin) Behavior

`anchor` is the transform origin in layer-local coordinates.

- if `anchor` is `[0, 0]`, transforms pivot around top-left of layer space
- if `anchor` is `[width/2, height/2]`, transforms pivot around visual center
- animating `anchor` creates moving-pivot effects

Center-anchor example for a 400x200 layer:

```json
"anchor": {
  "value": [200, 100],
  "keyframes": null
}
```

Visual description:

- With anchor at center, a rotation looks like a clean spin.
- With anchor at left edge, the same rotation looks like a swinging door.

## Negative Scale and Flipping

`scale` uses percent values:

- `[100, 100]` = original size
- `[50, 50]` = half size
- `[200, 100]` = double width only
- `[-100, 100]` = horizontal flip
- `[100, -100]` = vertical flip
- `[-100, -100]` = horizontal + vertical flip (equivalent to 180-degree mirror around anchor)

Horizontal flip example:

```json
"scale": {
  "value": [-100, 100],
  "keyframes": null
}
```

## Combining Multiple Transforms

Transforms are designed to be combined. Typical animation stacks include position + scale + rotation together.

Example: intro pop + settle

```json
"transform": {
  "anchor": { "value": [200, 100], "keyframes": null },
  "position": {
    "value": [960, 540],
    "keyframes": [
      { "time": 0.0, "value": [960, 620], "easing": "easeOutCubic" },
      { "time": 0.6, "value": [960, 540] }
    ]
  },
  "scale": {
    "value": [100, 100],
    "keyframes": [
      { "time": 0.0, "value": [85, 85], "easing": "easeOutBack" },
      { "time": 0.6, "value": [100, 100] }
    ]
  },
  "rotation": {
    "value": 0,
    "keyframes": [
      { "time": 0.0, "value": -8, "easing": "easeOutCubic" },
      { "time": 0.6, "value": 0 }
    ]
  },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```

## Transform Inheritance with Parent Layers

You can build transform hierarchies with `parentId`.

- parent transform is evaluated first
- child local transform is applied relative to parent space
- final child transform is the composed parent + child result

Conceptual composition:

```text
child_world_matrix = parent_world_matrix * child_local_matrix
```

Hierarchy example:

```json
{
  "layers": [
    {
      "id": "ctrl_camera",
      "type": "null",
      "enabled": true,
      "startTime": 0,
      "duration": 10,
      "opacity": { "value": 100, "keyframes": null },
      "transform": {
        "anchor": { "value": [0, 0], "keyframes": null },
        "position": {
          "value": [0, 0],
          "keyframes": [
            { "time": 0, "value": [0, 0] },
            { "time": 10, "value": [-300, 0], "easing": "linear" }
          ]
        },
        "scale": { "value": [100, 100], "keyframes": null },
        "rotation": { "value": 0, "keyframes": null },
        "skew": { "value": [0, 0], "keyframes": null },
        "skewAxis": { "value": 0, "keyframes": null }
      }
    },
    {
      "id": "logo",
      "type": "image",
      "parentId": "ctrl_camera",
      "enabled": true,
      "startTime": 0,
      "duration": 10,
      "opacity": { "value": 100, "keyframes": null },
      "transform": {
        "anchor": { "value": [256, 256], "keyframes": null },
        "position": { "value": [960, 540], "keyframes": null },
        "scale": { "value": [100, 100], "keyframes": null },
        "rotation": { "value": 0, "keyframes": null },
        "skew": { "value": [0, 0], "keyframes": null },
        "skewAxis": { "value": 0, "keyframes": null }
      }
    }
  ]
}
```

Visual description:

- Animate one `null` parent to move, rotate, or scale an entire group.
- Children keep their local offsets while following the parent motion.

## Practical Examples

### 1) Center a layer

For a 1920x1080 composition, center position is `[960, 540]`.

```json
"transform": {
  "anchor": { "value": [200, 100], "keyframes": null },
  "position": { "value": [960, 540], "keyframes": null },
  "scale": { "value": [100, 100], "keyframes": null },
  "rotation": { "value": 0, "keyframes": null },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```

Visual description: layer center sits exactly in comp center.

### 2) Scale from center

Set anchor to layer center, then animate scale.

```json
"transform": {
  "anchor": { "value": [200, 100], "keyframes": null },
  "position": { "value": [960, 540], "keyframes": null },
  "scale": {
    "value": [100, 100],
    "keyframes": [
      { "time": 0.0, "value": [0, 0], "easing": "easeOutBack" },
      { "time": 0.6, "value": [100, 100] }
    ]
  },
  "rotation": { "value": 0, "keyframes": null },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```

Visual description: object grows outward from middle instead of stretching from a corner.

### 3) Rotate around custom anchor

Move anchor to left edge to create a hinge effect.

```json
"transform": {
  "anchor": { "value": [0, 100], "keyframes": null },
  "position": { "value": [960, 540], "keyframes": null },
  "scale": { "value": [100, 100], "keyframes": null },
  "rotation": {
    "value": 0,
    "keyframes": [
      { "time": 0.0, "value": -45, "easing": "easeOutCubic" },
      { "time": 1.0, "value": 0 }
    ]
  },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```

Visual description: layer swings like a door and settles.

### 4) Create parallax with position

Animate background and foreground at different speeds.

```json
{
  "backgroundTransform": {
    "anchor": { "value": [0, 0], "keyframes": null },
    "position": {
      "value": [0, 0],
      "keyframes": [
        { "time": 0, "value": [0, 0] },
        { "time": 8, "value": [-120, 0], "easing": "linear" }
      ]
    },
    "scale": { "value": [105, 105], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "foregroundTransform": {
    "anchor": { "value": [0, 0], "keyframes": null },
    "position": {
      "value": [0, 0],
      "keyframes": [
        { "time": 0, "value": [0, 0] },
        { "time": 8, "value": [-360, 0], "easing": "linear" }
      ]
    },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  }
}
```

Visual description: near elements move faster than distant elements, creating depth.

### 5) Flip horizontally and vertically

```json
{
  "flipHorizontal": {
    "scale": { "value": [-100, 100], "keyframes": null }
  },
  "flipVertical": {
    "scale": { "value": [100, -100], "keyframes": null }
  }
}
```

Visual description: mirrored orientation around current anchor.

### 6) Skew for perspective-like effects

Use `skew` with `skewAxis` for controlled directional shear.

```json
"transform": {
  "anchor": { "value": [200, 100], "keyframes": null },
  "position": { "value": [960, 540], "keyframes": null },
  "scale": { "value": [100, 100], "keyframes": null },
  "rotation": { "value": 0, "keyframes": null },
  "skew": {
    "value": [0, 0],
    "keyframes": [
      { "time": 0.0, "value": [0, 0] },
      { "time": 0.5, "value": [18, 0], "easing": "easeOut" },
      { "time": 1.0, "value": [0, 0], "easing": "easeIn" }
    ]
  },
  "skewAxis": { "value": 90, "keyframes": null }
}
```

Visual description: shape leans in a directional way, useful for faux perspective and stylized motion.

## Cautions and Best Practices

- Transform order is critical: `anchor -> position -> scale -> rotation -> skew`.
- Negative scale is powerful for flips but may produce unexpected behavior with some effects.
- Skew without a meaningful `skewAxis` often gives unintuitive results.
- Very large scale values can introduce visible pixelation, especially raster media.
- Keep anchor values intentional; many "wrong pivot" bugs are anchor issues, not rotation bugs.
- Prefer parent controllers (`null` layers) when animating groups to keep timelines clean.

## Quick Copy Template

```json
"transform": {
  "anchor": { "value": [0, 0], "keyframes": null },
  "position": { "value": [960, 540], "keyframes": null },
  "scale": { "value": [100, 100], "keyframes": null },
  "rotation": { "value": 0, "keyframes": null },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```
