//! Profile management module.
//!
//! Handles saving and loading display configuration profiles.

mod types;
mod storage;
mod convert;

pub use storage::{
    list_profiles, profile_exists, save_profile, load_profile, delete_profile,
    get_profile_details, current_monitors, MonitorDetails,
};
pub use convert::*;
