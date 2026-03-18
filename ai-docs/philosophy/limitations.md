# Known Limitations

Current limitations and known issues.

## Technical Limitations

### Rendering

1. **Software Rendering Only**
   - No GPU acceleration in v1.0
   - CPU-based compositing
   - Slow for complex projects

2. **Frame-by-Frame**
   - No real-time preview
   - Must render to see results
   - Slow iteration

3. **PNG Sequence Caching**
   - High disk space usage
   - 1 frame ≈ 8MB at 1080p
   - No compression

### File Format

4. **JSON Performance**
   - Large projects = large files
   - Slower parse times
   - Consider for 10+ minute projects

5. **No Binary Format**
   - Human-readable but larger
   - Not optimal for massive assets
   - Git history grows

### Effects

6. **Limited Effects**
   - ~30 effects in v1.0
   - Missing many After Effects features
   - Will expand over time

7. **No Advanced Effects**
   - No motion tracking
   - No warping
   - No 3D

### Animation

8. **No Expressions (v1.0)**
   - Only keyframes
   - Procedural animation limited
   - Coming in v1.1

9. **No Per-Character Animation**
   - Text animators not implemented
   - Only whole-layer animation

### Audio

10. **Basic Audio**
    - No effects on audio
    - No keyframe volume
    - Simple playback

## Missing Features

### From After Effects

- 3D layers
- Camera layers
- Light layers
- Motion tracking
- Warp stabilizers
- Time remapping
- Scripting
- Expressions (planned)

### From Video Editors

- Multi-camera editing
- Proxy workflows
- Audio mixing
- Audio effects
- Titler functionality

## Platform Limitations

### Linux

- Less tested
- FFmpeg version dependent

### Windows

- No GPU encoding in all cases
- Path handling differences

### macOS

- VideoToolbox not integrated
- Metal rendering not available

## Performance Issues

### Large Projects

May be slow with:
- 50+ layers
- 4K+ resolution
- 60fps frame rate
- Complex effects stacks
- Long durations (30+ min)

### Memory Usage

High memory with:
- Many compositions
- Large images
- Uncached rendering

## Future Improvements

These limitations will be addressed:

| Limitation | Planned Version |
|------------|-----------------|
| Expressions | v1.1 |
| GPU rendering | v1.2 |
| Real-time preview | v1.2 |
| Audio effects | v1.3 |
| Motion tracking | v2.0 |

## Workarounds

### For Slow Rendering

1. **Lower resolution preview**
   ```bash
   # Create smaller version for testing
   ```

2. **Fewer layers**
   - Pre-compose when possible
   - Use adjustment layers

3. **Simpler effects**
   - Disable effects for preview
   - Render in passes

### For Large Files

1. **Minify JSON**
   ```bash
   outocut export-json project.outocut
   # Not --pretty
   ```

2. **External assets**
   - Keep images/videos outside
   - Reference by path

### For Missing Features

1. **Pre-render in other tools**
   - Use AE/Blender for complex effects
   - Import as video layer

2. **Script generation**
   - Generate OutoCut JSON from other tools
   - Python/Rust scripts

## Reporting Issues

Found something else? Open an issue:

```bash
# Check existing issues
# Provide:
# - Project file (if possible)
# - Command used
# - Error message
# - Expected behavior
```
