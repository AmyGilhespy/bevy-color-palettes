//! Procedural macros for bevy-color-palettes

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::braced;
use syn::token::{Colon, Comma};
use syn::{
	Ident, LitStr, Result, parenthesized,
	parse::{Parse, ParseStream},
	parse_macro_input,
};

/// Get the crate root (rename-safe)
fn crate_root() -> proc_macro2::TokenStream {
	match crate_name("bevy-color-palettes") {
		Ok(FoundCrate::Itself) => quote!(crate),
		Ok(FoundCrate::Name(name)) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(::#ident)
		}
		Err(_) => quote!(::bevy_color_palettes),
	}
}

/// A color definition with a name and RGBA values
struct ColorDef {
	name: String,
	r8: u8,
	g8: u8,
	b8: u8,
	a8: u8,
}

/// A palette definition with a name and a list of color definitions
struct PaletteDef {
	name: Ident,
	colors: Vec<ColorDef>,
}

/// Parse a color definition from a stream
impl Parse for ColorDef {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the color name as a string literal
		let name_lit = input.parse::<LitStr>()?;
		let name = name_lit.value();

		// Parse the colon
		input.parse::<Colon>()?;

		// Check if the next token is a string
		if input.peek(syn::LitStr) {
			let lit = input.parse::<LitStr>()?;
			let (r8, g8, b8, a8) = parse_hex_color(&lit.value(), lit.span())?;
			Ok(ColorDef {
				name,
				r8,
				g8,
				b8,
				a8,
			})
		} else {
			// Otherwise, fall back to the version in weirdboi_bevy_colour (so we can merge upstream palettes with no issue)
			let content;
			parenthesized!(content in input);

			let r: f32 = content.parse::<syn::LitFloat>()?.base10_parse()?;
			content.parse::<Comma>()?;
			let g: f32 = content.parse::<syn::LitFloat>()?.base10_parse()?;
			content.parse::<Comma>()?;
			let b: f32 = content.parse::<syn::LitFloat>()?.base10_parse()?;

			#[allow(clippy::cast_possible_truncation)]
			let r32 = (r * 255.0) as i32;
			#[allow(clippy::cast_possible_truncation)]
			let g32 = (g * 255.0) as i32;
			#[allow(clippy::cast_possible_truncation)]
			let b32 = (b * 255.0) as i32;

			let r8 = if r32 <= 255 {
				#[allow(clippy::cast_possible_truncation)]
				#[allow(clippy::cast_sign_loss)]
				if r32 >= 0 { r32 as u8 } else { 0 }
			} else {
				255
			};
			let g8 = if g32 <= 255 {
				#[allow(clippy::cast_possible_truncation)]
				#[allow(clippy::cast_sign_loss)]
				if g32 >= 0 { g32 as u8 } else { 0 }
			} else {
				255
			};
			let b8 = if b32 <= 255 {
				#[allow(clippy::cast_possible_truncation)]
				#[allow(clippy::cast_sign_loss)]
				if b32 >= 0 { b32 as u8 } else { 0 }
			} else {
				255
			};

			let a8 = 255;

			Ok(ColorDef {
				name,
				r8,
				g8,
				b8,
				a8,
			})
		}
	}
}

fn parse_hex_color(html_hex_color_string: &str, span: Span) -> Result<(u8, u8, u8, u8)> {
	let hex = html_hex_color_string
		.strip_prefix('#')
		.ok_or_else(|| syn::Error::new(span, "HTML hex color string must start with '#'."))?; // So we can support named colors in the future.

	let (r, g, b, a) = match hex.len() {
		8 => {
			let r = u8::from_str_radix(&hex[0..2], 16)
				.map_err(|_| syn::Error::new(span, "#RRggbbaa RR was invalid."))?;
			let g = u8::from_str_radix(&hex[2..4], 16)
				.map_err(|_| syn::Error::new(span, "#rrGGbbaa GG was invalid."))?;
			let b = u8::from_str_radix(&hex[4..6], 16)
				.map_err(|_| syn::Error::new(span, "#rrggBBaa BB was invalid."))?;
			let a = u8::from_str_radix(&hex[6..8], 16)
				.map_err(|_| syn::Error::new(span, "#rrggbbAA AA was invalid."))?;
			(r, g, b, a)
		}
		6 => {
			let r = u8::from_str_radix(&hex[0..2], 16)
				.map_err(|_| syn::Error::new(span, "#RRggbb RR was invalid."))?;
			let g = u8::from_str_radix(&hex[2..4], 16)
				.map_err(|_| syn::Error::new(span, "#rrGGbb GG was invalid."))?;
			let b = u8::from_str_radix(&hex[4..6], 16)
				.map_err(|_| syn::Error::new(span, "#rrggBB BB was invalid."))?;
			(r, g, b, 255_u8)
		}
		4 => {
			let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
				.map_err(|_| syn::Error::new(span, "#Rgba R was invalid."))?;
			let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
				.map_err(|_| syn::Error::new(span, "#rGba G was invalid."))?;
			let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
				.map_err(|_| syn::Error::new(span, "#rgBa B was invalid."))?;
			let a = u8::from_str_radix(&hex[3..4].repeat(2), 16)
				.map_err(|_| syn::Error::new(span, "#rgbA A was invalid."))?;
			(r, g, b, a)
		}
		3 => {
			let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
				.map_err(|_| syn::Error::new(span, "#Rgb R was invalid."))?;
			let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
				.map_err(|_| syn::Error::new(span, "#rGb G was invalid."))?;
			let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
				.map_err(|_| syn::Error::new(span, "#rgB B was invalid."))?;
			(r, g, b, 255_u8)
		}
		_ => {
			return Err(syn::Error::new(
				span,
				"Hex color must be in #rrggbb, #rrggbbaa, #rgb, or #rgba format.",
			));
		}
	};

	Ok((r, g, b, a))
}

