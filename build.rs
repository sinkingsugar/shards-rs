use cmake::Config;
use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let mut config = Config::new("shards");

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
    // Default everything OFF, then enable based on features
    config.define("SHARDS_WITH_EVERYTHING", "OFF");
    config.define("SHARDS_WITH_DEFAULT", "OFF");

    // Core modules (always needed)
    config.define("SHARDS_WITH_CORE", "ON");

    // Feature-gated modules
    if cfg!(feature = "gfx") {
        config.define("SHARDS_WITH_GFX", "ON");
        config.define("SHARDS_WITH_INPUTS", "ON"); // Required by gfx for InputMaster
    }
    if cfg!(feature = "egui") {
        config.define("SHARDS_WITH_EGUI", "ON");
    }
    if cfg!(feature = "ml") {
        config.define("SHARDS_WITH_ML", "ON");
    }
    if cfg!(feature = "crypto") {
        config.define("SHARDS_WITH_CRYPTO", "ON");
    }
    if cfg!(feature = "csv") {
        config.define("SHARDS_WITH_CSV", "ON");
    }
    if cfg!(feature = "fs") {
        config.define("SHARDS_WITH_FS", "ON");
    }
    if cfg!(feature = "http") {
        config.define("SHARDS_WITH_HTTP", "ON");
    }
    if cfg!(feature = "network") {
        config.define("SHARDS_WITH_NETWORK", "ON");
    }
    if cfg!(feature = "pdf") {
        config.define("SHARDS_WITH_PDF", "ON");
    }
    if cfg!(feature = "ssh") {
        config.define("SHARDS_WITH_SSH", "ON");
    }
    if cfg!(feature = "svg") {
        config.define("SHARDS_WITH_SVG", "ON");
    }
    if cfg!(feature = "random") {
        config.define("SHARDS_WITH_RANDOM", "ON");
    }
    if cfg!(feature = "markdown") {
        config.define("SHARDS_WITH_MARKDOWN", "ON");
    }
    if cfg!(feature = "localshell") {
        config.define("SHARDS_WITH_LOCALSHELL", "ON");
    }
    if cfg!(feature = "langffi") {
        config.define("SHARDS_WITH_LANGFFI", "ON");
    }
    if cfg!(feature = "py") {
        config.define("SHARDS_WITH_PY", "ON");
    }
    if cfg!(feature = "tracy") {
        config.define("TRACY_ENABLE", "ON");
    }

    // Build only the C++ union target
    config.build_target("shards-cpp-union");

    let dst = config.build();

    // Link paths
    let build_dir = dst.join("build");
    let lib_dir = build_dir.join("lib");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // TBB is in a weird location
    let tbb_dir = build_dir.join("appleclang_17.0_cxx17_64_debug");
    if tbb_dir.exists() {
        println!("cargo:rustc-link-search=native={}", tbb_dir.display());
    }
    let tbb_dir_release = build_dir.join("appleclang_17.0_cxx17_64_release");
    if tbb_dir_release.exists() {
        println!("cargo:rustc-link-search=native={}", tbb_dir_release.display());
    }

    // Rust libraries built by corrosion (gfx, etc)
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
    println!("cargo:rustc-link-lib=static=boost_stacktrace_basic");

    // Third-party libraries
    if profile == "release" {
        println!("cargo:rustc-link-lib=static=spdlog");
        println!("cargo:rustc-link-lib=static=tbb");
    } else {
        println!("cargo:rustc-link-lib=static=spdlogd");
        println!("cargo:rustc-link-lib=static=tbb_debug");
    }
    println!("cargo:rustc-link-lib=static=kcp");
    println!("cargo:rustc-link-lib=static=brotlicommon");
    println!("cargo:rustc-link-lib=static=brotlidec");
    println!("cargo:rustc-link-lib=static=draco");
    println!("cargo:rustc-link-lib=static=TracyClient");

    // GFX libraries
    if cfg!(feature = "gfx") {
        println!("cargo:rustc-link-lib=static=gfx");
        println!("cargo:rustc-link-lib=static=gfx-swift");
        println!("cargo:rustc-link-lib=static=SDL3");
        println!("cargo:rustc-link-lib=static=shards-core-swift-impl");
    }

    // Platform-specific dependencies
    match target_os.as_str() {
        "macos" | "ios" => {
            println!("cargo:rustc-link-lib=c++");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=Security");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreServices");

            // Swift runtime
            println!("cargo:rustc-link-arg=-Xlinker");
            println!("cargo:rustc-link-arg=-rpath");
            println!("cargo:rustc-link-arg=-Xlinker");
            println!("cargo:rustc-link-arg=/usr/lib/swift");

            if cfg!(feature = "gfx") {
                println!("cargo:rustc-link-lib=framework=Metal");
                println!("cargo:rustc-link-lib=framework=MetalKit");
                println!("cargo:rustc-link-lib=framework=QuartzCore");
                println!("cargo:rustc-link-lib=framework=Cocoa");
                println!("cargo:rustc-link-lib=framework=Carbon");
                println!("cargo:rustc-link-lib=framework=ForceFeedback");
                println!("cargo:rustc-link-lib=framework=GameController");
                println!("cargo:rustc-link-lib=framework=CoreHaptics");
                println!("cargo:rustc-link-lib=framework=AVFoundation");
                println!("cargo:rustc-link-lib=framework=CoreMedia");
                println!("cargo:rustc-link-lib=framework=CoreVideo");
                println!("cargo:rustc-link-lib=framework=CoreAudio");
                println!("cargo:rustc-link-lib=framework=AudioToolbox");
            }
        }
        "linux" => {
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=dl");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=shell32");
            println!("cargo:rustc-link-lib=ole32");
        }
        _ => {}
    }

    // Re-run if shards source changes
    println!("cargo:rerun-if-changed=shards/shards/core");
    println!("cargo:rerun-if-changed=shards/CMakeLists.txt");
}
