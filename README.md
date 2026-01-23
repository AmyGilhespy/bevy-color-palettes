# Bevy Color Palettes

This is a fork of `weirdboi_bevy_colour` which adds more pre-defined palettes that I needed for my own projects.  It is currently in early development.

- All palettes from Aseprite have been added.
- All palettes from bevy::color::palettes have been added (with egui support).

## Bevy Compatibility

| version | bevy |
|---------|------|
| 0.2     | 0.18 |
| 0.1     | 0.17 |

## Attribution

This project is based on work originally published at:
https://weirdboi.dev/libraries/bevy-colours

Licensed under the Apache License, Version 2.0.

This fork significantly expands the available palettes
and is independently maintained.

## Original Description

A Rust library providing a collection of popular colour palettes for the [Bevy](https://bevy.org/) game engine, with utilities for interacting with them.

- **Create colour palettes**: Create flexible colour palettes with the `palette!` macro

- **Pre-defined Colour Palettes**: Includes several popular colour palettes from [Lospec](https://lospec.com/):
  - [Nanner Pancakes](https://lospec.com/palette-list/nanner-pancakes)
  - [Resurrect 64](https://lospec.com/palette-list/resurrect-64)
  - [Resurrect 32](https://lospec.com/palette-list/resurrect-32)
  - [Dawnbringer 16](https://lospec.com/palette-list/dawnbringer-16)
  - [Dawnbringer 32](https://lospec.com/palette-list/dawnbringer-32)

- **Visual Documentation**: Each palette includes custom HTML when generating a Rustdoc to showcase the available colours. Integrates nicely with IDE doc previews

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy-color-palettes = { git = "https://github.com/AmyGilhespy/bevy-color-palettes", branch = "main" }
```

## Usage

### Using Existing Palettes

```rust
use bevy::prelude::*;
use bevy_color_palettes::Dawnbringer16;

fn setup(mut commands: Commands) {
  let mut x = 0.0;
  
  // Spawn a coloured square for each colour in the palette
  for color in &Dawnbringer16 {
    commands.spawn((
      Sprite {
        color,
        custom_size: Some(Vec2::new(40.0, 40.0)),
        ..default()
      },
      Transform::from_xyz(x, 0.0, 0.0)
    ));
    x += 50.0;
  }
}
```

### Creating Custom Palettes

You can create your own colour palettes using the `palette!` macro:

```rust
use bevy_color_palettes::palette;

palette!(MyGamePalette {
    "hero": (0.2, 0.6, 0.9),
    "enemy": (0.9, 0.2, 0.3),
    "background": (0.1, 0.1, 0.2),
    "highlight": (1.0, 0.8, 0.0),
});

// Now you can use your palette just like the built-in ones:
let hero_colour = MyGamePalette::HERO;
let enemy_colour = MyGamePalette::enemy();
let bg_colour = MyGamePalette::get("BaCKgrouND").unwrap();

// Iterate over all colours
for colour in &MyGamePalette {
    // Use colour...
}
```
