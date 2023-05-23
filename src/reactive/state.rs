//! Reactive state management.

use super::Observable;
use anyhow::Result;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// A reactive state.
pub struct State<T> {
    value: Mutex<T>,
    subscribers: Mutex<Vec<Arc<dyn Fn(&T) -> Result<()>>>>,
}

//----------------------------------------------------------------
// Methods
//----------------------------------------------------------------

impl<T> State<T> {
    /// Creates a new state.
    pub fn new(value: T) -> Self {
        Self {
            value: Mutex::new(value),
            subscribers: Mutex::new(Vec::new()),
        }
    }

    /// Sets the state value and notifies the subscribers of the change.
    pub fn set(&self, value: T) -> Result<()> {
        *self.value.lock().unwrap() = value;
        self.notify()
    }
}

//----------------------------------------------------------------
// Trait Implementations
//----------------------------------------------------------------

impl<T> Observable<T> for State<T> {
    fn add_subscriber(&self, callback: Arc<dyn Fn(&T) -> Result<()> + 'static>) -> usize {
        let id = &callback as *const _ as usize;
        self.subscribers.lock().unwrap().push(callback);
        id
    }

    fn remove_subscriber(&mut self, id: usize) {
        self.subscribers.lock().unwrap().retain(|s| {
            let ptr = s.as_ref() as *const dyn Fn(&T) -> Result<()> as *const () as usize;
            ptr != id
        });
    }

    fn notify(&self) -> Result<()> {
        let value = self.value.lock().unwrap();
        for subscriber in &*self.subscribers.lock().unwrap() {
            subscriber(&value)?;
        }

        Ok(())
    }
}

impl<T: Debug> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").field("value", &self.value).finish()
    }
}
