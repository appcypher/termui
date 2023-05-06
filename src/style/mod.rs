//! Style is a collection of properties that can be applied to a widget.

mod color;
mod r#macro;
mod property;
mod selector;

pub use color::*;
pub use property::*;
pub use selector::*;

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// Style is a collection of properties that can be applied to a widget.
#[derive(Clone, Debug)]
pub struct Style {
    /// The list of selectors.
    selectors: Vec<selector::Selector>,
}

//----------------------------------------------------------------
// Trait Implementations
//----------------------------------------------------------------

impl FromIterator<Selector> for Style {
    fn from_iter<T: IntoIterator<Item = Selector>>(iter: T) -> Self {
        Self {
            selectors: iter.into_iter().collect(),
        }
    }
}
