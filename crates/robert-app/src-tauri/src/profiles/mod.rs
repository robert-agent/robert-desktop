//! User profiles and multi-user support system
//!
//! This module now re-exports types and functionality from the `robert-types` crate.

pub use robert_types::profiles::auth;
pub use robert_types::profiles::command;
pub use robert_types::profiles::command_md;
pub use robert_types::profiles::crypto;
pub use robert_types::profiles::manager;
pub use robert_types::profiles::markdown;
pub use robert_types::profiles::storage;
pub use robert_types::profiles::types;

pub use robert_types::profiles::types::{
    CommandConfig, SimpleParameter, SimpleParameterType, UserConfig, UserPreferences, UserStats,
};
