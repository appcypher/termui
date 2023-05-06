//! Attributes for elements

use crossterm::event::KeyEvent;

use crate::handler::Handler;

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// An attribute for an element.
#[derive(Debug, Clone)]
pub enum Attribute {
    /// A class attribute.
    Class(String),
    /// An id attribute.
    Id(String),
    /// On keybord event attribute.
    OnKeyEvent(Handler<KeyEvent>),
}
