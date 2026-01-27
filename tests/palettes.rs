#[allow(unused_imports)]
use bevy_color_palettes::{color::Color, palette};

palette!(TestPalette {
	"red": (1.0, 0.0, 0.0),
	"green": (0.0, 1.0, 0.0),
	"blue": (0.0, 0.0, 1.0),
	"customColor": (0.5, 0.5, 0.5),
});

#[cfg(feature = "bevy")]
#[test]
fn test_palette_constants() {
	// Test the constants
	assert_eq!(
		TestPalette::RED,
		Color::from(bevy::color::Color::srgb(1.0, 0.0, 0.0))
	);
	assert_eq!(
		TestPalette::GREEN,
		Color::from(bevy::color::Color::srgb(0.0, 1.0, 0.0))
	);
	assert_eq!(
		TestPalette::BLUE,
		Color::from(bevy::color::Color::srgb(0.0, 0.0, 1.0))
	);
	assert_eq!(
		TestPalette::CUSTOM_COLOR,
		Color::from(bevy::color::Color::srgb(0.5, 0.5, 0.5))
	);
}

#[cfg(feature = "bevy")]
#[test]
fn test_palette_methods() {
	// Test the static methods
	assert_eq!(
		TestPalette::red(),
		Color::from(bevy::color::Color::srgb(1.0, 0.0, 0.0))
	);
	assert_eq!(
		TestPalette::green(),
		Color::from(bevy::color::Color::srgb(0.0, 1.0, 0.0))
	);
	assert_eq!(
		TestPalette::blue(),
		Color::from(bevy::color::Color::srgb(0.0, 0.0, 1.0))
	);
	assert_eq!(
		TestPalette::custom_color(),
		Color::from(bevy::color::Color::srgb(0.5, 0.5, 0.5))
	);
}

#[cfg(feature = "bevy")]
#[test]
fn test_all_method() {
	// Test the all method
	let colors = TestPalette::all();
	assert_eq!(colors.len(), 4);
	//assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));
}

#[cfg(feature = "bevy")]
#[test]
fn test_iter_method() {
	// Test the iter method
	let colors: Vec<Color> = TestPalette::iter().collect();
	assert_eq!(colors.len(), 4);
	//assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));
}

#[cfg(feature = "bevy")]
#[test]
fn test_into_iter() {
	// Test IntoIterator for the struct
	let colors: Vec<Color> = TestPalette.into_iter().collect();
	assert_eq!(colors.len(), 4);
	//assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));

	// Test IntoIterator for references to the struct
	let palette = TestPalette;
	let colors: Vec<Color> = (&palette).into_iter().collect();
	assert_eq!(colors.len(), 4);
	//assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	//assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));
}

#[cfg(feature = "bevy")]
#[test]
fn test_get() {
	// Test get method
	assert_eq!(
		TestPalette::get("red"),
		Some(Color::from(bevy::color::Color::srgb(1.0, 0.0, 0.0)))
	);
	assert_eq!(
		TestPalette::get("green"),
		Some(Color::from(bevy::color::Color::srgb(0.0, 1.0, 0.0)))
	);
	assert_eq!(
		TestPalette::get("blue"),
		Some(Color::from(bevy::color::Color::srgb(0.0, 0.0, 1.0)))
	);
	assert_eq!(
		TestPalette::get("customColor"),
		Some(Color::from(bevy::color::Color::srgb(0.5, 0.5, 0.5)))
	);
	assert_eq!(
		TestPalette::get("custom_color"),
		Some(Color::from(bevy::color::Color::srgb(0.5, 0.5, 0.5)))
	);
	assert_eq!(
		TestPalette::get("CUSTOM_COLOR"),
		Some(Color::from(bevy::color::Color::srgb(0.5, 0.5, 0.5)))
	);
	assert_eq!(TestPalette::get("nonexistent"), None);
}
