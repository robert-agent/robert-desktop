mod agent;

mod developer_mode;
mod logging;
mod profiles;

pub use agent::*;
// Note: browser module is pub mod so we can selectively export commands to avoid conflicts
pub use developer_mode::*;
pub use logging::*;
pub use profiles::*;

// Browser automation commands removed as robert-webdriver is deprecated.
// Future implementation will use a different approach if needed.
