//! Style properties.

use super::Color;

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// A style property.
#[derive(Clone, Debug)]
pub enum Property {
    /// The background color.
    BackgroundColor(Color),
    /// The color.
    Color(Color),
    // BorderColor(Color),
    // FontWeight(f32),
    // Height(f32),
    // Left(f32),
    // Margin(f32),
    // MarginBottom(f32),
    // MarginLeft(f32),
    // MarginRight(f32),
    // MarginTop(f32),
    // Opacity(f32),
    // Padding(f32),
    // PaddingBottom(f32),
    // PaddingLeft(f32),
    // PaddingRight(f32),
    // PaddingTop(f32),
    // Position(Position),
    // Right(f32),
    // Text(String),
    // Top(f32),
    // Width(f32),
    // ZIndex(f32),
}
