#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
	pub r8: u8,
	pub g8: u8,
	pub b8: u8,
	pub a8: u8,
}

impl Color {
	#[must_use]
	pub const fn new(r8: u8, g8: u8, b8: u8, a8: u8) -> Self {
		Self { r8, g8, b8, a8 }
	}

	#[must_use]
	pub const fn with_alpha_f32(&self, alpha: f32) -> Self {
		#[allow(clippy::cast_possible_truncation)]
		let a32 = (alpha * 255.0) as i32;

		#[allow(clippy::cast_possible_truncation)]
		#[allow(clippy::cast_sign_loss)]
		let a8 = if a32 <= 255 {
			if a32 >= 0 { a32 as u8 } else { 0 }
		} else {
			255
		};

		self.with_alpha_u8(a8)
	}

	#[must_use]
	pub const fn with_alpha_u8(&self, alpha: u8) -> Self {
		Self::new(self.r8, self.g8, self.b8, alpha)
	}
}

#[cfg(feature = "bevy")]
impl From<Color> for ::bevy::color::Color {
	fn from(c: Color) -> Self {
		::bevy::color::Color::srgba_u8(c.r8, c.g8, c.b8, c.a8)
	}
}

#[cfg(feature = "bevy")]
impl From<::bevy::color::Color> for Color {
	fn from(c: ::bevy::color::Color) -> Self {
		let srgba = c.to_srgba();
		let r = srgba.red;
		let g = srgba.green;
		let b = srgba.blue;
		let a = srgba.alpha;

		#[allow(clippy::cast_sign_loss)]
		let r32 = (r * 255.0) as i32;
		#[allow(clippy::cast_sign_loss)]
		let g32 = (g * 255.0) as i32;
		#[allow(clippy::cast_sign_loss)]
		let b32 = (b * 255.0) as i32;
		#[allow(clippy::cast_sign_loss)]
		let a32 = (a * 255.0) as i32;

		#[allow(clippy::cast_possible_truncation)]
		let r8 = if r32 <= 255 {
			if r32 >= 0 { r32 as u8 } else { 0 }
		} else {
			255
		};
		#[allow(clippy::cast_possible_truncation)]
		let g8 = if g32 <= 255 {
			if g32 >= 0 { g32 as u8 } else { 0 }
		} else {
			255
		};
		#[allow(clippy::cast_possible_truncation)]
		let b8 = if b32 <= 255 {
			if b32 >= 0 { b32 as u8 } else { 0 }
		} else {
			255
		};
		#[allow(clippy::cast_possible_truncation)]
		let a8 = if a32 <= 255 {
			if a32 >= 0 { a32 as u8 } else { 0 }
		} else {
			255
		};

		Color { r8, g8, b8, a8 }
	}
}

#[cfg(feature = "egui")]
impl From<Color> for ::egui::Color32 {
	fn from(c: Color) -> Self {
		::egui::Color32::from_rgba_unmultiplied(c.r8, c.g8, c.b8, c.a8)
	}
}

#[cfg(feature = "egui")]
impl From<::egui::Color32> for Color {
	fn from(c: ::egui::Color32) -> Self {
		let [r, g, b, a] = c.to_array();
		Color {
			r8: r,
			g8: g,
			b8: b,
			a8: a,
		}
	}
}
