# shards-embed

A Rust crate for embedding the [Shards](https://github.com/fragcolor-xyz/shards) programming language runtime.

## Overview

`shards-embed` provides an easy way for Rust projects to consume and embed Shards. The main Shards repository is a complex mix of C++ and Rust, making direct integration challenging. This crate simplifies that by providing a clean Cargo-based interface with the C++ core built transparently via CMake.

Shards is a visual/textual programming language designed for interactive applications, game development, and creative coding.

## Features

The language runtime (`core` + `langffi`) is always built. Every other module is a
feature flag, so an embedder only pays for what it uses.

### Default Features
- `cli` - Command-line interface (`shards` binary)
- `fs` - File system operations (pulls in native file dialogs on Linux)
- `random`, `assert`, `bigint`, `channels`, `json`, `reflection`, `struct` - Core modules

### Optional Features
- `ml` - Machine learning (candle / llama.cpp)
- `crypto` - Cryptography
- `csv` - CSV file handling
- `geo` - Geospatial
- `http` - HTTP client/server
- `network` - Networking primitives
- `pdf`, `svg`, `imaging` - Document and image processing
- `markdown` - Markdown parsing
- `localshell` - Local shell execution
- `anim`, `audio`, `debug`, `fileops`, `os` - Misc modules
- `brotli`, `snappy` - Compression
- `crdts` - Conflict-free replicated data types
- `sqlite` - SQLite database (with cr-sqlite and sqlite-vec)

Use the `full` feature to enable all modules (except `geo`).

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
shards-embed = { git = "https://github.com/sinkingsugar/shards-rs" }
```

### Language only

To embed just the scripting language, turn off default features and add back the
modules your scripts use:

```toml
[dependencies]
shards-embed = { git = "https://github.com/sinkingsugar/shards-rs", default-features = false, features = ["json", "struct"] }
```

With no features at all you still get the language and its core shards (wires, flow control,
math, sequences, etc.); shards from disabled modules simply don't exist at runtime.

If you enable `ml`, copy the `[patch."https://github.com/huggingface/candle.git"]` section
from this crate's `Cargo.toml` into your own: Cargo only applies `[patch]` from the
top-level workspace.

Basic example:

```rust
use shards_embed;

fn main() {
    shards_embed::init();
    let result = shards_embed::run_file("script.shs");
    std::process::exit(result);
}
```

## Building

### Requirements
- Rust nightly toolchain (required for dependencies)
- CMake 3.15+
- Ninja build system
- C++17 compiler
- Platform-specific dependencies:
  - **Linux**: `libssl-dev`, `libasound2-dev`, `libpulse-dev`, `libwayland-dev`, `binutils-dev`
  - **macOS**: Xcode command line tools
  - **Windows**: LLVM/Clang

### Build Commands

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Build with all features
cargo build --all-features
```

## Platform Support

- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)
- Windows (x86_64)

## Development

This project uses a justfile for common tasks:

```bash
# Full CI check locally
just ci

# Prepare for release
just release-prep
```

## License

BSD 3-Clause License - See [LICENSE](LICENSE) file for details.

## Links

- [Shards Main Repository](https://github.com/fragcolor-xyz/shards)
- [Shards Documentation](https://docs.fragcolor.com)
