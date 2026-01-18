use bevy_color_palettes::palette;

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
	assert_eq!(TestPalette::RED, bevy::color::Color::srgb(1.0, 0.0, 0.0));
	assert_eq!(TestPalette::GREEN, bevy::color::Color::srgb(0.0, 1.0, 0.0));
	assert_eq!(TestPalette::BLUE, bevy::color::Color::srgb(0.0, 0.0, 1.0));
	assert_eq!(
		TestPalette::CUSTOM_COLOR,
		bevy::color::Color::srgb(0.5, 0.5, 0.5)
	);
}

#[cfg(feature = "bevy")]
#[test]
fn test_palette_methods() {
	// Test the static methods
	assert_eq!(TestPalette::red(), bevy::color::Color::srgb(1.0, 0.0, 0.0));
	assert_eq!(
		TestPalette::green(),
		bevy::color::Color::srgb(0.0, 1.0, 0.0)
	);
	assert_eq!(TestPalette::blue(), bevy::color::Color::srgb(0.0, 0.0, 1.0));
	assert_eq!(
		TestPalette::custom_color(),
		bevy::color::Color::srgb(0.5, 0.5, 0.5)
	);
}

#[cfg(feature = "bevy")]
#[test]
fn test_all_method() {
	// Test the all method
	let colors = TestPalette::all();
	assert_eq!(colors.len(), 4);
	assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));
}

#[cfg(feature = "bevy")]
#[test]
fn test_iter_method() {
	// Test the iter method
	let colors: Vec<Color> = TestPalette::iter().collect();
	assert_eq!(colors.len(), 4);
	assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));
}

#[cfg(feature = "bevy")]
#[test]
fn test_into_iter() {
	// Test IntoIterator for the struct
	let colors: Vec<Color> = TestPalette.into_iter().collect();
	assert_eq!(colors.len(), 4);
	assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));

	// Test IntoIterator for references to the struct
	let palette = TestPalette;
	let colors: Vec<Color> = (&palette).into_iter().collect();
	assert_eq!(colors.len(), 4);
	assert!(colors.contains(&bevy::color::Color::srgb(1.0, 0.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 1.0, 0.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.0, 0.0, 1.0)));
	assert!(colors.contains(&bevy::color::Color::srgb(0.5, 0.5, 0.5)));
}

#[cfg(feature = "bevy")]
#[test]
fn test_get() {
	// Test get method
	assert_eq!(
		TestPalette::get("red"),
		Some(bevy::color::Color::srgb(1.0, 0.0, 0.0))
	);
	assert_eq!(
		TestPalette::get("green"),
		Some(bevy::color::Color::srgb(0.0, 1.0, 0.0))
	);
	assert_eq!(
		TestPalette::get("blue"),
		Some(bevy::color::Color::srgb(0.0, 0.0, 1.0))
	);
	assert_eq!(
		TestPalette::get("customColor"),
		Some(bevy::color::Color::srgb(0.5, 0.5, 0.5))
	);
	assert_eq!(
		TestPalette::get("custom_color"),
		Some(bevy::color::Color::srgb(0.5, 0.5, 0.5))
	);
	assert_eq!(
		TestPalette::get("CUSTOM_COLOR"),
		Some(bevy::color::Color::srgb(0.5, 0.5, 0.5))
	);
	assert_eq!(TestPalette::get("nonexistent"), None);
}
