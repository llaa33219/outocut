# Adding Features

Guide for adding new features to OutOcut.

## Overview

This guide covers how to extend OutOcut with:
- New layer types
- New effects
- New blend modes
- New easing functions
- New CLI commands

## Adding a New Layer Type

### Step 1: Add to LayerType Enum

In `src/models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerType {
    Video,
    Audio,
    Image,
    Text,
    Shape,
    Solid,
    Null,
    Adjustment,
    Composition,
    // Add new type here
    NewLayerType,
}
```

### Step 2: Define Content Type (if needed)

In `src/models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LayerContent {
    // Existing types...
    // Add new content type
    NewContent(NewContentData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewContentData {
    pub field1: String,
    pub field2: u32,
}
```

### Step 3: Implement Rendering

In `src/render.rs`, add to `render_layer` function:

```rust
fn render_layer(...) -> Result<()> {
    match &layer.content {
        // Existing cases...
        Some(LayerContent::NewContent(data)) => {
            Self::render_new_content(data, &mut layer_data, width, height)?;
        }
        _ => {}
    }
    Ok(())
}

fn render_new_content(
    data: &NewContentData,
    data_buffer: &mut [u8],
    width: u32,
    height: u32,
) -> Result<()> {
    // Implement rendering logic
    Ok(())
}
```

## Adding a New Effect

### Step 1: Add to EffectType Enum

In `src/models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EffectType {
    // Existing effects...
    DropShadow,
    // Add new effect
    NewEffect,
}
```

### Step 2: Define Effect Parameters

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEffectParams {
    pub param1: f64,
    pub param2: String,
}
```

### Step 3: Implement Effect

In `src/render.rs`, add effect application:

```rust
fn apply_effect(
    effect: &Effect,
    frame_data: &mut [u8],
    width: u32,
    height: u32,
) -> Result<()> {
    match effect.effect_type {
        // Existing effects...
        EffectType::NewEffect => {
            let params: NewEffectParams = 
                serde_json::from_value(effect.params.clone())?;
            // Apply effect
        }
    }
    Ok(())
}
```

## Adding a New Blend Mode

### Step 1: Add to BlendMode Enum

In `src/models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlendMode {
    // Existing modes...
    Normal,
    Multiply,
    Screen,
    // Add new mode
    NewBlendMode,
}
```

### Step 2: Implement Blend Calculation

In `src/composition.rs`:

```rust
fn apply_blend(&self, base: &[u8; 4], blend: &[u8; 4], mode: &BlendMode) -> [u8; 4] {
    // Convert to f64 (0.0 - 1.0)
    let b = [base[0] as f64 / 255.0, /* ... */];
    let f = [blend[0] as f64 / 255.0, /* ... */];

    let result: [f64; 4] = match mode {
        // Existing modes...
        BlendMode::NewBlendMode => [
            // Custom blend formula
            b[0].min(f[0]),
            b[1].min(f[1]),
            b[2].min(f[2]),
            f[3],
        ],
    };

    // Convert back to u8
    [
        (result[0] * 255.0) as u8,
        // ...
    ]
}
```

## Adding a New Easing Function

### Step 1: Add to Easing Enum

In `src/models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Easing {
    // Existing easings...
    Linear,
    EaseIn,
    // Add new easing
    CustomEasing {
        param1: f64,
        param2: f64,
    },
}
```

### Step 2: Implement Easing Function

In `src/animation.rs`:

```rust
fn apply_easing(t: f64, easing: &Option<Easing>) -> f64 {
    match easing {
        // Existing easings...
        Some(Easing::CustomEasing { param1, param2 }) => {
            // Custom easing formula
            let c1 = *param1;
            let c3 = c1 + 1.0;
            c3 * t * t * t - c1 * t * t
        }
        // Fallback
        _ => t,
    }
}
```

## Adding a New CLI Command

### Step 1: Add Command Variant

In `src/cli.rs`:

```rust
#[derive(Subcommand)]
pub enum Commands {
    // Existing commands...
    
    #[command(about = "Description of new command")]
    NewCommand {
        #[arg(help = "Path to project file")]
        project: PathBuf,
        
        #[arg(short, long, help = "Optional parameter")]
        option: Option<String>,
    },
}
```

### Step 2: Implement Handler

In appropriate module (or new file):

```rust
pub async fn handle_new_command(
    project: &Path,
    option: Option<String>,
) -> Result<()> {
    // Implementation
    Ok(())
}
```

### Step 3: Wire to Main

In `src/main.rs`:

```rust
match args.command {
    Commands::NewCommand { project, option } => {
        handle_new_command(&project, option).await?;
    }
    // Existing commands...
}
```

## Testing New Features

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blend_mode() {
        // Test blend calculation
    }

    #[test]
    fn test_new_easing() {
        // Test easing function
    }
}
```

### Integration Tests

```rust
// tests/new_feature_test.rs
#[test]
fn test_new_layer_render() {
    // Test full render pipeline with new layer
}
```

## Documentation Updates

When adding new features, update:

1. **README.md** - Quick reference
2. **ai-docs/usage/** - Usage documentation
3. **ai-docs/architecture/** - If architecture changed
4. **Code comments** - Implementation details

## Checklist

- [ ] Add enum variant
- [ ] Implement core logic
- [ ] Add unit tests
- [ ] Add integration test
- [ ] Update documentation
- [ ] Run full test suite
- [ ] Verify build succeeds
