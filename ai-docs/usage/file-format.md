# File Format Specification

Complete specification for `.outocut` file format.

## Overview

- **Extension**: `.outocut`
- **Format**: JSON (UTF-8)
- **Comment Support**: `//` line comments, `/* */` block comments
- **Minifiable**: Yes (remove comments)

## Structure

```json
{
  "version": "1.0",
  "metadata": { ... },
  "settings": { ... },
  "assets": [ ... ],
  "compositions": { ... },
  "mainCompositionId": "comp_main",
  "exportPresets": [ ... ]
}
```

## Complete Example

```json
{
  "version": "1.0",
  "metadata": {
    "name": "my_project",
    "created": "2026-03-17T22:40:00Z",
    "author": "John Doe",
    "description": "A sample project",
    "tags": ["intro", "animation"]
  },
  "settings": {
    "width": 1920,
    "height": 1080,
    "fps": 30,
    "duration": 60.0,
    "backgroundColor": "#000000",
    "pixelAspect": 1.0,
    "sampleRate": 48000,
    "audioChannels": 2
  },
  "assets": [
    {
      "id": "vid_001",
      "type": "video",
      "path": "./footage/intro.mp4",
      "trimStart": 0,
      "trimEnd": 15.3
    },
    {
      "id": "img_001",
      "type": "image",
      "path": "./images/logo.png"
    },
    {
      "id": "font_001",
      "type": "font",
      "path": "./fonts/Pretendard-Black.otf"
    },
    {
      "id": "audio_001",
      "type": "audio",
      "path": "./music/track.wav"
    }
  ],
  "compositions": {
    "main": {
      "id": "comp_main",
      "duration": 60.0,
      "width": 1920,
      "height": 1080,
      "layers": [ ... ]
    }
  },
  "mainCompositionId": "main",
  "exportPresets": [
    {
      "name": "youtube",
      "codec": "h264",
      "crf": 18,
      "preset": "slow"
    }
  ]
}
```

## Field Reference

### version

```json
"version": "1.0"
```

Type: `string`

File format version. Currently `1.0`.

---

### metadata

```json
"metadata": {
  "name": "project_name",
  "created": "2026-03-17T22:40:00Z",
  "author": "John Doe",
  "description": "Description",
  "tags": ["tag1", "tag2"]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | string | Yes | Project name |
| created | ISO8601 datetime | Yes | Creation timestamp |
| author | string | No | Author name |
| description | string | No | Project description |
| tags | string[] | No | Searchable tags |

---

### settings

```json
"settings": {
  "width": 1920,
  "height": 1080,
  "fps": 30,
  "duration": 60.0,
  "backgroundColor": "#000000",
  "pixelAspect": 1.0,
  "sampleRate": 48000,
  "audioChannels": 2
}
```

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| width | u32 | Yes | - | Frame width in pixels |
| height | u32 | Yes | - | Frame height in pixels |
| fps | f64 | Yes | - | Frames per second |
| duration | f64 | Yes | - | Duration in seconds |
| backgroundColor | string | Yes | "#000000" | Background color (hex) |
| pixelAspect | f64 | Yes | 1.0 | Pixel aspect ratio |
| sampleRate | u32 | Yes | 48000 | Audio sample rate |
| audioChannels | u32 | Yes | 2 | Number of audio channels |

---

### assets

```json
"assets": [
  {
    "id": "asset_001",
    "type": "video",
    "path": "./path/to/file.mp4",
    "trimStart": 0,
    "trimEnd": 10.0
  }
]
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Unique identifier |
| type | string | Yes | video, audio, image, font |
| path | string | Yes | Relative or absolute path |
| trimStart | f64 | No | Trim start time (seconds) |
| trimEnd | f64 | No | Trim end time (seconds) |

---

### compositions