/// Parse a palette definition from a stream
impl Parse for PaletteDef {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the palette name as an identifier
		let name = input.parse::<Ident>()?;

		// Parse the color definitions inside braces
		let content;
		braced!(content in input);

		// Parse the color definitions
		let mut colors = Vec::new();
		while !content.is_empty() {
			colors.push(content.parse::<ColorDef>()?);

			// Parse the comma if there is one and we're not at the end
			if content.peek(Comma) {
				content.parse::<Comma>()?;
			} else if !content.is_empty() {
				return Err(content.error("Expected comma or end of block."));
			}
		}

		Ok(PaletteDef { name, colors })
	}
}

/// Convert a string to `UPPER_SNAKE_CASE`
fn to_upper_snake_case(s: &str) -> String {
	let mut result = String::new();
	for (i, c) in s.chars().enumerate() {
		if c.is_uppercase() && i > 0 && !s.chars().nth(i - 1).unwrap_or(' ').is_uppercase() {
			result.push('_');
		}
		result.push(c.to_ascii_uppercase());
	}
	result
}

/// Convert a string to `lower_snake_case`
fn to_lower_snake_case(s: &str) -> String {
	let mut result = String::new();
	for (i, c) in s.chars().enumerate() {
		if c.is_uppercase() {
			if i > 0 && !s.chars().nth(i - 1).unwrap_or(' ').is_uppercase() {
				result.push('_');
			}
			result.push(c.to_ascii_lowercase());
		} else {
			result.push(c);
		}
	}
	result
}

