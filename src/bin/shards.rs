//! Shards CLI - command line interface for the Shards programming language

use std::ffi::{c_char, CString};
use std::env;

fn main() {
    // Initialize runtime
    shards_embed::init();

    // Convert args to C strings
    let args: Vec<CString> = env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();

    let argv: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();

    // Run CLI
    let result = shards_lang::cli::process_args(
        argv.len() as i32,
        argv.as_ptr(),
        false,
    );

    std::process::exit(result);
}
