use cmake::Config;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap_or_default();
    let profile = env::var("PROFILE").unwrap();
    let is_apple = target_vendor == "apple";

    // Find shards source - either local or Cargo-cached
    let shards_dir = find_shards_source();

    let mut config = Config::new(&shards_dir);

    // Use Ninja generator (required for Swift support)
    config.generator("Ninja");

    // We ARE the Rust union, so skip CMake's Rust build
    config.define("SHARDS_NO_RUST_UNION", "ON");

    // Set build type based on Cargo profile
    let cmake_build_type = if profile == "release" {
        "Release"
    } else {
        "Debug"
    };
    config.define("CMAKE_BUILD_TYPE", cmake_build_type);

    // Map Cargo features to CMake options
    config.define("SHARDS_WITH_EVERYTHING", "OFF");
    config.define("SHARDS_WITH_DEFAULT", "OFF");

    // Default modules (always enabled)
    config.define("SHARDS_WITH_ANIM", "ON");
    config.define("SHARDS_WITH_ASSERT", "ON");
    config.define("SHARDS_WITH_AUDIO", "ON");
    config.define("SHARDS_WITH_BIGINT", "ON");
    config.define("SHARDS_WITH_BROTLI", "ON");
    config.define("SHARDS_WITH_CHANNELS", "ON");
    config.define("SHARDS_WITH_CORE", "ON");
    config.define("SHARDS_WITH_CRDTS", "ON");
    config.define("SHARDS_WITH_DEBUG", "ON");
    config.define("SHARDS_WITH_FILEOPS", "ON");
    config.define("SHARDS_WITH_IMAGING", "ON");
    config.define("SHARDS_WITH_JSON", "ON");
    config.define("SHARDS_WITH_OS", "ON");
    config.define("SHARDS_WITH_REFLECTION", "ON");
    config.define("SHARDS_WITH_RUN", "ON");
    config.define("SHARDS_WITH_SNAPPY", "ON");
    config.define("SHARDS_WITH_SQLITE", "ON");
    config.define("SHARDS_WITH_STRUCT", "ON");

    // Disabled modules (no feature flags for these)
    config.define("SHARDS_WITH_CLIPBOARD", "OFF");
    config.define("SHARDS_WITH_DEBUGGER", "OFF");
    config.define("SHARDS_WITH_DESKTOP", "OFF");
    config.define("SHARDS_WITH_GFX", "OFF");
    config.define("SHARDS_WITH_INPUTS", "OFF");
    config.define("SHARDS_WITH_PHYSICS", "OFF");
    config.define("SHARDS_WITH_WASM", "OFF");
    config.define("SHARDS_WITH_EGUI", "OFF");

    // Feature-gated modules
    if cfg!(feature = "ml") {
        config.define("SHARDS_WITH_ML", "ON");
        config.define("SHARDS_WITH_LLM", "ON");
    } else {
        config.define("SHARDS_WITH_ML", "OFF");
        config.define("SHARDS_WITH_LLM", "OFF");
    }
    if cfg!(feature = "crypto") {
        config.define("SHARDS_WITH_CRYPTO", "ON");
    } else {
        config.define("SHARDS_WITH_CRYPTO", "OFF");
    }
    if cfg!(feature = "csv") {
        config.define("SHARDS_WITH_CSV", "ON");
    } else {
        config.define("SHARDS_WITH_CSV", "OFF");
    }
    if cfg!(feature = "fs") {
        config.define("SHARDS_WITH_FS", "ON");
    } else {
        config.define("SHARDS_WITH_FS", "OFF");
    }
    if cfg!(feature = "http") {
        config.define("SHARDS_WITH_HTTP", "ON");
    } else {
        config.define("SHARDS_WITH_HTTP", "OFF");
    }
    if cfg!(feature = "network") {
        config.define("SHARDS_WITH_NETWORK", "ON");
    } else {
        config.define("SHARDS_WITH_NETWORK", "OFF");
    }
    if cfg!(feature = "pdf") {
        config.define("SHARDS_WITH_PDF", "ON");
    } else {
        config.define("SHARDS_WITH_PDF", "OFF");
    }
    if cfg!(feature = "ssh") {
        config.define("SHARDS_WITH_SSH", "ON");
    } else {
        config.define("SHARDS_WITH_SSH", "OFF");
    }
    if cfg!(feature = "svg") {
        config.define("SHARDS_WITH_SVG", "ON");
    } else {
        config.define("SHARDS_WITH_SVG", "OFF");
    }
    if cfg!(feature = "random") {
        config.define("SHARDS_WITH_RANDOM", "ON");
    } else {
        config.define("SHARDS_WITH_RANDOM", "OFF");
    }
    if cfg!(feature = "markdown") {
        config.define("SHARDS_WITH_MARKDOWN", "ON");
    } else {
        config.define("SHARDS_WITH_MARKDOWN", "OFF");
    }
    if cfg!(feature = "localshell") {
        config.define("SHARDS_WITH_LOCALSHELL", "ON");
    } else {
        config.define("SHARDS_WITH_LOCALSHELL", "OFF");
    }
    if cfg!(feature = "langffi") {
        config.define("SHARDS_WITH_LANGFFI", "ON");
    } else {
        config.define("SHARDS_WITH_LANGFFI", "OFF");
    }
    if cfg!(feature = "py") {
        config.define("SHARDS_WITH_PY", "ON");
        config.define("ENABLE_PYTHON_SHARDS", "ON");
        config.define("ENABLE_RUSTPYTHON_EMBEDDED", "ON");
    } else {
        config.define("SHARDS_WITH_PY", "OFF");
        config.define("ENABLE_PYTHON_SHARDS", "OFF");
        config.define("ENABLE_RUSTPYTHON_EMBEDDED", "OFF");
    }
    if cfg!(feature = "tracy") {
        config.define("TRACY_ENABLE", "ON");
        config.define("SHARDS_WITH_TRACY", "ON");
    } else {
        config.define("TRACY_ENABLE", "OFF");
        config.define("SHARDS_WITH_TRACY", "OFF");
    }

    // Build the C++ union target
    config.build_target("shards-cpp-union");

    let dst = config.build();

    // Build crsql_bundle-rust separately (for SQLite CRDT support)
    // This must be built after the main configure step
    let cmake_build_dir = dst.join("build");
    let status = std::process::Command::new("ninja")
        .arg("-C")
        .arg(&cmake_build_dir)
        .arg("cargo-crsql_bundle-rust")
        .status()
        .expect("Failed to build crsql_bundle-rust");
    if !status.success() {
        panic!("Failed to build crsql_bundle-rust");
    }

    // Link paths
    let build_dir = dst.join("build");
    let lib_dir = build_dir.join("lib");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // TBB puts libraries in compiler-specific directories
    // Scan for directories that contain TBB library files
    if let Ok(entries) = std::fs::read_dir(&build_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Check if this directory contains TBB libraries
                let has_tbb = path.join("libtbb.a").exists()
                    || path.join("libtbb_debug.a").exists()
                    || path.join("tbb.lib").exists()
                    || path.join("tbb_debug.lib").exists();
                if has_tbb {
                    println!("cargo:rustc-link-search=native={}", path.display());
                }
            }
        }
    }

    // External dependencies in nested build directories
    let kissfft_dir = build_dir.join("deps/kissfft_a/src/kissfft_a-build");
    if kissfft_dir.exists() {
        println!("cargo:rustc-link-search=native={}", kissfft_dir.display());
    }
    let mozjpeg_dir = build_dir.join("deps/mozjpeg_a/src/mozjpeg_a-build");
    if mozjpeg_dir.exists() {
        println!("cargo:rustc-link-search=native={}", mozjpeg_dir.display());
    }

    // Rust libraries built by CMake's corrosion (crsql, etc)
    let target_arch = env::var("TARGET").unwrap_or_else(|_| "aarch64-apple-darwin".to_string());
    let rust_target_dir = if profile == "release" {
        build_dir.join("target").join(&target_arch).join("release")
    } else {
        build_dir.join("target").join(&target_arch).join("debug")
    };
    if rust_target_dir.exists() {
        println!("cargo:rustc-link-search=native={}", rust_target_dir.display());
        println!("cargo:rustc-link-search=native={}", rust_target_dir.join("deps").display());
    }

    // Link the main C++ union (must come first)
    println!("cargo:rustc-link-lib=static=shards-cpp-union");

    // Core shards libraries
    println!("cargo:rustc-link-lib=static=shards-core");
    println!("cargo:rustc-link-lib=static=shards-logging");
    println!("cargo:rustc-link-lib=static=shards-fast-string");

    // Boost libraries
    println!("cargo:rustc-link-lib=static=boost_filesystem");
    println!("cargo:rustc-link-lib=static=boost_container");
    println!("cargo:rustc-link-lib=static=boost_context");
    println!("cargo:rustc-link-lib=static=boost_thread");
    println!("cargo:rustc-link-lib=static=boost_atomic");
    println!("cargo:rustc-link-lib=static=boost_chrono");
    println!("cargo:rustc-link-lib=static=boost_date_time");
    println!("cargo:rustc-link-lib=static=boost_random");
    // boost_stacktrace variant differs by platform
    if is_apple {
        println!("cargo:rustc-link-lib=static=boost_stacktrace_basic");
    } else if target_os == "windows" {
        println!("cargo:rustc-link-lib=static=boost_stacktrace_windbg");
    } else {
        println!("cargo:rustc-link-lib=static=boost_stacktrace_addr2line");
    }

    // Third-party libraries
    if profile == "release" {
        println!("cargo:rustc-link-lib=static=spdlog");
        println!("cargo:rustc-link-lib=static=tbb");
    } else {
        println!("cargo:rustc-link-lib=static=spdlogd");
        println!("cargo:rustc-link-lib=static=tbb_debug");
    }
    println!("cargo:rustc-link-lib=static=kcp");
    println!("cargo:rustc-link-lib=static=TracyClient");

    // Compression libraries
    println!("cargo:rustc-link-lib=static=brotlicommon");
    println!("cargo:rustc-link-lib=static=brotlidec");
    println!("cargo:rustc-link-lib=static=brotlienc");
    println!("cargo:rustc-link-lib=static=snappy");

    // SQLite
    println!("cargo:rustc-link-lib=static=sqlite-static");
    println!("cargo:rustc-link-lib=static=sqlite-vec");
    println!("cargo:rustc-link-lib=static=crsql_bundle");

    // Audio
    println!("cargo:rustc-link-lib=static=opus");
    println!("cargo:rustc-link-lib=static=kissfft-float");

    // Imaging
    println!("cargo:rustc-link-lib=static=jpeg");

    // SDL3 is used by core for SDL_getenv etc
    println!("cargo:rustc-link-lib=static=SDL3");

    // Platform-specific dependencies
    if is_apple {
        // Swift implementation for core (shards_openURL etc)
        println!("cargo:rustc-link-lib=static=shards-core-swift-impl");

        println!("cargo:rustc-link-lib=c++");

        // Common Apple frameworks
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=iconv");

        let is_watchos = target_os == "watchos";
        let is_mobile = target_os == "ios" || target_os == "visionos" || is_watchos;

        // Not watchOS
        if !is_watchos {
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalKit");
            println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
            println!("cargo:rustc-link-lib=framework=CoreHaptics");
            println!("cargo:rustc-link-lib=framework=GameController");
            println!("cargo:rustc-link-lib=framework=SystemConfiguration");
        }

        // Common to all (needed by SDL3)
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=CoreMedia");

        // iOS/visionOS/watchOS
        if is_mobile {
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=UIKit");
            println!("cargo:rustc-link-lib=framework=CoreMotion");
        } else {
            // macOS only
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=ForceFeedback");
            println!("cargo:rustc-link-lib=framework=CoreServices");
        }

        // Swift runtime
        println!("cargo:rustc-link-arg=-Xlinker");
        println!("cargo:rustc-link-arg=-rpath");
        println!("cargo:rustc-link-arg=-Xlinker");
        println!("cargo:rustc-link-arg=/usr/lib/swift");
    } else if target_os == "linux" {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
        // For boost_stacktrace_basic
        println!("cargo:rustc-link-lib=bfd");
    } else if target_os == "windows" {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=ole32");
    }

    // Re-run if shards source changes
    println!("cargo:rerun-if-changed=shards/shards/core");
    println!("cargo:rerun-if-changed=shards/CMakeLists.txt");
}

