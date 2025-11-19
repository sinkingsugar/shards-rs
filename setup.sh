#!/bin/bash
# Setup script for shards-embed
# Run this once before first build
#
# Usage:
#   ./setup.sh              # Minimal (no gfx)
#   ./setup.sh --gfx        # With graphics
#   ./setup.sh --full       # Everything
#   ./setup.sh --ml         # With ML/LLM

set -e

if [ -d "shards" ] || [ -L "shards" ]; then
    echo "shards directory already exists"
    exit 0
fi

# Parse arguments
GFX=false
ML=false
FULL=false

for arg in "$@"; do
    case $arg in
        --gfx)  GFX=true ;;
        --ml)   ML=true ;;
        --full) FULL=true; GFX=true; ML=true ;;
    esac
done

echo "Cloning shards repository..."
git clone --depth 1 --branch devel https://github.com/fragcolor-xyz/shards.git shards

cd shards

# Core submodules (always needed)
SUBMODULES=(
    deps/stb deps/json deps/magic_enum deps/cpp-taskflow
    deps/nameof deps/pdqsort deps/filesystem deps/xxHash
    deps/linalg deps/spdlog deps/brotli deps/tracy deps/oneTBB
    deps/crdt-lite deps/utf8.h deps/entt deps/kcp
    deps/sqlite/cr-sqlite deps/miniaudio deps/kissfft deps/snappy
)

# GFX submodules
if [ "$GFX" = true ]; then
    echo "Including GFX submodules..."
    SUBMODULES+=(
        deps/SDL3 deps/tinygltf deps/draco
        shards/gfx/rust/wgpu-native shards/gfx/rust/wgpu shards/gfx/rust/profiling
    )
fi

# ML submodules
if [ "$ML" = true ]; then
    echo "Including ML submodules..."
    SUBMODULES+=(deps/llama.cpp deps/whisper.cpp)
fi

echo "Initializing ${#SUBMODULES[@]} submodules..."
git submodule update --init --depth 1 "${SUBMODULES[@]}"

echo ""
echo "Setup complete!"
echo "Run: cargo build"
if [ "$GFX" = false ]; then
    echo ""
    echo "Note: GFX not included. Run with --gfx for graphics support."
fi
