//! CCD (Connecting and Configuring Displays) API module.
//!
//! Provides Windows API bindings for display configuration management.

mod types;
mod api;
mod matcher;

pub use types::*;
pub use api::*;
pub use matcher::*;
