# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

EchoVoice is a local AI voice input method application. Press and hold F9 to record, release to transcribe and polish text using local Whisper and SmolLM2 models, then output to clipboard.

## Architecture

### Workspace Structure

This is a Rust workspace with multiple crates:

```
crates/
├── audio/      # Audio recording (cpal)
├── asr/        # Speech recognition (whisper.cpp bindings)
├── llm/        # Text polishing (llama.cpp bindings)
├── config/     # Configuration management (YAML)
├── hotkey/     # Global hotkey listener (rdev)
├── output/     # Clipboard output (arboard)
├── floating/   # Floating capsule UI (platform-specific)
└── tray/       # System tray (optional)
```

Main binary is at `src/main.rs` which orchestrates the pipeline: Hotkey → Audio → ASR → LLM → Output.

### Platform-Specific Code

The codebase supports macOS and Windows with platform-specific implementations:

- **Floating capsule**: `crates/floating/src/` has `macos/` and `windows/` submodules using Core Animation vs Direct2D
- **Build flags**: Windows requires `/FORCE:MULTIPLE` linker flag (configured in `.cargo/config.toml`)
- **Hotkey**: Uses `rdev` with different permission models per platform

### External Dependencies

- **whisper.cpp**: C++ library for ASR, bound via `whisper-rs`
- **llama.cpp**: C++ library for LLM inference, bound via `llama-cpp-rs`
- **Tauri**: Settings panel UI (in `src-tauri/`)

## Common Commands

### Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Build specific crate
cargo build -p echovoice-audio
```

### Run

```bash
# Run main application (requires models in models/ directory)
./target/release/echovoice

# Development run
cargo run
```

### Test

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p echovoice-config
cargo test -p echovoice-audio
```

### Lint/Check

```bash
# Check compilation without building
cargo check

# Format code
cargo fmt

# Clippy linting
cargo clippy
```

## Glue Programming Practice

**Trigger**: Any code generation, modification, or refactoring task

### Core Principle

```
                    Glue Programming Pyramid

                        /\
                       /  \
                      /    \
                     / Reuse \
                    /────────\
                   / Compose  \
                  /──────────\
                 / Adapter    \
                /──────────────\
               / Custom Logic   \
              /──────────────────\
             / Business Wrapper   \
            /──────────────────────\
           / Domain Specific Logic   \
          /──────────────────────────\
         /    Glue Layer (5-10%)      \
        /──────────────────────────────\
       /     Mature Components (90-95%)  \
      /──────────────────────────────────\
     /                                    \
    └────────────────────────────────────┘
```

### Execution Workflow

1. **Search Components**: Prioritize searching GitHub Topics, crates.io, docs.rs, and official documentation for mature solutions
2. **Evaluate Quality**: Check GitHub stars, maintenance frequency, documentation completeness, and community activity
3. **Analyze Interfaces**: Read official docs to understand input/output data formats
4. **Design Glue Layer**: Write only necessary adapter code, keep logic simple
5. **Integration Test**: Verify data flow between modules

### Code Reuse Priority (High to Low)

1. **Rust Standard Library** - `std::sync`, `std::io`, `std::path`, `std::collections` - zero dependencies
2. **Mature Framework Features** - Tauri's WebView, Tokio's async runtime, Serde's serialization
3. **crates.io High-Star Crates** - Choose crates with high download counts and active maintenance
4. **GitHub High-Star Projects** - Reference implementation patterns from similar projects
5. **Custom Implementation** - Only write your own when none of the above meet requirements

### Research Methodology

- Use WebSearch: `"rust" + feature + "github/stars"`
- Use WebFetch to get official documentation and code examples
- Prioritize reading official documentation, then reference high-star projects
- Record found code patterns and apply to current project

### Code Review Checklist

- [ ] Searched all available mature components
- [ ] Custom code is within 10% of total
- [ ] No external dependency source code modified
- [ ] Interface adapters follow official documentation
- [ ] Edge cases handled by frameworks

## Development Setup

### Required Models

Download required models to `models/` directory:

```bash
# Using provided script
./scripts/download-models.sh

# Or manually
curl -L -o models/ggml-base.bin \
  "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin"
curl -L -o models/smollm2-360m-q8.gguf \
  "https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF/resolve/main/smollm2-360m-instruct-q8_0.gguf"
```

### Configuration

Config file location:
- macOS: `~/.config/echovoice/config.yaml`
- Windows: `%APPDATA%\echovoice\config.yaml`

Example:
```yaml
hotkey:
  primary: "F9"
asr:
  model: "whisper-base"
  language: "auto"
llm:
  model: "smollm2-360m"
```

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | Main application loop, orchestrates all modules |
| `Cargo.toml` | Workspace definition, lists all crates |
| `.cargo/config.toml` | Platform-specific build configuration |
| `src-tauri/tauri.conf.json` | Tauri settings panel configuration |
| `.dev/PLAN.md` | Development roadmap and phase tracking |
| `.dev/architecture/data-flow.md` | Detailed data flow documentation |

## CI/CD

GitHub Actions workflow (`.github/workflows/build.yml`):
- Builds for macOS and Windows
- Downloads models during build
- Creates DMG (macOS) and MSI (Windows) packages
- Uses `scripts/build-dmg.sh` and `scripts/build-msi.sh` for packaging

## Important Notes

- The `target/` directory should NEVER be committed (was accidentally committed before, fixed in commit `3d7dfde`)
- C++ bindings (whisper.cpp, llama.cpp) require CMake and take significant time to compile
- First build will be slow due to C++ compilation; subsequent builds are faster
- Windows builds require LLVM installed (`choco install llvm`)
