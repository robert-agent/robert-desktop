//! Claude CLI execution module
//!
//! Provides interfaces for executing claude-cli processes and streaming results.

pub mod executor;
pub mod mock;

pub use executor::ClaudeExecutor;
pub use mock::MockClaudeExecutor;

use crate::error::RobertError;
use crate::models::{ClaudeEvent, RobertRequest};
use futures::Stream;

/// Trait for Claude CLI executors
///
/// Defines the interface for executing Claude CLI requests with streaming responses.
/// Implementations include real executor (spawns claude-cli) and mock executor (for testing).
#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    /// Executes a Robert request and returns a stream of Claude events
    ///
    /// The implementation should:
    /// 1. Validate the request
    /// 2. Spawn/simulate claude-cli process
    /// 3. Stream stdout/stderr events
    /// 4. Handle timeout and cleanup
    /// 5. Emit Complete or Error event at end
    ///
    /// # Arguments
    /// * `request` - The validated Robert request to execute
    ///
    /// # Returns
    /// Async stream of ClaudeEvent instances
    ///
    /// # Errors
    /// Stream may include ClaudeEvent::Error for execution failures
    async fn execute(
        &self,
        request: RobertRequest,
    ) -> Box<dyn Stream<Item = Result<ClaudeEvent, RobertError>> + Send + Unpin + 'static>;
}
