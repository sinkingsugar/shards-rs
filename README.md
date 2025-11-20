# shards-embed

Rust bindings for the [Shards](https://github.com/fragcolor-xyz/shards) programming language runtime.

## Overview

`shards-embed` provides safe Rust bindings to embed the Shards programming language in your Rust applications. Shards is a visual/textual programming language designed for interactive applications, game development, and creative coding.

## Features

The crate supports modular feature flags to control which Shards modules are compiled:

### Default Features
- `cli` - Command-line interface (`shards` binary)
- `core` - Core language features
- `langffi` - Foreign function interface
- `fs` - File system operations
- `random` - Random number generation
- `assert`, `bigint`, `channels`, `json`, `reflection`, `struct` - Core modules

### Optional Features
- `ml` - Machine learning
- `crypto` - Cryptography
- `csv` - CSV file handling
- `http` - HTTP client/server
- `network` - Networking primitives
- `pdf`, `svg`, `imaging` - Document and image processing
- `ssh` - SSH client
- `markdown` - Markdown parsing
- `localshell` - Local shell execution
- `py` - Python interop (RustPython)
- `audio` - Audio processing
- `brotli`, `snappy` - Compression
- `crdts` - Conflict-free replicated data types
- `sqlite` - SQLite database
- And more...

Use `full` feature to enable all modules.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
shards-embed = "0.1"
```

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
