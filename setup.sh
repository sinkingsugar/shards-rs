#!/bin/bash
# Setup script for shards-embed
# Run this once before first build

set -e

if [ -d "shards" ] || [ -L "shards" ]; then
    echo "shards directory already exists"
    exit 0
fi

echo "Cloning shards repository..."
git clone --depth 1 --branch devel https://github.com/fragcolor-xyz/shards.git shards

echo "Initializing submodules..."
cd shards
git submodule update --init --depth 1 \
    deps/stb deps/json deps/magic_enum deps/cpp-taskflow \
    deps/nameof deps/pdqsort deps/filesystem deps/xxHash \
    deps/linalg deps/spdlog deps/brotli deps/tracy deps/oneTBB \
    deps/crdt-lite deps/utf8.h deps/entt deps/kcp deps/SDL3 \
    deps/tinygltf deps/draco deps/sqlite/cr-sqlite deps/miniaudio \
    deps/kissfft deps/snappy \
    shards/gfx/rust/wgpu-native shards/gfx/rust/wgpu shards/gfx/rust/profiling

echo "Setup complete! Run: cargo build"
