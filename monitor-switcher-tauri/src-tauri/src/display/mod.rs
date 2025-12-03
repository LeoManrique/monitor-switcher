//! Display configuration management.
//!
//! This module provides a platform-agnostic interface for managing display configurations.
//! Platform-specific implementations are in separate submodules:
//!
//! - `windows/` - Windows CCD API implementation
//! - `linux/` - Linux XRandR implementation
//!
//! ## Architecture
//!
//! Each platform module exports the same public API, allowing the rest of the application
//! to work identically regardless of the underlying platform.
//!
//! ## Adding a New Platform
//!
//! 1. Create a new submodule (e.g., `macos/`)
//! 2. Implement the required public functions matching the existing API
//! 3. Add conditional compilation below

// ============================================================================
// Platform-Specific Implementations
// ============================================================================

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

// ============================================================================
// Compile-time check for unsupported platforms
// ============================================================================

#[cfg(not(any(windows, target_os = "linux")))]
compile_error!("Unsupported platform. Only Windows and Linux are supported.");