fn find_shards_source() -> String {
    // Check for local shards directory first (symlink or clone)
    let local_shards = Path::new("shards");
    if local_shards.exists() {
        return "shards".to_string();
    }

    // Find Cargo's cached git checkout
    let cargo_home = env::var("CARGO_HOME")
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME not set");
            format!("{}/.cargo", home)
        });

    let git_checkouts = Path::new(&cargo_home).join("git").join("checkouts");

    // Look for shards checkout directory
    if let Ok(entries) = std::fs::read_dir(&git_checkouts) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("shards-") {
                // Find the actual checkout (there may be multiple revisions)
                if let Ok(revisions) = std::fs::read_dir(entry.path()) {
                    for rev in revisions.flatten() {
                        let rev_path = rev.path();
                        if rev_path.join("CMakeLists.txt").exists() {
                            let path_str = rev_path.to_string_lossy().to_string();
                            println!("cargo:warning=Using Cargo-cached shards at {}", path_str);

                            // Initialize submodules needed for CMake
                            init_submodules(&rev_path);

                            return path_str;
                        }
                    }
                }
            }
        }
    }

    panic!(
        "Could not find shards source!\n\
         Either:\n\
         - Create a symlink: ln -s /path/to/shards shards\n\
         - Or run: cargo build (Cargo will fetch from git)\n\
         \n\
         Note: The shards repo needs a root Cargo.toml with workspace members.\n\
         See: https://github.com/fragcolor-xyz/shards"
    );
}

