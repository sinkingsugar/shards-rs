//! Shards - Embeddable flow-based programming language
//!
//! # Example
//!
//! ```rust,no_run
//! fn main() {
//!     shards::init();
//!     let result = shards::run_file("script.shs");
//!     std::process::exit(result);
//! }
//! ```

use std::ffi::{c_char, CString};

// Re-export base shards crate
pub use shards::*;

// Re-export language
pub use shards_lang;

// Re-export fileops (forces linking of shardsRegister_fileops_rust)
pub use shards_fileops;

// CR-SQLite bundle disabled due to package resolution issues
// pub use crsql_bundle;

// Conditional re-exports based on features
// GFX/EGUI disabled for now - need nested workspace support
// #[cfg(feature = "gfx")]
// pub use gfx;
// #[cfg(feature = "egui")]
// pub use shards_egui_register;

#[cfg(feature = "ml")]
pub use shards_ml;

#[cfg(feature = "core")]
pub use shards_core;

#[cfg(feature = "crypto")]
pub use shards_crypto;

#[cfg(feature = "csv")]
pub use shards_csv;

#[cfg(feature = "fs")]
pub use shards_fs;

#[cfg(feature = "http")]
pub use shards_http;

#[cfg(feature = "network")]
pub use shards_network;

#[cfg(feature = "pdf")]
pub use shards_pdf;

#[cfg(feature = "ssh")]
pub use shards_ssh;

#[cfg(feature = "svg")]
pub use shards_svg;

#[cfg(feature = "random")]
pub use shards_random;

#[cfg(feature = "markdown")]
pub use shards_markdown;

#[cfg(feature = "localshell")]
pub use shards_localshell;

#[cfg(feature = "langffi")]
pub use shards_langffi;

#[cfg(feature = "py")]
pub use shards_py;

// FFI declarations for C++ core
extern "C" {
    fn shardsInterface(version: u32) -> *mut shards::shardsc::SHCore;
    fn shards_install_signal_handlers();
    fn shards_decompress_strings();
}

/// Initialize the shards runtime.
///
/// Must be called once before any other shards functions.
/// Safe to call multiple times (subsequent calls are no-ops).
pub fn init() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        unsafe {
            shards::core::Core = shardsInterface(shards::SHARDS_CURRENT_ABI as u32);
            (*shards::core::Core).init.unwrap()();
            shards_install_signal_handlers();
        }
    });
}

/// Initialize with decompressed help strings.
///
/// Use this if you need access to shard documentation/help text.
pub fn init_with_docs() {
    init();
    unsafe {
        shards_decompress_strings();
    }
}

/// Run a shards script file.
///
/// Returns 0 on success, non-zero on error.
pub fn run_file(path: &str) -> i32 {
    init();

    let args = vec![
        CString::new("shards").unwrap(),
        CString::new("run").unwrap(),
        CString::new(path).unwrap(),
    ];
    let argv: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();

    shards_lang::cli::process_args(argv.len() as i32, argv.as_ptr(), false)
}

/// Run a shards script file with custom arguments.
///
/// Arguments should be in "key:value" format.
/// Returns 0 on success, non-zero on error.
pub fn run_file_with_args(path: &str, script_args: &[&str]) -> i32 {
    init();

    let mut args = vec![
        CString::new("shards").unwrap(),
        CString::new("run").unwrap(),
        CString::new(path).unwrap(),
    ];

    for arg in script_args {
        args.push(CString::new(*arg).unwrap());
    }

    let argv: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();

    shards_lang::cli::process_args(argv.len() as i32, argv.as_ptr(), false)
}

/// Evaluate shards code from a string.
///
/// Returns 0 on success, non-zero on error.
pub fn eval_string(code: &str) -> i32 {
    init();

    // Use stdin-like evaluation
    let args = vec![
        CString::new("shards").unwrap(),
        CString::new("eval").unwrap(),
    ];
    let argv: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();

    // TODO: This needs piping code to stdin, for now just use run_file
    // For proper string eval, we'd need to extend the CLI or use the lower-level API
    shards_lang::cli::process_args(argv.len() as i32, argv.as_ptr(), false)
}

/// Build a shards script to binary format.
///
/// Returns 0 on success, non-zero on error.
pub fn build_file(input: &str, output: &str) -> i32 {
    init();

    let args = vec![
        CString::new("shards").unwrap(),
        CString::new("build").unwrap(),
        CString::new(input).unwrap(),
        CString::new("-o").unwrap(),
        CString::new(output).unwrap(),
    ];
    let argv: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();

    shards_lang::cli::process_args(argv.len() as i32, argv.as_ptr(), false)
}

/// Load and run a pre-compiled shards binary.
///
/// Returns 0 on success, non-zero on error.
pub fn load_binary(path: &str) -> i32 {
    init();

    let args = vec![
        CString::new("shards").unwrap(),
        CString::new("load").unwrap(),
        CString::new(path).unwrap(),
    ];
    let argv: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();

    shards_lang::cli::process_args(argv.len() as i32, argv.as_ptr(), false)
}
