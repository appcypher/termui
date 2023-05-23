//! This module contains the error types used in the library.

use thiserror::Error;

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// Control error type contains a single variant `Exit` which is used to exit
/// the termui application event loop gracefully as it allows for clean up unlike process::exit.
#[derive(Debug, Error)]
pub enum Control {
    /// Exit with the given code.
    #[error("exit with code {0}")]
    Exit(usize),
}

//----------------------------------------------------------------
// Macros
//----------------------------------------------------------------

/// This macro allows you to exit the termui application gracefully.
#[macro_export]
macro_rules! exit {
    () => {
        return anyhow::Result::<_>::Err($crate::error::Control::Exit(0).into());
    };
    ($exit_code:expr) => {
        return anyhow::Result::<_>::Err($crate::error::Control::Exit($exit_code).into());
    };
}
