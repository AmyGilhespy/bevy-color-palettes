//! A collection of popular color palettes, with utilities interacting with them.
//!
//! ## Using palettes
//!
//! The recommended way to use this crate is to import it with an `as` rename:
//!
//! ```ignore
//! use bevy_color_palettes as pal;
//!
//! let transparent_white: bevy::color::Color = pal::Common::TRANSPARENT_WHITE.into();
//! let red: bevy::color::Color = pal::aseprite::GoogleUi::RED_500.into();
//! ```
//!
//! ## Creating palettes
//!
//! The palette! macro is available for generating additional colour palettes with
//! convenience methods:
//!
//! ```ignore
//! use bevy_color_palettes::palette;
//!
//!
//! palette!(MyNewPalette {
//!   "someColor": "#7f7f7f",
//!   "anotherColor": "#bf3b00",
//! });
//!
//! // Access by constant:
//! MyNewPalette::SOME_COLOR;
//!
//! // Access by method:
//! MyNewPalette::some_color();
//!
//! // Look up colour by case and format insensitive name:
//! assert_eq!(MyNewPalette::get("someColor"), MyNewPalette::get("SOME_COLOR"));
//!
//! // Iterate over all the colours in a palette:
//! for color in &MyNewPalette {
//!    // color is a color::Color instance.
//!    // with the "bevy" feature flag, color can be converted to ::bevy::color::Color
//!    // with the "egui" feature flag, color can be converted to ::egui::Color32
//!    println!("{:?}", color);
//! }
//! ```
//!
//! ## Using existing palettes
//!
//! `bevy_color_palettes` provides a collection of popular colour palettes, including:
//! - [Nanner Pancakes](https://lospec.com/palette-list/nanner-pancakes)
//! - [Resurrect 64](https://lospec.com/palette-list/resurrect-64)
//! - [Resurrect 32](https://lospec.com/palette-list/resurrect-32)
//! - [Dawnbringer 16](https://lospec.com/palette-list/dawnbringer-16)
//! - [Dawnbringer 32](https://lospec.com/palette-list/dawnbringer-32)
//!
//! ## What is each color?
//!
//! If you want to see a preview of each color, check the rustdoc for each palette. The doc for
//! the palette will contain a grid of available colors, and each color constant will contain
//! a color bar.

pub use macros::palette;

pub mod aseprite;
pub mod bevy;
pub mod color;
mod common;
mod dawnbringer;
#[cfg(feature = "parse")]
pub mod error;
mod nanner;
mod resurrect;

pub use common::Common;
pub use dawnbringer::{Dawnbringer16, Dawnbringer32};
pub use nanner::NannerPancakes;
pub use resurrect::{Resurrect32, Resurrect64};
