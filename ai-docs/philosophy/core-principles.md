# Core Principles

The fundamental principles that guide OutOcut development.

## Principle 1: Everything is a Layer

### Definition

Every visual or audio element in OutOcut is a **Layer**. This includes:
- Video footage
- Audio tracks
- Images
- Text
- Vector shapes
- Color solids
- Null objects (for parenting)
- Adjustment layers
- Compositions (pre-comps)

### Why Layers?

Layers provide:
- **Composability**: Stack and combine any elements
- **Organization**: Clear visual stacking order
- **Editability**: Non-destructive editing
- **Predictability**: Order-based rendering

### Implementation

```rust
pub struct Layer {
    pub id: String,
    pub layer_type: LayerType,
    pub transform: Transform,
    pub opacity: AnimatedProperty<f64>,
    // ... all layers share these properties
}
```

### Implications

1. All layers have transforms
2. All layers can be animated
3. All layers support effects
4. All layers can be masked

## Principle 2: Everything is Keyframable

### Definition

Any property that can change over time can have keyframes. This includes:
- Position (x, y)
- Scale (x, y)
- Rotation
- Opacity
- Effect parameters
- Mask properties
- Text content
- Shape properties

### Keyframe Structure

```rust
pub struct Keyframe {
    pub time: f64,
    pub value: serde_json::Value,
    pub easing: Option<Easing>,
}
```

### Why Keyframes?

- **Animation**: Essential for motion graphics
- **Interpolation**: Smooth transitions
- **Control**: Fine-grained timing
- **Expression-ready**: Foundation for expressions

### Implementation

The `AnimatedProperty<T>` type wraps any value:

```rust
pub struct AnimatedProperty<T> {
    pub value: T,                              // Static value
    pub keyframes: Option<Vec<Keyframe>>,     // Animated values
}
```

## Principle 3: Everything Supports Expression

### Definition

Properties can use JavaScript-like expressions:

```javascript
position.x = time * 50
opacity = Math.sin(time) * 100
scale[0] = 100 + Math.sin(time * 2) * 20
```

### Why Expressions?

- **Procedural animation**: Generate motion algorithmically
- **Mathematical relationships**: Link properties
- **AI-friendly**: Generate code, not just keyframes
- **Flexibility**: Beyond keyframe limitations

### Status

Expression support is planned for v1.1. The current keyframe system provides the foundation.

## Principle 4: Composition First

### Definition

Nested compositions (pre-comps) are first-class citizens:
- Any composition can be used as a layer
- Unlimited nesting depth
- Independent timelines
- Local/global time conversion

### Why Compositions?

- **Modularity**: Reusable components
- **Organization**: Break complex projects into pieces
- **Performance**: Cache pre-rendered content
- **Collaboration**: Work on components separately

### Implementation

```rust
pub enum LayerContent {
    Composition(CompositionContent),
    // ...
}

pub struct CompositionContent {
    pub composition_id: String,
}
```

## Principle 5: Deterministic Render

### Definition

The same project file always produces identical output:
- Same frame N looks exactly the same every time
- No randomness in rendering
- Git-friendly (deterministic diffs)
- Reproducible results

### Why Deterministic?

- **Version control**: Meaningful diffs
- **AI training**: Consistent results
- **Debugging**: Reproduce issues
- **Caching**: Safe frame caching

### Implementation

```rust
// No random number generation
// Frame-accurate timing
// Consistent interpolation

fn render_frame(project, time, ...) {
    // time = frame_number / fps (exact)
    // Same time = same result
}
```

### Git Integration

```bash
# Add project
git add project.outocut

# View diff
git diff project.outocut

# Meaningful changes every time
```

## Principle 6: Vector-First

### Definition

Motion graphics are vector-based:
- Resolution-independent shapes
- Text as vectors (not rasterized)
- Infinite scalability
- Cairo-based rendering

### Why Vectors?

- **Quality**: Never pixelated
- **Resolution**: Any output resolution
- **Performance**: Efficient recompositing
- **AI generation**: Easy to generate procedurally

### Implementation

```rust
// Shapes are defined as vectors
pub struct ShapeContent {
    pub shape_type: ShapeType,  // Rect, Ellipse, Path...
    pub size: Option<Vec<f64>>,
    pub position: Option<Vec<f64>>,
    // ...
}

// Rendered via Cairo
// Output resolution independent
```

## Summary Table

| Principle | Benefit | Implementation |
|-----------|---------|----------------|
| Everything is a Layer | Composability | `Layer` struct |
| Everything is Keyframable | Animation | `AnimatedProperty<T>` |
| Everything supports Expression | Procedural motion | Planned v1.1 |
| Composition First | Modularity | Nested `Composition` |
| Deterministic Render | Reproducibility | No randomness |
| Vector-First | Quality | Cairo rendering |

## These Principles Are Non-Negotiable

Breaking these principles requires significant justification:
- New features must respect all six principles
- Design decisions are evaluated against principles
- Tradeoffs are documented and justified
