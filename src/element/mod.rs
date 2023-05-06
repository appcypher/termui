//! Elements are the building blocks of the user interface.

mod attribute;
mod elements;
mod r#macro;

pub use attribute::*;
pub use elements::*;

//----------------------------------------------------------------
// Traits
//----------------------------------------------------------------

/// An element is a widget that can be drawn on the terminal.
pub trait Element: std::fmt::Debug {
    /// Gets the base element.
    fn get_base(&self) -> &std::sync::Mutex<BaseElement>;
}
