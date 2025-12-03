//! Profile management module.
//!
//! Handles saving and loading display configuration profiles.
//! Platform-specific profile formats are handled transparently.

mod types;
mod storage;

#[cfg(windows)]
mod convert;

#[cfg(windows)]
pub use convert::*;

pub use storage::{
    list_profiles, profile_exists, delete_profile,
    get_profile_details, current_monitors, MonitorDetails,
};

// Windows uses the original DisplayProfile format
#[cfg(windows)]
pub use storage::{save_profile, load_profile};

// Linux uses its own profile format
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::{save_linux_profile, load_linux_profile};
