//! Handler for events.

use anyhow::Result;
use std::{fmt::Debug, sync::Arc};

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// Handler for events.
#[derive(Clone)]
pub struct Handler<E>(Arc<dyn Fn(&E) -> Result<()>>);

//----------------------------------------------------------------
// Methods
//----------------------------------------------------------------

impl<E> Handler<E> {
    /// Create a new handler.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&E) -> Result<()> + 'static,
    {
        Self(Arc::new(f))
    }

    /// Call the handler.
    #[inline]
    pub fn call(&self, event: &E) -> Result<()> {
        (self.0)(event)
    }
}

//----------------------------------------------------------------
// Trait Implementations
//----------------------------------------------------------------

impl<E> Debug for Handler<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handler").finish()
    }
}
