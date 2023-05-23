//! The reactive components of for elements.

mod state;

pub use state::*;

//----------------------------------------------------------------
// Traits
//----------------------------------------------------------------

/// An observable is a type that can be observed by subscribers.
pub trait Observable<E> {
    /// Adds a subscriber to the observable.
    fn add_subscriber(
        &self,
        callback: std::sync::Arc<dyn Fn(&E) -> anyhow::Result<()> + 'static>,
    ) -> usize;

    /// Removes a subscriber from the observable.
    fn remove_subscriber(&mut self, id: usize);

    /// Notifies the subscribers of the observable.
    fn notify(&self) -> anyhow::Result<()>;
}
