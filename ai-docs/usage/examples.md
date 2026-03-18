# Usage Examples

Practical examples for common tasks.

## Basic Examples

### Creating a Simple Project

```json
{
  "version": "1.0",
  "metadata": {
    "name": "hello_world",
    "created": "2026-03-18T00:00:00Z",
    "author": "you",
    "description": "My first project",
    "tags": []
  },
  "settings": {
    "width": 1920,
    "height": 1080,
    "fps": 30,
    "duration": 10.0,
    "backgroundColor": "#000000",
    "pixelAspect": 1.0,
    "sampleRate": 48000,
    "audioChannels": 2
  },
  "assets": [],
  "compositions": {
    "main": {
      "id": "comp_main",
      "duration": 10.0,
      "width": 1920,
      "height": 1080,
      "layers": []
    }
  },
  "mainCompositionId": "main",
  "exportPresets": []
}
```

### Rendering

```bash
# Quick render
outocut render project.outocut -o video.mp4

# High quality
outocut render project.outocut -o video.mp4 --preset veryslow --crf 15
```

---

## Layer Examples

### Solid Color Layer

```json
{
  "id": "bg_layer",
  "type": "solid",
  "name": "Background",
  "enabled": true,
  "startTime": 0,
  "duration": 10.0,
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
    "color": "#FF0000"
  }
}
```

### Text Layer

```json
{
  "id": "title_layer",
  "type": "text",
  "name": "Title",
  "enabled": true,
  "startTime": 0,
  "duration": 5.0,
  "opacity": { "value": 100, "keyframes": null },
  "transform": {
    "anchor": { "value": [960, 200], "keyframes": null },
    "position": { "value": [960, 200], "keyframes": null },
    "scale": { "value": [100, 100], "keyframes": null },
    "rotation": { "value": 0, "keyframes": null },
    "skew": { "value": [0, 0], "keyframes": null },
    "skewAxis": { "value": 0, "keyframes": null }
  },
  "content": {
    "text": "OUTOCUT",
    "fontSize": 120,
    "color": "#FFFFFF",
    "alignment": "center"
  }
}
```

---

## Animation Examples

### Fade In

```json
{
  "opacity": {
    "value": 0,
    "keyframes": [
      { "time": 0, "value": 0, "easing": "linear" },
      { "time": 1, "value": 100, "easing": "easeOutCubic" }
    ]
  }
}
```

### Position Move

```json
{
  "transform": {
    "position": {
      "value": [960, 700],
      "keyframes": [
        { "time": 0, "value": [960, 700], "easing": "easeOutCubic" },
        { "time": 3, "value": [960, 540] }
      ]
    }
  }
}
```

### Scale Bounce

```json
{
  "transform": {
    "scale": {
      "value": [0, 0],
      "keyframes": [
        { "time": 0, "value": [0, 0], "easing": "easeOutBack" },
        { "time": 0.5, "value": [110, 110], "easing": "easeInOutCubic" },
        { "time": 1, "value": [100, 100] }
      ]
    }
  }
}
```

### Rotation

```json
{
  "transform": {
    "rotation": {
      "value": 0,
      "keyframes": [
        { "time": 0, "value": 0 },
        { "time": 5, "value": 360 }
      ]
    }
  }
}
```

---

## Effects Examples

### Drop Shadow

```json
{
  "effects": [
    {
      "id": "shadow_1",
      "type": "dropShadow",
      "enabled": true,
      "params": {
        "distance": 10,
        "angle": 45,
        "blur": 20,
        "color": "#000000",
        "opacity": 50
      },
      "keyframes": null
    }
  ]
}
```

### Glow

```json
{
  "effects": [
    {
      "id": "glow_1",
      "type": "glow",
      "enabled": true,
      "params": {
        "radius": 30,
        "color": "#FF8800",
        "opacity": 80
      },
      "keyframes": null
    }
  ]
}
```

---

## Blend Mode Examples

### Screen Blend

```json
{
  "blendMode": "screen"
}
```

### Add Blend

```json
{
  "blendMode": "add"
}
```

---

## Composition Examples

### Basic Composition

```json
{
  "compositions": {
    "main": {
      "id": "comp_main",
      "duration": 60.0,
      "width": 1920,
      "height": 1080,
      "layers": [
        { /* layer 1 */ },
        { /* layer 2 */ }
      ]
    }
  },
  "mainCompositionId": "main"
}
```

### Nested Composition

