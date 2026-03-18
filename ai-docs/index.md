# OutoCut Developer Documentation

Comprehensive documentation for developers working on OutoCut.

## Quick Links

### Getting Started
- [Development Setup](development/setup.md)
- [Contributing Guide](development/contributing.md)
- [Coding Standards](maintenance/coding-standards.md)

### Architecture
- [Architecture Overview](architecture/overview.md)
- [Module Breakdown](architecture/modules.md)
- [Data Flow](architecture/data-flow.md)

### Usage
- [File Format Specification](usage/file-format.md)
- [CLI Commands](usage/cli-commands.md)
- [Usage Examples](usage/examples.md)

### Philosophy
- [Core Principles](philosophy/core-principles.md)
- [Design Decisions](philosophy/design-decisions.md)
- [Known Limitations](philosophy/limitations.md)

### Deployment
- [Release Process](deployment/release-process.md)
- [Install Script](deployment/install-script.md)

### Maintenance
- [AI Docs Update Process](maintenance/ai-docs-update.md)
- [Coding Standards](maintenance/coding-standards.md)

---

## Documentation Map

```
ai-docs/
├── index.md                  ← You are here
├── architecture/
│   ├── overview.md           ← System architecture
│   ├── modules.md            ← Module details
│   └── data-flow.md         ← Data flow diagrams
├── development/
│   ├── setup.md              ← Development setup
│   ├── contributing.md       ← Contribution guide
│   ├── adding-features.md    ← Feature development
│   └── testing.md            ← Testing guide
├── philosophy/
│   ├── core-principles.md    ← Core principles
│   ├── design-decisions.md   ← Design rationale
│   └── limitations.md        ← Known limitations
├── deployment/
│   ├── release-process.md    ← Release workflow
│   └── install-script.md    ← Installation
├── usage/
│   ├── file-format.md        ← .outocut spec
│   ├── cli-commands.md        ← CLI reference
│   └── examples.md            ← Usage examples
└── maintenance/
    ├── ai-docs-update.md     ← Doc update process
    └── coding-standards.md   ← Coding conventions
```

---

## Common Tasks

### New Developer

1. Read [Core Principles](philosophy/core-principles.md)
2. Follow [Development Setup](development/setup.md)
3. Review [Architecture Overview](architecture/overview.md)

### Adding a Feature

1. Read [Adding Features](development/adding-features.md)
2. Follow coding standards
3. Add tests
4. Update documentation

### Fixing a Bug

1. Understand architecture
2. Write failing test
3. Fix the bug
4. Verify tests pass

### Contributing

1. Read [Contributing Guide](development/contributing.md)
2. Follow [Coding Standards](maintenance/coding-standards.md)
3. Update [AI Docs](maintenance/ai-docs-update.md)

---

## External Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Serde Documentation](https://serde.rs/)
- [Clap Documentation](https://clap.rs/)
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)

---

## Getting Help

- **Issues**: Open a GitHub issue
- **Discussions**: Start a GitHub discussion
- **Documentation**: Check this docs folder
