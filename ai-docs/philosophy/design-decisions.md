# Design Decisions

Architecture and design decisions with rationale.

## Language Choice: Rust

### Decision

Written entirely in Rust.

### Rationale

**Performance**: 
- Near-native speed for video processing
- No garbage collection pauses
- Efficient memory usage

**Safety**:
- Memory safety without garbage collection
- Thread safety for parallel rendering
- Compile-time error catching

**Tooling**:
- Excellent package ecosystem (crates.io)
- Fast compilation
- Great IDE support

### Tradeoffs

- **Learning curve**: Steeper than Python/JS
- **GUI development**: Less mature than other languages
- **FFI**: Some complexity wrapping C libraries

## JSON for Project Files

### Decision

`.outocut` files are pure JSON with comment support.

### Rationale

**AI-friendliness**:
- Easy to generate programmatically
- Simple to parse and transform
- No custom parser needed

**Git compatibility**:
- Text format = meaningful diffs
- Merge conflicts resolvable
- Line-by-line history

**Tooling**:
- Every language has JSON libraries
- Schema validation available
- Editor support ubiquitous

### Alternative Considered

**Binary format (MessagePack, Protocol Buffers)**:
- ❌ Not human-readable
- ❌ Harder to generate
- ❌ Git diffs useless

**Custom text format**:
- ❌ Need custom parser
- ❌ Less tooling support

### Comment Support

JSON doesn't support comments natively. We strip comments before parsing:

```json
{
  // This is a comment
  "key": "value",  // Inline comment
  /* Multi-line
     comment */
}
```

## FFmpeg for Video Encoding

### Decision

Use FFmpeg via subprocess for final video encoding.

### Rationale

**Industry standard**:
- Best-in-class codecs
- Extensive format support
- Well-tested and maintained

**Focus**:
- OutoCut focuses on composition/animation
- Encoding is a solved problem
- No need to reinvent

### Alternative Considered

**Rust-native encoding (VideoCodec, etc.)**:
- ❌ Less codec support
- ❌ Less optimization
- ❌ More maintenance burden

**Custom FFmpeg bindings**:
- ❌ Complex FFI
- ❌ Version compatibility issues

## Single Binary Distribution

### Decision

Single `outocut` binary with no external runtime.

### Rationale

**Simplicity**:
- One file to install
- No dependency management
- Easy CI/CD

**Portability**:
- Works anywhere Rust compiles
- No system library conflicts

### Tradeoffs

- **Binary size**: Larger than dynamic linking
- **Update frequency**: Must recompile for updates

## Easing Functions Implementation

### Decision

Implement 20+ easing functions as enum variants.

### Rationale

**Type safety**:
- Compile-time validation
- No runtime parsing errors
- IDE autocomplete

**Performance**:
- Match arm = fast dispatch
- No string comparison
- Inline potential

### Implementation

```rust
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInCubic,
    // ... 20+ variants
    CubicBezier(f64, f64, f64, f64),  // Parametric
}
```

## No GUI (CLI-First)

### Decision

CLI-only for v1.0, GUI planned for future.

### Rationale

**AI focus**:
- AI agents use CLI easily
- No GUI automation complexity
- Easy integration with scripts

**Complexity management**:
- Focus on core engine
- Incremental development
- User-provided GUIs possible

### Future

GUI will wrap CLI, not replace it:
- CLI remains primary interface
- GUI calls CLI commands
- Same core functionality

## Error Handling: anyhow

### Decision

Use `anyhow` for application error handling.

### Rationale

**Simplicity**:
- Easy error propagation
- No boilerplate
- Context preservation

**Compatibility**:
- Works with any error type
- Integrates with std::error::Error

### Alternative Considered

**thiserror for each module**:
- More type safety
- More boilerplate
- Overkill for CLI app

## Frame-by-Frame Rendering

### Decision

Render each frame independently, then encode.

### Rationale

**Flexibility**:
- Easy debugging (inspect frames)
- Frame caching possible
- Parallel frame rendering

**Simplicity**:
- No complex streaming architecture
- Easy to understand
- Straightforward caching

### Tradeoffs

**Memory**:
- More memory for frame buffer
- Could stream for lower memory

**Speed**:
- Could be faster with pipelining
- Acceptable for v1.0

## No Plugin System (v1.0)

### Decision

No runtime plugin support in v1.0.

### Rationale

**Simplicity**:
- Focus on core features
- No plugin API complexity
- Stable API

**Future**:
- Can add later
- Learn requirements from usage

## Blend Mode Implementation

### Decision

Implement blend modes in Rust, not via FFmpeg filters.

### Rationale

**Control**:
- Full control over composition
- Per-pixel operations
- Debugging easier

**Flexibility**:
- Works with any layer type
- No FFmpeg filter complexity

### Tradeoffs

**Performance**:
- Slower than GPU shaders
- Acceptable for software rendering

## Caching Strategy

### Decision

Cache rendered frames as PNG sequence.

### Rationale

**Simplicity**:
- Easy to implement
- FFmpeg understands PNG sequences
- Debugging easy

**Tradeoffs**

**Disk space**:
- Can use significant space
- Could compress better
- Acceptable for v1.0

## Future Design Questions

These decisions may evolve:

1. **Expression language**: JavaScript or custom?
2. **GUI framework**:egui, Iced, or GTK?
3. **Plugin system**: Wasm, Lua, or Rust modules?
4. **Distributed rendering**: Grid/cloud support?
5. **Real-time preview**: WebSocket server?
