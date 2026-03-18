# AI Documentation Update Process

Guide for maintaining AI documentation when code changes.

## Overview

When code changes, corresponding documentation in `ai-docs/` must be updated. This ensures AI agents can accurately understand and modify the codebase.

## Why This Matters

AI agents rely on documentation to:
- Understand architecture
- Make correct changes
- Avoid breaking existing functionality
- Follow project conventions

## Update Triggers

Update ai-docs when changing:

1. **New modules** - Add module documentation
2. **New features** - Add feature documentation
3. **API changes** - Update CLI reference
4. **Design decisions** - Document rationale
5. **Bug fixes** - Update known issues if relevant

## Documentation Mapping

| Code Change | Documentation to Update |
|-------------|------------------------|
| New CLI command | `usage/cli-commands.md` |
| New module | `architecture/modules.md` |
| New layer type | `usage/file-format.md` |
| New effect | `usage/file-format.md` |
| New easing | `usage/file-format.md` |
| Design change | `philosophy/design-decisions.md` |
| Build process | `development/setup.md` |
| Bug fix | `philosophy/limitations.md` (if affects users) |

## Update Process

### Step 1: Identify Scope

Determine what documentation needs updates:

```bash
# Check what changed
git diff --name-only

# Identify affected areas
```

### Step 2: Make Code Changes

```bash
# Make your code changes
# Test thoroughly
cargo test
```

### Step 3: Update Documentation

Update each affected file:

```bash
# Edit relevant ai-docs files
vim ai-docs/architecture/modules.md
vim ai-docs/usage/cli-commands.md
# etc.
```

### Step 4: Verify Consistency

```bash
# Ensure docs are accurate
# Check for typos
# Verify examples work
```

## Documentation Standards

### Be Specific

```markdown
# Good
## Parser Module (`src/parser.rs`)
Provides JSON parsing with comment support.
- `parse_project(path)` - Parse .outocut file
- `validate_project(path)` - Validate structure
```

```markdown
# Bad
## Parser
Parses files. Important for the project.
```

### Include Code References

```markdown
# Good
Use `AnimatedProperty<T>` for animatable properties:
```rust
pub struct AnimatedProperty<T> {
    pub value: T,
    pub keyframes: Option<Vec<Keyframe>>,
}
```
```

### Document Rationale

```markdown
# Good
## JSON for Project Files

We chose JSON because:
- AI-friendly (easy to generate)
- Git-compatible
- Standard tooling

See [Design Decisions](philosophy/design-decisions.md)
```

## Required Updates by Change Type

### New CLI Command

Update: `ai-docs/usage/cli-commands.md`

Add:
- Command syntax
- Arguments
- Options
- Examples

### New Module

Update: `ai-docs/architecture/modules.md`

Add:
- Module name and path
- Purpose
- Key functions
- Dependencies

### New Layer Type

Update: `ai-docs/usage/file-format.md`

Add:
- Layer type name
- Content structure
- Example

### New Effect

Update: `ai-docs/usage/file-format.md`

Add:
- Effect name
- Parameters
- Example

### Design Decision

Update: `ai-docs/philosophy/design-decisions.md`

Add:
- Decision description
- Rationale
- Alternatives considered

## Quality Checklist

Before committing documentation updates:

- [ ] All code changes documented
- [ ] Examples are accurate
- [ ] Cross-references valid
- [ ] No broken links
- [ ] Typo check passed

## Automation (Future)

Planned automation:

- [ ] Generate CLI docs from code
- [ ] Generate API docs
- [ ] Link checking CI
- [ ] Documentation tests

## Questions?

If unsure what to update:
1. Check existing documentation structure
2. Look at similar past changes
3. Ask in issue/discussion
