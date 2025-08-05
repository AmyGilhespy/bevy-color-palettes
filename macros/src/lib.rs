//! Procedural macros for weirdboi_bevy_colour

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::braced;
use syn::token::{Colon, Comma};
use syn::{
	Ident, LitStr, Result, parenthesized,
	parse::{Parse, ParseStream},
	parse_macro_input,
};

/// A color definition with a name and RGB values
struct ColorDef {
	name: String,
	r: f32,
	g: f32,
	b: f32,
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

		// Parse the RGB tuple (r, g, b)
		let content;
		parenthesized!(content in input);

		let r = content.parse::<syn::LitFloat>()?.base10_parse()?;
		content.parse::<Comma>()?;
		let g = content.parse::<syn::LitFloat>()?.base10_parse()?;
		content.parse::<Comma>()?;
		let b = content.parse::<syn::LitFloat>()?.base10_parse()?;

		Ok(ColorDef { name, r, g, b })
	}
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
				return Err(content.error("expected comma or end of block"));
			}
		}

		Ok(PaletteDef { name, colors })
	}
}

/// Convert a string to UPPER_SNAKE_CASE
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

/// Convert a string to lower_snake_case
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
/// ```
/// use macros::palette;
/// palette!(MyPalette {
///     "red": (1.0, 0.0, 0.0),
///     "green": (0.0, 1.0, 0.0),
///     "blue": (0.0, 0.0, 1.0),
/// });
/// ```
///
/// This will generate:
///
/// ```no_run
/// pub struct MyPalette;
///
/// impl MyPalette {
///     /// RED; <div style="background-color: rgb(100% 0% 0%);" height: 20px"></div>
///     pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
///     /// GREEN; <div style="background-color: rgb(0% 100% 0%);" height: 20px"></div>
///     pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
///     /// BLUE; <div style="background-color: rgb(0% 0% 100%);" height: 20px"></div>
///     pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
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
#[proc_macro]
pub fn palette(input: TokenStream) -> TokenStream {
	// Parse the input
	let palette_def = parse_macro_input!(input as PaletteDef);

	// Generate the struct definition
	let palette_name = &palette_def.name;
	let bevy_color = quote! { ::bevy::color::Color };

	// Generate the color constants and methods
	let mut const_defs = Vec::new();
	let mut method_defs = Vec::new();
	let mut get_color_match_arms = Vec::new();
	let mut from_str_match_arms = Vec::new();
	let mut color_values = Vec::new();

	for color in &palette_def.colors {
		let color_name = &color.name;
		let const_name = Ident::new(&to_upper_snake_case(color_name), Span::call_site());
		let method_name = format_ident!("{}", to_lower_snake_case(color_name));

		let r = color.r;
		let g = color.g;
		let b = color.b;

		let rustdoc = format!(
			r#"{}; <div style="background-color: rgb({:.0}% {:.0}% {:.0}%);" height: 20px"></div>"#,
			const_name,
			r * 100.0,
			g * 100.0,
			b * 100.0
		);

		// Add the constant definition
		const_defs.push(quote! {
            #[doc = #rustdoc]
			pub const #const_name: #bevy_color = #bevy_color::srgb(#r, #g, #b);
		});

		// Add the method definition (static, no &self)
		method_defs.push(quote! {
			pub fn #method_name() -> #bevy_color {
				Self::#const_name
			}
		});

		// Add the match arm for get_color
		get_color_match_arms.push(quote! {
			#color_name => Some(Self::#const_name),
		});

		// Add the match arm for from_str
		from_str_match_arms.push(quote! {
			#color_name => Ok(Self::#const_name),
		});

		// Add the color value for the iterator
		color_values.push(quote! {
			Self::#const_name,
		});
	}


	// Get the number of colors
	let num_colors = palette_def.colors.len();
	let num_colors_lit = proc_macro2::Literal::usize_unsuffixed(num_colors);
	let iter_type = quote! { ::core::array::IntoIter<#bevy_color, #num_colors_lit> };

	// Generate the final code
	let expanded = quote! {
		/// The #palette_name palette
		#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
		pub struct #palette_name;

		impl #palette_name {
			#(#const_defs)*

			#(#method_defs)*

			/// Returns all colors in the palette as a fixed-size array
			pub fn all() -> [#bevy_color; #num_colors_lit] {
				[#(#color_values)*]
			}

			/// Returns an iterator over all colors in the palette
			pub fn iter() -> impl Iterator<Item = #bevy_color> {
				Self::all().into_iter()
			}

			/// Returns a color by name, if it exists in the palette
			pub fn get(name: &str) -> Option<#bevy_color> {
				match name {
                    #(#get_color_match_arms)*
                    _ => None,
                }
			}
		}

		impl IntoIterator for #palette_name {
			type Item = #bevy_color;
			type IntoIter = #iter_type;

			fn into_iter(self) -> Self::IntoIter {
				Self::all().into_iter()
			}
		}

		impl<'a> IntoIterator for &'a #palette_name {
			type Item = #bevy_color;
			type IntoIter = #iter_type;

			fn into_iter(self) -> Self::IntoIter {
				#palette_name::all().into_iter()
			}
		}
	};

	// Return the generated code
	expanded.into()
}
