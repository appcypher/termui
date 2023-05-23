//! Geometrics.

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// A size is a width and height.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Size {
    /// The width.
    pub width: u16,
    /// The height.
    pub height: u16,
}

/// A point is an x and y.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Point {
    /// The x.
    pub x: u16,
    /// The y.
    pub y: u16,
}

//----------------------------------------------------------------
// Methods
//----------------------------------------------------------------

impl Size {
    /// Create a new `Size` with the given width and height.
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Point {
    /// Create a new `Point` with the given x and y.
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

//----------------------------------------------------------------
// Trait Implementations
//----------------------------------------------------------------

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Self { width, height }
    }
}

impl From<(u16, u16)> for Point {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}
