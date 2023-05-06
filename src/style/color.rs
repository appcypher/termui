//! The different supported colors in the library.

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// The different supported colors in the library.
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    /// A color with a red, green, and blue component.
    Rgb(u8, u8, u8),
    /// A color with a hue, saturation, and lightness component.
    Hsl(u8, u8, u8),
}