```json
{
  "compositions": {
    "main": {
      "id": "comp_main",
      "duration": 30.0,
      "width": 1920,
      "height": 1080,
      "layers": [
        {
          "id": "precomp_layer",
          "type": "composition",
          "content": {
            "compositionId": "title_anim"
          }
        }
      ]
    },
    "title_anim": {
      "id": "comp_title_anim",
      "duration": 5.0,
      "width": 1920,
      "height": 1080,
      "layers": [
        { /* title layers */ }
      ]
    }
  }
}
```

---

## Complete Project Example

```json
{
  "version": "1.0",
  "metadata": {
    "name": "animated_intro",
    "created": "2026-03-18T00:00:00Z",
    "author": "you",
    "description": "Animated intro sequence",
    "tags": ["intro", "animation"]
  },
  "settings": {
    "width": 1920,
    "height": 1080,
    "fps": 30,
    "duration": 5.0,
    "backgroundColor": "#000000",
    "pixelAspect": 1.0,
    "sampleRate": 48000,
    "audioChannels": 2
  },
  "assets": [],
  "compositions": {
    "main": {
      "id": "comp_main",
      "duration": 5.0,
      "width": 1920,
      "height": 1080,
      "layers": [
        {
          "id": "bg",
          "type": "solid",
          "name": "Background",
          "enabled": true,
          "startTime": 0,
          "duration": 5.0,
          "opacity": { "value": 100, "keyframes": null },
          "transform": {
            "anchor": { "value": [960, 540], "keyframes": null },
            "position": { "value": [960, 540], "keyframes": null },
            "scale": { "value": [100, 100], "keyframes": null },
            "rotation": { "value": 0, "keyframes": null },
            "skew": { "value": [0, 0], "keyframes": null },
            "skewAxis": { "value": 0, "keyframes": null }
          },
          "content": { "color": "#1a1a2e" }
        },
        {
          "id": "title",
          "type": "text",
          "name": "Title",
          "enabled": true,
          "startTime": 0,
          "duration": 5.0,
          "opacity": {
            "value": 0,
            "keyframes": [
              { "time": 0.5, "value": 0, "easing": "linear" },
              { "time": 1.5, "value": 100, "easing": "easeOutCubic" }
            ]
          },
          "transform": {
            "anchor": { "value": [960, 540], "keyframes": null },
            "position": {
              "value": [960, 540],
              "keyframes": [
                { "time": 0, "value": [960, 700], "easing": "easeOutBack" },
                { "time": 1.5, "value": [960, 540] }
              ]
            },
            "scale": {
              "value": [0, 0],
              "keyframes": [
                { "time": 0.5, "value": [0, 0], "easing": "easeOutBack" },
                { "time": 1.5, "value": [100, 100] }
              ]
            },
            "rotation": { "value": 0, "keyframes": null },
            "skew": { "value": [0, 0], "keyframes": null },
            "skewAxis": { "value": 0, "keyframes": null }
          },
          "content": {
            "text": "HELLO WORLD",
            "fontSize": 150,
            "color": "#ffffff",
            "alignment": "center"
          },
          "effects": [
            {
              "id": "glow",
              "type": "glow",
              "enabled": true,
              "params": {
                "radius": 20,
                "color": "#00aaff",
                "opacity": 60
              },
              "keyframes": null
            }
          ]
        }
      ]
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

---

## Scripting Examples

### Generate Project with Python

```python
import json
import uuid

def create_project(name, duration):
    return {
        "version": "1.0",
        "metadata": {
            "name": name,
            "created": "2026-03-18T00:00:00Z",
            "author": "script",
            "description": "Generated project",
            "tags": []
        },
        "settings": {
            "width": 1920,
            "height": 1080,
            "fps": 30,
            "duration": duration,
            "backgroundColor": "#000000",
            "pixelAspect": 1.0,
            "sampleRate": 48000,
            "audioChannels": 2
        },
        "assets": [],
        "compositions": {
            "main": {
                "id": "comp_main",
                "duration": duration,
                "width": 1920,
                "height": 1080,
                "layers": []
            }
        },
        "mainCompositionId": "main",
        "exportPresets": []
    }

project = create_project("my_project", 30.0)
with open("project.outocut", "w") as f:
    json.dump(project, f, indent=2)
```

### Batch Render

```bash
#!/bin/bash
for project in projects/*.outocut; do
    name=$(basename "$project" .outocut)
    outocut render "$project" -o "output/${name}.mp4"
done
```
