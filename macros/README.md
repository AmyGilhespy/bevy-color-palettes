# weirdboi_bevy_colour_macros

This is the proc-macro crate used by the `weirdboi_bevy_colour` library.

You normally won’t depend on this crate directly. Instead, use the main crate, which re-exports the macros for defining palettes at compile time.

- Main crate: https://crates.io/crates/weirdboi_bevy_colour (repo homepage: https://weirdboi.dev/libraries/bevy-colours)
- Provides the `palette!` macro for declaring colour palettes and utilities used by the main crate’s APIs.

## Usage

Most users should import macros through the main crate:

```rust
use weirdboi_bevy_colour::palette;

palette!(MyPalette {
    "bg": (0.1, 0.1, 0.2),
    "fg": (0.9, 0.9, 1.0),
});
```

If you really need to depend on the macros crate directly (advanced use-cases), add this to your Cargo.toml:

```toml
[dependencies]
weirdboi_bevy_colour_macros = "0.1.0"
```

But prefer the main crate unless you know you need only the macros.
