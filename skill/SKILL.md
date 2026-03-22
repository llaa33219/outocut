---
name: outocut
description: OutOcut video editor - CLI commands, JSON project format (.outocut), animation system, effects, blend modes, and AI visual self-verification workflow
metadata:
  tags: outocut, video, motion-graphics, animation, cli, video-editing, composition
---

# OutOcut

## When to Use
This skill activates when working with:
- OutOcut CLI commands (render, preview, validate, watch, export-json)
- Creating or editing .outocut project files (JSON format)
- Video composition with layers (video, audio, text, shape, solid, image)
- Animation with keyframes and easing functions
- Applying effects (glow, blur, color correction, etc.)
- Blend modes and track matte compositing
- AI agents rendering and verifying video output visually

## Quick Reference

### CLI Commands
| Command | Description |
|---------|-------------|
| `outocut render <file> -o <output>` | Render project to video |
| `outocut preview <file> --time <seconds>` | Preview at specific time |
| `outocut validate <file>` | Validate project structure |
| `outocut watch <file>` | Watch mode for auto-reload |
| `outocut export-json <file>` | Export formatted JSON |

### Coordinate System
- Origin: Top-left (0,0)
- X increases rightward, Y increases downward
- Default canvas: 1920×1080

### File Structure
.project.json → settings, assets[], compositions{}, mainCompositionId

## Detailed Guides
See rules/ directory for comprehensive guides:
- [rules/file-format.md](rules/file-format.md) - Complete .outocut JSON format specification
- [rules/cli-commands.md](rules/cli-commands.md) - Full CLI reference
- [rules/ai-agent-verify.md](rules/ai-agent-verify.md) - AI visual self-verification protocol
- [rules/best-practices.md](rules/best-practices.md) - Production best practices
- [rules/coordinate-system.md](rules/coordinate-system.md) - Position and coordinate reference
- [rules/transforms.md](rules/transforms.md) - Transform properties
- [rules/effects.md](rules/effects.md) - Effect reference
- [rules/layer-types.md](rules/layer-types.md) - Layer type reference
- [rules/blend-modes.md](rules/blend-modes.md) - Blend mode reference
- [rules/shapes-masks.md](rules/shapes-masks.md) - Shape and mask data
- [rules/track-matte.md](rules/track-matte.md) - Track matte reference
- [rules/animation-system.md](rules/animation-system.md) - Animation system
- [rules/examples.md](rules/examples.md) - Usage examples
- [rules/review-guide.md](rules/review-guide.md) - Production review & QA checklist

## Core Concepts

### Everything is a Layer
Video, audio, text, shapes, images - all are layers with transform, opacity, and effects.

### Everything is Keyframable
Time, value, easing - all can be animated with keyframe interpolation.

### Deterministic Render
Same JSON = 100% identical output. No randomness during render.

### Expression Support (planned)
JavaScript-like expressions: `position.x = time * 50`

## Architecture
CLI (clap) → Parser → Models → Composition → Animation → Render → FFmpeg encode
