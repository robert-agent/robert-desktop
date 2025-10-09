pub mod browser;
pub mod cdp;
pub mod error;

//  Re-export commonly used items
pub use browser::chrome::{ChromeDriver, ConnectionMode};
pub use cdp::{
    CdpCommand, CdpExecutor, CdpScript, CdpScriptGenerator, CdpValidator, CommandResult,
    CommandStatus, ErrorLocation, ExecutionReport, ValidationError, ValidationErrorType,
    ValidationResult,
};
pub use error::BrowserError;
