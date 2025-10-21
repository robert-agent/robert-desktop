//! DEPRECATED: Old JSON-based command system
//!
//! This module is deprecated and replaced by `command_md.rs` which implements
//! the correct markdown-template-based approach as per the specification.
//!
//! DO NOT USE - Use `command_md` instead.
//!
//! This file is kept for reference only and will be removed in the future.
//!
//! # Why Deprecated
//!
//! The original implementation used JSON files with simple `{{param}}` substitution.
//! The correct approach is to use markdown template files that describe tasks for
//! an AI agent, with optional CDP JSON that can be generated dynamically.

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::profiles::{
    crypto::EncryptionKey,
    storage::{get_commands_dir, StorageError},
    types::{CommandConfig, SimpleParameter},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Error, Debug)]
pub enum CommandError {
    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    /// Crypto error
    #[error("Crypto error: {0}")]
    CryptoError(#[from] crate::profiles::crypto::CryptoError),

    /// JSON error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Command not found
    #[error("Command not found: {0}")]
    CommandNotFound(String),

    /// Invalid command name
    #[error("Invalid command name: {0}")]
    InvalidCommandName(String),

    /// Missing required parameter
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// Invalid parameter value
    #[error("Invalid parameter value for {0}: {1}")]
    InvalidParameterValue(String, String),

    /// Script execution error
    #[error("Script execution error: {0}")]
    ExecutionError(String),
}

pub type Result<T> = std::result::Result<T, CommandError>;

// ============================================================================
// Command Manager
// ============================================================================

/// Command manager for storing and loading commands
pub struct CommandManager {
    username: String,
    encryption_key: EncryptionKey,
    base_dir: Option<PathBuf>,
}

impl CommandManager {
    /// Create a new command manager for a user
    pub fn new(username: String, encryption_key: EncryptionKey) -> Self {
        Self {
            username,
            encryption_key,
            base_dir: None,
        }
    }

    /// Create a new command manager with custom base directory (for testing)
    #[allow(dead_code)]
    pub fn with_base_dir(
        username: String,
        encryption_key: EncryptionKey,
        base_dir: PathBuf,
    ) -> Self {
        Self {
            username,
            encryption_key,
            base_dir: Some(base_dir),
        }
    }

    /// Get the commands directory for this user
    fn get_commands_dir(&self) -> Result<PathBuf> {
        Ok(get_commands_dir(&self.username, self.base_dir.as_deref())?)
    }

    /// Get the file path for a command
    fn get_command_path(&self, name: &str) -> Result<PathBuf> {
        validate_command_name(name)?;
        Ok(self.get_commands_dir()?.join(format!("{}.json", name)))
    }

    /// Save a command to encrypted JSON file
    pub fn save_command(&self, config: &CommandConfig) -> Result<()> {
        validate_command_name(&config.name)?;

        let command_path = self.get_command_path(&config.name)?;

        // Ensure commands directory exists
        let commands_dir = self.get_commands_dir()?;
        if !commands_dir.exists() {
            fs::create_dir_all(&commands_dir)?;
        }

        // Serialize to JSON
        let json = serde_json::to_string_pretty(config)?;

        // Encrypt
        let encrypted =
            crate::profiles::crypto::encrypt_file(json.as_bytes(), &self.encryption_key)?;

        // Write to file
        fs::write(command_path, encrypted)?;

        log::info!(
            "✅ Saved command '{}' for user '{}'",
            config.name,
            self.username
        );

        Ok(())
    }

    /// Load a command from encrypted JSON file
    pub fn load_command(&self, name: &str) -> Result<CommandConfig> {
        validate_command_name(name)?;

        let command_path = self.get_command_path(name)?;

        if !command_path.exists() {
            return Err(CommandError::CommandNotFound(name.to_string()));
        }

        // Read encrypted file
        let encrypted = fs::read(command_path)?;

        // Decrypt
        let decrypted = crate::profiles::crypto::decrypt_file(&encrypted, &self.encryption_key)?;

        // Parse JSON
        let config: CommandConfig = serde_json::from_slice(&decrypted)?;

        log::debug!("✅ Loaded command '{}' for user '{}'", name, self.username);

        Ok(config)
    }

