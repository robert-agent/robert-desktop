//! Robert Server - Remote execution server for Robert desktop application
//!
//! This library provides a Rust-based Warp web server that receives SSL-encrypted
//! streamed content from the Robert desktop application and forwards requests to
//! the claude-cli tool running headlessly.
//!
//! # Architecture
//!
//! The server implements a REST API with the following key components:
//!
//! - **HTTP/HTTPS Server**: Built on Warp with async Tokio runtime
//! - **Authentication**: Bearer token validation with rate limiting
//! - **Session Management**: Tracks concurrent claude-cli executions
//! - **Streaming**: Server-Sent Events (SSE) for real-time response streaming
//! - **Process Management**: Spawns and manages headless claude-cli processes
//!
//! # Example
//!
//! ```rust,no_run
//! use robert_server::Config;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = Config::dev_default();
//!     // Start server with config...
//! }
//! ```

pub mod api;
pub mod auth;
pub mod claude;
pub mod config;
pub mod error;
pub mod models;
pub mod session;

// Re-export commonly used types
pub use config::Config;
pub use error::{ErrorResponse, RobertError};
pub use models::{
    ClaudeEvent, HealthResponse, RequestContext, RobertRequest, Screenshot, SessionState,
    SessionStatus,
};