fn init_submodules(shards_dir: &Path) {
    // Check if submodules already initialized (check for one key dep)
    if shards_dir.join("deps/spdlog/CMakeLists.txt").exists() {
        return;
    }

    println!("cargo:warning=Initializing shards submodules for CMake...");

    // Core submodules needed for CMake build
    let submodules = [
        "deps/stb",
        "deps/json",
        "deps/magic_enum",
        "deps/cpp-taskflow",
        "deps/nameof",
        "deps/pdqsort",
        "deps/filesystem",
        "deps/xxHash",
        "deps/linalg",
        "deps/spdlog",
        "deps/brotli",
        "deps/tracy",
        "deps/oneTBB",
        "deps/crdt-lite",
        "deps/utf8.h",
        "deps/entt",
        "deps/kcp",
        "deps/SDL3",
        "deps/tinygltf",
        "deps/draco",
        "deps/sqlite/cr-sqlite",
        "deps/miniaudio",
        "deps/kissfft",
        "deps/snappy",
    ];

    let status = Command::new("git")
        .current_dir(shards_dir)
        .args(["submodule", "update", "--init", "--depth", "1"])
        .args(&submodules)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Submodules initialized successfully");
        }
        _ => {
            println!("cargo:warning=Failed to initialize some submodules - CMake may fail");
        }
    }
}
