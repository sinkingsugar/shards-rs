# Shards-RS Development Notes

## Adding a New Rust Module

When adding support for a new Rust module from the shards repository:

1. **Cargo.toml** - Add the feature and dependency:
   ```toml
   # In [features] section:
   modulename = ["dep:shards-modulename"]

   # In [dependencies] section:
   shards-modulename = { git = "https://github.com/fragcolor-xyz/shards.git", rev = "CURRENT_REV", optional = true }
   ```

2. **build.rs** - Add the CMake define (alphabetically with other modules):
   ```rust
   if cfg!(feature = "modulename") {
       config.define("SHARDS_WITH_MODULENAME", "ON");
   } else {
       config.define("SHARDS_WITH_MODULENAME", "OFF");
   }
   ```

3. **src/lib.rs** - Add the re-export to force linking:
   ```rust
   #[cfg(feature = "modulename")]
   pub use shards_modulename;
   ```

4. **Update the git rev** if the module isn't in the current pinned revision.

## Module Types

- **Rust modules** (like geo, csv, fs): Built by Cargo as dependencies, use `["dep:shards-xxx"]` feature syntax
- **C++ modules** (like anim, channels): Built by CMake, use empty `[]` feature syntax, no Cargo dependency needed

## Module-Specific Features

Some modules require specific features to be enabled:

- **shards-http**: Requires `native-tls` or `rustls` feature for TLS support (methods like `danger_accept_invalid_certs` need this)
- **shards-fs**: Uses `rfd-enabled` and `rfd-xdg` features for file dialogs
- **shards-core**: Uses `default` features

Check the module's CMakeLists.txt in shards to see what features it uses (look for `FEATURES` argument in `add_rust_library`).
