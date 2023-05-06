//! A selector is a pattern that can be used to match an element.

use super::property::Property;

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

#[derive(Clone, Debug)]
/// A selector is a pattern that can be used to match an element.
pub enum Selector {
    /// A selector that matches a widget by its tag.
    Tag(String, Vec<Property>),
    /// A selector that matches a widget by its id.
    Id(String, Vec<Property>),
    /// A selector that matches a widget by its class.
    Class(String, Vec<Property>),
    // /// A selector that matches a widget by its state.
    // State(StateSelector),
    // /// A selector that matches a widget by its parent.
    // Parent(ParentSelector),
    // /// A selector that matches a widget by its child.
    // Child(ChildSelector),
    // /// A selector that matches a widget by its sibling.
    // Sibling(SiblingSelector),
    // /// A selector that matches a widget by its descendant.
    // Descendant(DescendantSelector),
}
