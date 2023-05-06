//! This module contains the definitions of the elements.

use crate::style::Selector;

use super::{Attribute, Element};
use std::sync::{Arc, Mutex, Weak};

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// BaseElement is the base struct for all elements.
#[derive(Debug, Clone, Default)]
pub struct BaseElement {
    /// The attributes of the element.
    pub attributes: Vec<Attribute>,
    /// The children of the element.
    pub children: Vec<Arc<dyn Element>>,
    /// The content of the element.
    pub content: String,
    /// The parent of the element.
    pub parent: Option<Weak<dyn Element>>,
    /// The index of the element in the parent.
    pub index: u16,
    /// Selectors for the element.
    pub selectors: Vec<Arc<Selector>>,
}

/// Div is by default a basic block-level element.
#[derive(Debug)]
pub struct Div {
    base: Mutex<BaseElement>,
}

/// Span is a basic inline-level element.
#[derive(Debug)]
pub struct Span {
    base: Mutex<BaseElement>,
}

/// A paragraph element.
#[derive(Debug)]
pub struct P {
    base: Mutex<BaseElement>,
}

crate::create_element_builder_struct!(Div, DivBuilder);
crate::create_element_builder_struct!(Span, SpanBuilder);
crate::create_element_builder_struct!(P, PBuilder);

//----------------------------------------------------------------
// Methods
//----------------------------------------------------------------

crate::impl_element_methods!(Div);
crate::impl_element_methods!(Span);
crate::impl_element_methods!(P);

crate::impl_element_builder_methods!(Div, DivBuilder);
crate::impl_element_builder_methods!(Span, SpanBuilder);
crate::impl_element_builder_methods!(P, PBuilder);

//----------------------------------------------------------------
// Trait Implementations
//----------------------------------------------------------------

crate::impl_element_trait!(Div);
crate::impl_element_trait!(Span);
crate::impl_element_trait!(P);
