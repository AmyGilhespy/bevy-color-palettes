#[cfg(feature = "parse")]
use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
	pub r8: u8,
	pub g8: u8,
	pub b8: u8,
	pub a8: u8,
	intensity16: u16,
}

impl Color {
	#[must_use]
	pub const fn new(r8: u8, g8: u8, b8: u8, a8: u8) -> Self {
		Self {
			r8,
			g8,
			b8,
			a8,
			intensity16: 256_u16,
		}
	}

	/// # Errors
	/// - `Error::ParseError` - Parse format errors.
	#[cfg(feature = "parse")]
	pub fn try_parse(string: &str) -> Result<Self, Error> {
		let Some(mut hex) = string.strip_prefix('#') else {
			return Err(Error::ParseError(
				"HTML hex color string must start with '#'.".into(),
			)); // So we can support named colors in the future.
		};

		let mut i16 = 256_u16;
		if hex.as_bytes().get(hex.len() - 5) == Some(&b'+') {
			i16 = u16::from_str_radix(&hex[(hex.len() - 4)..], 16).map_err(|err| {
				Error::ParseError(format!("Error parsing intensity portion: {err}"))
			})?;
			hex = &hex[0..(hex.len() - 5)];
		}

		let (r, g, b, a) = match hex.len() {
			8 => {
				let r = u8::from_str_radix(&hex[0..2], 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #RRggbbaa portion: {err}"))
				})?;
				let g = u8::from_str_radix(&hex[2..4], 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rrGGbbaa portion: {err}"))
				})?;
				let b = u8::from_str_radix(&hex[4..6], 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rrggBBaa portion: {err}"))
				})?;
				let a = u8::from_str_radix(&hex[6..8], 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rrggbbAA portion: {err}"))
				})?;
				(r, g, b, a)
			}
			6 => {
				let r = u8::from_str_radix(&hex[0..2], 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #RRggbb portion: {err}"))
				})?;
				let g = u8::from_str_radix(&hex[2..4], 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rrGGbb portion: {err}"))
				})?;
				let b = u8::from_str_radix(&hex[4..6], 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rrggBB portion: {err}"))
				})?;
				(r, g, b, 255_u8)
			}
			4 => {
				let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #Rgba portion: {err}"))
				})?;
				let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rGba portion: {err}"))
				})?;
				let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rgBa portion: {err}"))
				})?;
				let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rgbA portion: {err}"))
				})?;
				(r, g, b, a)
			}
			3 => {
				let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #Rgb portion: {err}"))
				})?;
				let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rGb portion: {err}"))
				})?;
				let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|err| {
					Error::ParseError(format!("Error parsing #rgB portion: {err}"))
				})?;
				(r, g, b, 255_u8)
			}
			_ => {
				return Err(Error::ParseError(
					"Hex color must be in #rrggbb, #rrggbbaa, #rgb, or #rgba format.".into(),
				));
			}
		};

		Ok(Color::new(r, g, b, a).with_intensity_u16_experimental(i16))
	}

	#[must_use]
	pub const fn with_alpha_f32(&self, alpha: f32) -> Self {
		#[allow(clippy::cast_possible_truncation)]
		let a32 = (alpha * 255.0) as i32;

		#[allow(clippy::cast_possible_truncation)]
		#[allow(clippy::cast_sign_loss)]
		let a8 = if a32 <= 255 {
			if a32 >= 0 { a32 as u8 } else { 0_u8 }
		} else {
			255_u8
		};

		self.with_alpha_u8(a8)
	}

	#[must_use]
	pub const fn with_alpha_u8(&self, alpha: u8) -> Self {
		Self::new(self.r8, self.g8, self.b8, alpha)
	}

	#[must_use]
	pub const fn with_intensity_f32_experimental(&self, intensity: f32) -> Self {
		#[allow(clippy::cast_possible_truncation)]
		let i32 = (intensity * 256.0) as i32;

		#[allow(clippy::cast_possible_truncation)]
		#[allow(clippy::cast_sign_loss)]
		let i16 = if i32 <= 256 * 255 {
			if i32 >= 0 { i32 as u16 } else { 0_u16 }
		} else {
			256_u16
		};

		self.with_intensity_u16_experimental(i16)
	}

	#[must_use]
	pub const fn with_intensity_u16_experimental(&self, intensity: u16) -> Self {
		Self {
			r8: self.r8,
			g8: self.g8,
			b8: self.b8,
			a8: self.a8,
			intensity16: intensity,
		}
	}
}

#[cfg(feature = "bevy")]
impl From<Color> for ::bevy::color::Color {
	fn from(c: Color) -> Self {
		if c.intensity16 == 256 {
			::bevy::color::Color::srgba_u8(c.r8, c.g8, c.b8, c.a8)
		} else {
			#[allow(clippy::cast_precision_loss)]
			let ri32 = (u32::from(c.r8) * u32::from(c.intensity16)) as f32 / 65280.0;
			#[allow(clippy::cast_precision_loss)]
			let gi32 = (u32::from(c.g8) * u32::from(c.intensity16)) as f32 / 65280.0;
			#[allow(clippy::cast_precision_loss)]
			let bi32 = (u32::from(c.b8) * u32::from(c.intensity16)) as f32 / 65280.0;
			let a32 = f32::from(c.a8) / 255.0;
			::bevy::color::Color::srgba(ri32, gi32, bi32, a32)
		}
	}
}

#[cfg(feature = "bevy")]
impl From<::bevy::color::Color> for Color {
	fn from(bevy_color: ::bevy::color::Color) -> Self {
		let srgba = bevy_color.to_srgba();
		let r = srgba.red;
		let g = srgba.green;
		let b = srgba.blue;
		let a = srgba.alpha;

		#[allow(clippy::cast_sign_loss)]
		#[allow(clippy::cast_possible_truncation)]
		let r32 = (r * 255.0) as i32;
		#[allow(clippy::cast_sign_loss)]
		#[allow(clippy::cast_possible_truncation)]
		let g32 = (g * 255.0) as i32;
		#[allow(clippy::cast_sign_loss)]
		#[allow(clippy::cast_possible_truncation)]
		let b32 = (b * 255.0) as i32;
		#[allow(clippy::cast_sign_loss)]
		#[allow(clippy::cast_possible_truncation)]
		let a32 = (a * 255.0) as i32;

		#[allow(clippy::cast_possible_truncation)]
		let r8 = if r32 <= 255 {
			#[allow(clippy::cast_sign_loss)]
			if r32 >= 0 { r32 as u8 } else { 0 }
		} else {
			255
		};
		#[allow(clippy::cast_possible_truncation)]
		let g8 = if g32 <= 255 {
			#[allow(clippy::cast_sign_loss)]
			if g32 >= 0 { g32 as u8 } else { 0 }
		} else {
			255
		};
		#[allow(clippy::cast_possible_truncation)]
		let b8 = if b32 <= 255 {
			#[allow(clippy::cast_sign_loss)]
			if b32 >= 0 { b32 as u8 } else { 0 }
		} else {
			255
		};
		#[allow(clippy::cast_possible_truncation)]
		let a8 = if a32 <= 255 {
			#[allow(clippy::cast_sign_loss)]
			if a32 >= 0 { a32 as u8 } else { 0 }
		} else {
			255
		};

		Color::new(r8, g8, b8, a8)
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
	fn from(egui_color: ::egui::Color32) -> Self {
		let [r, g, b, a] = egui_color.to_array();
		Color::new(r, g, b, a)
	}
}