    /// List all command names
    pub fn list_commands(&self) -> Result<Vec<CommandInfo>> {
        let commands_dir = self.get_commands_dir()?;

        if !commands_dir.exists() {
            return Ok(Vec::new());
        }

        let mut commands = Vec::new();

        for entry in fs::read_dir(commands_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    // Try to load the command to get full info
                    match self.load_command(name) {
                        Ok(config) => {
                            commands.push(CommandInfo {
                                name: config.name,
                                description: config.description,
                                parameter_count: config.parameters.len(),
                                created_at: config.created_at,
                                updated_at: config.updated_at,
                            });
                        }
                        Err(e) => {
                            log::warn!("Failed to load command '{}': {}", name, e);
                        }
                    }
                }
            }
        }

        commands.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(commands)
    }

    /// Delete a command
    pub fn delete_command(&self, name: &str) -> Result<()> {
        validate_command_name(name)?;

        let command_path = self.get_command_path(name)?;

        if !command_path.exists() {
            return Err(CommandError::CommandNotFound(name.to_string()));
        }

        fs::remove_file(command_path)?;

        log::info!("✅ Deleted command '{}' for user '{}'", name, self.username);

        Ok(())
    }

    /// Check if a command exists
    pub fn command_exists(&self, name: &str) -> Result<bool> {
        let command_path = self.get_command_path(name)?;
        Ok(command_path.exists())
    }
}

// ============================================================================
// Command Executor
// ============================================================================

/// Command executor with parameter substitution
pub struct CommandExecutor {
    manager: CommandManager,
}

impl CommandExecutor {
    /// Create a new command executor
    pub fn new(username: String, encryption_key: EncryptionKey) -> Self {
        Self {
            manager: CommandManager::new(username, encryption_key),
        }
    }

    /// Execute a command with parameters
    ///
    /// # Parameters
    /// - `name`: Command name to execute
    /// - `params`: Map of parameter names to values
    ///
    /// # Returns
    /// - Substituted CDP script ready for execution
    pub fn execute_command(&self, name: &str, params: HashMap<String, String>) -> Result<String> {
        // Load command config
        let config = self.manager.load_command(name)?;

        // Validate required parameters
        for param in &config.parameters {
            if param.required && !params.contains_key(&param.name) {
                return Err(CommandError::MissingParameter(param.name.clone()));
            }
        }

        // Substitute parameters in script
        let mut script = config.script.clone();

        for param in &config.parameters {
            let value = if let Some(v) = params.get(&param.name) {
                v.clone()
            } else if let Some(default) = &param.default_value {
                default.clone()
            } else {
                continue; // Skip optional parameters without values
            };

            // Validate parameter type
            validate_parameter_value(&param.name, &value, &param.param_type)?;

            // Replace {{param_name}} with value
            let placeholder = format!("{{{{{}}}}}", param.name);
            script = script.replace(&placeholder, &value);
        }

        Ok(script)
    }

    /// Get the command manager for direct access
    pub fn manager(&self) -> &CommandManager {
        &self.manager
    }
}

// ============================================================================
// Helper Types
// ============================================================================

/// Command summary information for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    /// Command name
    pub name: String,

    /// Description
    pub description: String,

    /// Number of parameters
    pub parameter_count: usize,

    /// Creation timestamp
    pub created_at: chrono::DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: chrono::DateTime<Utc>,
}

// ============================================================================
// Validation
// ============================================================================

/// Validate command name format (kebab-case)
fn validate_command_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(CommandError::InvalidCommandName(
            "Command name cannot be empty".to_string(),
        ));
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(CommandError::InvalidCommandName(format!(
            "Command name '{}' contains invalid characters. Use kebab-case (alphanumeric and dash only)",
            name
        )));
    }

    Ok(())
}

