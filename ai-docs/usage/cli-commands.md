# CLI Commands

Complete reference for all CLI commands.

## Commands Overview

| Command | Description |
|---------|-------------|
| [render](#render) | Render project to video |
| [preview](#preview) | Preview at specific time |
| [validate](#validate) | Validate project file |
| [export-json](#export-json) | Export JSON |
| [watch](#watch) | Watch for changes |

## render

Render project to video file.

```bash
outocut render <PROJECT> -o <OUTPUT> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| PROJECT | Path to .outocut project file |

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| -o, --output | path | - | Output file path (required) |
| --gpu | flag | false | Use GPU acceleration |
| --preset | string | medium | Encoding preset |
| --crf | number | 23 | CRF value (0-51) |

### Presets

| Preset | Description |
|--------|-------------|
| ultrafast | Fastest, lowest quality |
| superfast | Very fast |
| veryfast | Fast |
| faster | Fast |
| fast | Moderately fast |
| medium | Default |
| slow | Slower, better quality |
| slower | Very slow |
| veryslow | Slowest, best quality |

### CRF Scale

- 0: Lossless
- 18: Visually lossless
- 23: Default (good quality)
- 28: Lower quality
- 51: Lowest quality

### Examples

```bash
# Basic render
outocut render project.outocut -o output.mp4

# High quality render
outocut render project.outocut -o output.mp4 --preset slow --crf 18

# GPU acceleration
outocut render project.outocut -o output.mp4 --gpu

# Custom output format
outocut render project.outocut -o output.mov
outocut render project.outocut -o output.webm
```

---

## preview

Preview project at specific time.

```bash
outocut preview <PROJECT> --time <TIME> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| PROJECT | Path to .outocut project file |

### Options

| Option | Type | Description |
|--------|------|-------------|
| --time | f64 | Start time in seconds (required) |
| --duration | f64 | Duration in seconds |

### Examples

```bash
# Preview at 5 seconds
outocut preview project.outocut --time 5.0

# Preview 3 seconds starting at 5
outocut preview project.outocut --time 5.0 --duration 3
```

### Output

Prints information about active layers at the specified time:
- Layer name
- Layer type
- Position
- Scale
- Opacity

---

## validate

Validate project file.

```bash
outocut validate <PROJECT>
```

### Arguments

| Argument | Description |
|----------|-------------|
| PROJECT | Path to .outocut project file |

### Validation Checks

- Dimensions > 0
- FPS > 0
- Duration > 0
- Main composition exists
- Main composition duration matches

### Examples

```bash
# Validate project
outocut validate project.outocut

# Use in scripts
if outocut validate project.outocut; then
    echo "Valid!"
fi
```

---

## export-json

Export project JSON.

```bash
outocut export-json <PROJECT> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| PROJECT | Path to .outocut project file |

### Options

| Option | Type | Description |
|--------|------|-------------|
| -p, --pretty | flag | Pretty print JSON |

### Examples

```bash
# Export minified
outocut export-json project.outocut > output.json

# Export pretty printed
outocut export-json project.outocut --pretty > output.json
```

---

## watch

Watch for file changes and auto-reload.

```bash
outocut watch <PROJECT>
```

### Arguments

| Argument | Description |
|----------|-------------|
| PROJECT | Path to .outocut project file |

### Behavior

1. Watch the project file for changes
2. On modification, validate the project
3. Print validation result

### Examples

```bash
# Watch project
outocut watch project.outocut
```

### Exit

Press Ctrl+C to exit watch mode.

---

## Global Options

| Option | Description |
|--------|-------------|
| -V, --version | Print version |
| -h, --help | Print help |

### Examples

```bash
# Print version
outocut --version

# Print help
outocut --help

# Print specific command help
outocut render --help
```

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| RUST_LOG | Logging level (debug, info, warn, error) |

### Examples

```bash
# Enable debug logging
RUST_LOG=debug outocut validate project.outocut
```

---

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | Error (file not found, invalid project, render failed) |