```json
"compositions": {
  "comp_name": {
    "id": "comp_001",
    "duration": 30.0,
    "width": 1920,
    "height": 1080,
    "layers": [ ... ]
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier |
| duration | f64 | Composition duration |
| width | u32 | Width |
| height | u32 | Height |
| layers | Layer[] | Array of layers |

---

### Layer

```json
{
  "id": "layer_001",
  "type": "text",
  "name": "Main Title",
  "enabled": true,
  "startTime": 5.0,
  "duration": 10.0,
  "inPoint": 0,
  "outPoint": 10.0,
  "parentId": null,
  "trackMatte": "alpha",
  "blendMode": "normal",
  "opacity": { "value": 100, "keyframes": null },
  "transform": { ... },
  "content": { ... },
  "shapeContents": [ ... ],
  "effects": [ ... ],
  "masks": [ ... ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier |
| type | string | Layer type |
| name | string | Display name |
| enabled | bool | Visibility |
| startTime | f64 | Start time on timeline |
| duration | f64 | Layer duration |
| inPoint | f64 | In point (trim start) |
| outPoint | f64 | Out point (trim end) |
| parentId | string | Parent layer ID |
| trackMatte | string | Track matte type |
| blendMode | string | Blend mode |
| opacity | AnimatedProperty | Opacity animation |
| transform | Transform | Transform properties |
| content | object | Type-specific content |
| shapeContents | object[] | Shape definitions |
| effects | object[] | Effect stack |
| masks | object[] | Mask definitions |

---

### AnimatedProperty

```json
{
  "value": 100,
  "keyframes": [
    {
      "time": 0,
      "value": 0,
      "easing": "easeOutCubic"
    },
    {
      "time": 1,
      "value": 100
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| value | any | Default/static value |
| keyframes | Keyframe[] | Animated keyframes |

---

### Keyframe

```json
{
  "time": 1.0,
  "value": 100,
  "easing": "easeInOutCubic"
}
```

| Field | Type | Description |
|-------|------|-------------|
| time | f64 | Time position |
| value | any | Value at this time |
| easing | string | Easing function |

---

### Transform

```json
{
  "anchor": { "value": [960, 540], "keyframes": null },
  "position": { "value": [960, 540], "keyframes": [...] },
  "scale": { "value": [100, 100], "keyframes": null },
  "rotation": { "value": 0, "keyframes": null },
  "skew": { "value": [0, 0], "keyframes": null },
  "skewAxis": { "value": 0, "keyframes": null }
}
```

| Field | Type | Description |
|-------|------|-------------|
| anchor | AnimatedProperty<Vec<f64>> | Anchor point [x, y] |
| position | AnimatedProperty<Vec<f64>> | Position [x, y] |
| scale | AnimatedProperty<Vec<f64>> | Scale [x%, y%] |
| rotation | AnimatedProperty<f64> | Rotation (degrees) |
| skew | AnimatedProperty<Vec<f64>> | Skew [x, y] |
| skewAxis | AnimatedProperty<f64> | Skew axis (degrees) |

---

## Layer Types

| Type | Description | Content Type |
|------|-------------|--------------|
| video | Video footage | VideoContent |
| audio | Audio track | AudioContent |
| image | Static image | ImageContent |
| text | Text layer | TextContent |
| shape | Vector shapes | (via shapeContents) |
| solid | Color solid | SolidContent |
| null | Parent layer | null |
| adjustment | Effect layer | null |
| composition | Pre-comp | CompositionContent |

---

## Easing Functions

| Name | Description |
|------|-------------|
| linear | No easing |
| easeIn | Quadratic ease in |
| easeOut | Quadratic ease out |
| easeInOut | Quadratic ease in-out |
| easeInCubic | Cubic ease in |
| easeOutCubic | Cubic ease out |
| easeInOutCubic | Cubic ease in-out |
| easeInBack | Back ease in |
| easeOutBack | Back ease out |
| easeInOutBack | Back ease in-out |
| easeInElastic | Elastic ease in |
| easeOutElastic | Elastic ease out |
| easeInOutElastic | Elastic ease in-out |
| easeInBounce | Bounce ease in |
| easeOutBounce | Bounce ease out |
| easeInOutBounce | Bounce ease in-out |
| cubicBezier | Custom bezier curve |

---

## Blend Modes

| Mode | Description |
|------|-------------|
| normal | Normal blending |
| multiply | Multiply |
| screen | Screen |
| overlay | Overlay |
| darken | Darken |
| lighten | Lighten |
| colorDodge | Color dodge |
| colorBurn | Color burn |
| hardLight | Hard light |
| softLight | Soft light |
| difference | Difference |
| exclusion | Exclusion |
| hue | Hue |
| saturation | Saturation |
| color | Color |
| luminosity | Luminosity |
| add | Add (linear dodge) |
| subtract | Subtract |
| divide | Divide |

---

## Track Matte

| Mode | Description |
|------|-------------|
| alpha | Alpha channel |
| alphaInverted | Inverted alpha |
| luma | Luminance |
| lumaInverted | Inverted luminance |
