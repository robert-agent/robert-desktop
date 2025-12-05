//! API endpoints for robert-server
//!
//! This module contains all HTTP endpoint handlers and route definitions.

pub mod execute;
pub mod health;
pub mod sessions;
pub mod inference;

pub use execute::execute_handler;
pub use health::health_handler;
pub use sessions::{delete_session_handler, get_session_handler};
pub use inference::inference_handler;