/// Validate parameter value against its type
fn validate_parameter_value(
    name: &str,
    value: &str,
    param_type: &crate::profiles::types::SimpleParameterType,
) -> Result<()> {
    use crate::profiles::types::SimpleParameterType;

    match param_type {
        SimpleParameterType::Text => {
            // Any string is valid
            Ok(())
        }
        SimpleParameterType::Number => {
            // Must be parseable as a number
            if value.parse::<f64>().is_err() {
                return Err(CommandError::InvalidParameterValue(
                    name.to_string(),
                    format!("'{}' is not a valid number", value),
                ));
            }
            Ok(())
        }
        SimpleParameterType::Boolean => {
            // Must be "true" or "false"
            if value != "true" && value != "false" {
                return Err(CommandError::InvalidParameterValue(
                    name.to_string(),
                    format!("'{}' is not a valid boolean (use 'true' or 'false')", value),
                ));
            }
            Ok(())
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{crypto::derive_key, types::SimpleParameterType};

    #[test]
    fn test_validate_command_name() {
        assert!(validate_command_name("test-command").is_ok());
        assert!(validate_command_name("command123").is_ok());
        assert!(validate_command_name("my-awesome-command").is_ok());
        assert!(validate_command_name("").is_err());
        assert!(validate_command_name("command_name").is_err());
        assert!(validate_command_name("command name").is_err());
    }

    #[test]
    fn test_validate_parameter_value() {
        assert!(validate_parameter_value("text", "hello", &SimpleParameterType::Text).is_ok());
        assert!(validate_parameter_value("num", "42", &SimpleParameterType::Number).is_ok());
        assert!(validate_parameter_value("num", "3.14", &SimpleParameterType::Number).is_ok());
        assert!(validate_parameter_value("bool", "true", &SimpleParameterType::Boolean).is_ok());
        assert!(validate_parameter_value("bool", "false", &SimpleParameterType::Boolean).is_ok());

        assert!(
            validate_parameter_value("num", "not-a-number", &SimpleParameterType::Number).is_err()
        );
        assert!(validate_parameter_value("bool", "yes", &SimpleParameterType::Boolean).is_err());
    }

    #[test]
    fn test_command_save_load() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let manager = CommandManager::with_base_dir(
            "testuser".to_string(),
            key,
            temp_dir.path().to_path_buf(),
        );

        // Create commands directory
        let commands_dir = manager.get_commands_dir().unwrap();
        fs::create_dir_all(&commands_dir).unwrap();

        let config = CommandConfig {
            name: "test-command".to_string(),
            description: "A test command".to_string(),
            script: "Page.navigate {\"url\": \"{{url}}\"}".to_string(),
            parameters: vec![SimpleParameter {
                name: "url".to_string(),
                param_type: SimpleParameterType::Text,
                label: "URL".to_string(),
                required: true,
                default_value: None,
            }],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Save command
        manager.save_command(&config).unwrap();

        // Load command
        let loaded = manager.load_command("test-command").unwrap();

        assert_eq!(loaded.name, config.name);
        assert_eq!(loaded.description, config.description);
        assert_eq!(loaded.script, config.script);
        assert_eq!(loaded.parameters.len(), 1);
    }

    #[test]
    fn test_parameter_substitution() {
        use tempfile::TempDir;

        let _temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let executor = CommandExecutor::new("testuser".to_string(), key);

        // Create commands directory
        let commands_dir = executor.manager.get_commands_dir().unwrap();
        fs::create_dir_all(&commands_dir).unwrap();

        let config = CommandConfig {
            name: "navigate-command".to_string(),
            description: "Navigate to URL".to_string(),
            script: r#"{"name": "Navigate", "cdp_commands": [{"method": "Page.navigate", "params": {"url": "{{url}}"}}]}"#.to_string(),
            parameters: vec![SimpleParameter {
                name: "url".to_string(),
                param_type: SimpleParameterType::Text,
                label: "URL".to_string(),
                required: true,
                default_value: None,
            }],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Save command
        executor.manager.save_command(&config).unwrap();

        // Execute with parameters
        let mut params = HashMap::new();
        params.insert("url".to_string(), "https://example.com".to_string());

        let result = executor
            .execute_command("navigate-command", params)
            .unwrap();

        assert!(result.contains("https://example.com"));
        assert!(!result.contains("{{url}}"));
    }

    #[test]
    fn test_missing_required_parameter() {
        use tempfile::TempDir;

        let _temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let executor = CommandExecutor::new("testuser".to_string(), key);

        // Create commands directory
        let commands_dir = executor.manager.get_commands_dir().unwrap();
        fs::create_dir_all(&commands_dir).unwrap();

        let config = CommandConfig {
            name: "test-required".to_string(),
            description: "Test required params".to_string(),
            script: "{{required_param}}".to_string(),
            parameters: vec![SimpleParameter {
                name: "required_param".to_string(),
                param_type: SimpleParameterType::Text,
                label: "Required".to_string(),
                required: true,
                default_value: None,
            }],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        executor.manager.save_command(&config).unwrap();

        // Execute without required parameter
        let params = HashMap::new();
        let result = executor.execute_command("test-required", params);

        assert!(result.is_err());
        assert!(matches!(result, Err(CommandError::MissingParameter(_))));
    }
}