/// Generate a palette struct and implementation
///
/// # Example
///
/// ```ignore
/// use macros::palette;
/// palette!(MyPalette {
///     "red": "#ff0000",
///     "green": "#00ff00",
///     "blue": "#0000ff",
/// });
/// ```
///
/// This will generate:
///
/// ```ignore
/// pub struct MyPalette;
///
/// impl MyPalette {
///     /// RED; <div style="background-color: rgb(100% 0% 0% 100%); height: 20px"></div>
///     pub const RED: Color = Color::new(255, 0, 0, 255);
///     /// GREEN; <div style="background-color: rgb(0% 100% 0% 100%); height: 20px"></div>
///     pub const GREEN: Color = Color::new(0, 255, 0, 255);
///     /// BLUE; <div style="background-color: rgb(0% 0% 100% 100%); height: 20px"></div>
///     pub const BLUE: Color = Color::new(0, 0, 255, 255);
///
///     pub fn red(&self) -> Color { Self::RED }
///     pub fn green(&self) -> Color { Self::GREEN }
///     pub fn blue(&self) -> Color { Self::BLUE }
/// }
///
/// impl Palette for MyPalette {
///     // Implementation of Palette trait
/// }
/// ```
#[allow(clippy::too_many_lines)]
#[proc_macro]
pub fn palette(input: TokenStream) -> TokenStream {
	// Parse the input
	let palette_def = parse_macro_input!(input as PaletteDef);

	// Generate the struct definition
	let palette_name = &palette_def.name;
	let crate_root = crate_root();
	let crate_color = quote! { #crate_root::color::Color };

	// Generate the color constants and methods
	let mut const_defs = Vec::new();
	let mut method_defs = Vec::new();
	let mut get_color_match_arms = Vec::new();
	let mut color_values = Vec::new();
	let mut doc_grid_entry = Vec::new();
	let mut color_rgba = Vec::new();

	for color in &palette_def.colors {
		let color_name = &color.name;
		let normalised = normalize_color_name(color_name);
		let const_name = Ident::new(&to_upper_snake_case(color_name), Span::call_site());
		let method_name = format_ident!("{}", to_lower_snake_case(color_name));

		let r8 = color.r8;
		let g8 = color.g8;
		let b8 = color.b8;
		let a8 = color.a8;

		let current_rgba = format!(
			"rgba({:.0}%, {:.0}%, {:.0}%, {:.2})",
			f32::from(r8) * (100.0 / 255.0),
			f32::from(g8) * (100.0 / 255.0),
			f32::from(b8) * (100.0 / 255.0),
			f32::from(a8) / 255.0,
		);

		let rustdoc =
			format!(r#"<div style="background-color: {current_rgba}; height: 20px"></div>"#,);

		let funcdoc =
			format!(r"Returns the value of [{palette_name}::{const_name}]<br/>{rustdoc}",);
		color_rgba.push(current_rgba);

		// Add the constant definition
		const_defs.push(quote! {
			#[doc = #rustdoc]
			pub const #const_name: #crate_color = #crate_color::new(#r8, #g8, #b8, #a8);
		});

		// Add the method definition (static, no &self)
		method_defs.push(quote! {
			#[doc = #funcdoc]
			pub const fn #method_name() -> #crate_color {
				Self::#const_name
			}
		});

		// Add the match arm for get_color
		get_color_match_arms.push(quote! {
			#normalised => Some(Self::#const_name),
		});

		// Add the color value for the iterator
		color_values.push(quote! {
			Self::#const_name,
		});

		doc_grid_entry.push(format!(
			r#"<div style="background-color: rgba({:.0}% {:.0}% {:.0}% {:.2}); width: 20px; height: 20px;"></div>"#,
			f32::from(r8) * (100.0 / 255.0),
			f32::from(g8) * (100.0 / 255.0),
			f32::from(b8) * (100.0 / 255.0),
			f32::from(a8) / 255.0,
		));
	}

	// Get the number of colors
	let num_colors = palette_def.colors.len();
	let num_colors_lit = proc_macro2::Literal::usize_unsuffixed(num_colors);
	let iter_type = quote! { ::core::array::IntoIter<#crate_color, #num_colors_lit> };

	let root_doc = format!(
		r#"<span>The {palette_name} palette, containing {num_colors} colors.</span> <br />
		<div style="display: grid; grid-template-columns: repeat(8, 20px); grid-auto-rows: 20px;">{}</div>"#,
		doc_grid_entry.join("\n")
	);

	// Generate the final code
	let expanded = quote! {
		#[doc = #root_doc]
		#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
		pub struct #palette_name;

		impl #palette_name {
			#(#const_defs)*

			#(#method_defs)*

			// Helper function to normalize color names for case-insensitive and format-agnostic comparison
			#[doc(hidden)]
			fn normalize_color_name(s: &str) -> String {
				s.chars()
					.filter(|c| c.is_alphanumeric())
					.map(|c| c.to_ascii_lowercase())
					.collect()
			}

			/// Returns all colors in the palette as a fixed-size array
			pub const fn all() -> [#crate_color; #num_colors_lit] {
				[#(#color_values)*]
			}

			/// Returns the number of colours in the palette
			pub const fn len() -> usize {
				#num_colors_lit
			}

			/// Returns an iterator over all colors in the palette
			pub fn iter() -> impl Iterator<Item = #crate_color> {
				Self::all().into_iter()
			}

			/// Returns a color by name, if it exists in the palette
			pub fn get(name: &str) -> Option<#crate_color> {
				let name = Self::normalize_color_name(name);
				match name.as_str() {
					#(#get_color_match_arms)*
					_ => None,
				}
			}
		}

		impl IntoIterator for #palette_name {
			type Item = #crate_color;
			type IntoIter = #iter_type;

			fn into_iter(self) -> Self::IntoIter {
				Self::all().into_iter()
			}
		}

		impl<'a> IntoIterator for &'a #palette_name {
			type Item = #crate_color;
			type IntoIter = #iter_type;

			fn into_iter(self) -> Self::IntoIter {
				#palette_name::all().into_iter()
			}
		}
	};

	// Return the generated code
	expanded.into()
}

fn normalize_color_name(s: &str) -> String {
	s.chars()
		.filter(|c| c.is_alphanumeric())
		.map(|c| c.to_ascii_lowercase())
		.collect()
}
