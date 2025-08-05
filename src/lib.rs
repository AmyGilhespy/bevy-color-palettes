 //! A collection of popular color palettes, with utilities interacting with them.
 //! ## Creating palettes
 //!
 //! The palette! macro is available for generating additional colour palettes with
 //! convenience methods:
 //!
 //! ```rust
 //! use weirdboi_bevy_colour::palette;
 //!
 //!
 //! palette!(MyNewPalette {
 //!   "someColor": (0.5, 0.5, 0.5),
 //!   "anotherColor": (0.75, 0.23, 0.0),
 //! });
 //!
 //! // Access by constant:
 //! MyNewPalette::SOME_COLOR;
 //!
 //! // Access by method:
 //! MyNewPalette::some_color();
 //!
 //! // Look up colour by case and format insensitive name:
 //! assert_eq!(MyNewPalette::get("someColor"), MyNewPalette::get("SOME_COLOR"))
 //! ```
 //!
 //! ## Using existing palettes
 //!
 //! `weirdboi_bevy_colour` provides a collection of popular colour palettes, including:
 //! - [Nanner Pancakes](https://lospec.com/palette-list/nanner-pancakes)
 //! - [Resurrect 64](https://lospec.com/palette-list/ressurect-64)
 //! - [Resurrect 32](https://lospec.com/palette-list/resurrect-32)
 //! - [Dawnbringer 16](https://lospec.com/palette-list/dawnbringer-16)
 //! - [Dawnbringer 32](https://lospec.com/palette-list/dawnbringer-32)
 //!
 //!
 //! ## What is each colour?
 //!
 //! If you want to see a preview of each colour, check the rustdoc for each palette. The doc for
 //! the palette will contain a grid of available colours, and each colour constant will contain
 //! a colour bar.
 //!

pub use macros::palette;

mod resurrect;
mod nanner;
mod dawnbringer;

pub use resurrect::{Resurrect32, Resurrect64};
pub use nanner::NannerPancakes;
pub use dawnbringer::{Dawnbringer16, Dawnbringer32};