//! Markdown-based command system
//!
//! This module handles commands stored as markdown template files that describe
//! tasks for an AI agent to execute. This is the correct implementation as per
//! the specification.
//!
//! Features:
//! - Save/load/list/delete commands as encrypted markdown files
//! - Parse markdown templates with YAML frontmatter
//! - AI-driven CDP generation from markdown descriptions
//! - Optional static CDP scripts
//!
//! # Markdown Format
//!
//! Commands are stored as `.md` files with:
//! - YAML frontmatter: metadata (command_name, description, version, etc.)
//! - Markdown sections: Parameters, Rules, Checklist
//! - Optional CDP Script Template section
//!
//! # AI Integration
//!
//! When executing a command:
//! 1. Load markdown template
//! 2. Build AI prompt with:
//!    - Markdown template content
//!    - User-provided parameter values
//!    - User profile context (if exists)
//!    - Request for CDP command generation
//! 3. Parse CDP JSON from AI response
//! 4. Fallback to static CDP if markdown includes it

use crate::profiles::{
    crypto::EncryptionKey,
    markdown::{generate_command_template, parse_command_template, MarkdownParseError},
    storage::{get_commands_dir, StorageError},
    types::{Command, CommandInfo},
};
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

    /// Markdown parsing error
    #[error("Markdown parsing error: {0}")]
    MarkdownError(#[from] MarkdownParseError),

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

    /// AI generation error
    #[error("AI generation error: {0}")]
    AIGenerationError(String),

    /// CDP execution error
    #[error("CDP execution error: {0}")]
    CDPExecutionError(String),
}

pub type Result<T> = std::result::Result<T, CommandError>;

// ============================================================================
// Command Manager (Markdown-based)
// ============================================================================

/// Command manager for markdown-based commands
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
        Ok(self.get_commands_dir()?.join(format!("{}.md", name)))
    }

    /// Save a command as encrypted markdown file
    ///
    /// # Parameters
    /// - `command`: The Command structure to save
    ///
    /// # Returns
    /// - `Ok(())` if successful
    ///
    /// # Errors
    /// - `InvalidCommandName` if command name is invalid
    /// - `IoError` if file write fails
    /// - `CryptoError` if encryption fails
    pub fn save_command(&self, command: &Command) -> Result<()> {
        validate_command_name(&command.frontmatter.command_name)?;

        let command_path = self.get_command_path(&command.frontmatter.command_name)?;

        // Ensure commands directory exists
        let commands_dir = self.get_commands_dir()?;
        if !commands_dir.exists() {
            fs::create_dir_all(&commands_dir)?;
        }

        // Generate markdown template
        let markdown = generate_command_template(command)?;

        // Encrypt
        let encrypted =
            crate::profiles::crypto::encrypt_file(markdown.as_bytes(), &self.encryption_key)?;

        // Write to file
        fs::write(command_path, encrypted)?;

        log::info!(
            "Saved command '{}' for user '{}'",
            command.frontmatter.command_name,
            self.username
        );

        Ok(())
    }

    /// Load a command from encrypted markdown file
    ///
    /// # Parameters
    /// - `name`: The command name to load
    ///
    /// # Returns
    /// - `Command` structure parsed from markdown
    ///
    /// # Errors
    /// - `CommandNotFound` if command doesn't exist
    /// - `IoError` if file read fails
    /// - `CryptoError` if decryption fails
    /// - `MarkdownError` if markdown parsing fails
    pub fn load_command(&self, name: &str) -> Result<Command> {
        validate_command_name(name)?;

        let command_path = self.get_command_path(name)?;

        if !command_path.exists() {
            return Err(CommandError::CommandNotFound(name.to_string()));
        }

        // Read encrypted file
        let encrypted = fs::read(command_path)?;

        // Decrypt
        let decrypted = crate::profiles::crypto::decrypt_file(&encrypted, &self.encryption_key)?;

        // Parse markdown
        let markdown = String::from_utf8_lossy(&decrypted);
        let command = parse_command_template(&markdown)?;

        log::debug!("Loaded command '{}' for user '{}'", name, self.username);

        Ok(command)
    }

    /// List all command names with metadata
    ///
    /// # Returns
    /// - Vector of `CommandInfo` with basic command metadata
    ///
    /// # Errors
    /// - `IoError` if directory read fails
    pub fn list_commands(&self) -> Result<Vec<CommandInfo>> {
        let commands_dir = self.get_commands_dir()?;

        if !commands_dir.exists() {
            return Ok(Vec::new());
        }

        let mut commands = Vec::new();

        for entry in fs::read_dir(commands_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    // Try to load the command to get full info
                    match self.load_command(name) {
                        Ok(command) => {
                            commands.push(CommandInfo {
                                command_name: command.frontmatter.command_name,
                                description: command.frontmatter.description,
                                browser_profile: command.frontmatter.browser_profile,
                                created_at: command.frontmatter.created_at,
                                updated_at: command.frontmatter.updated_at,
                                version: command.frontmatter.version,
                            });
                        }
                        Err(e) => {
                            log::warn!("Failed to load command '{}': {}", name, e);
                        }
                    }
                }
            }
        }

        commands.sort_by(|a, b| a.command_name.cmp(&b.command_name));

        Ok(commands)
    }

    /// Delete a command
    ///
    /// # Parameters
    /// - `name`: The command name to delete
    ///
    /// # Errors
    /// - `CommandNotFound` if command doesn't exist
    /// - `IoError` if file deletion fails
    pub fn delete_command(&self, name: &str) -> Result<()> {
        validate_command_name(name)?;

        let command_path = self.get_command_path(name)?;

        if !command_path.exists() {
            return Err(CommandError::CommandNotFound(name.to_string()));
        }

        fs::remove_file(command_path)?;

        log::info!("Deleted command '{}' for user '{}'", name, self.username);

        Ok(())
    }

    /// Check if a command exists
    ///
    /// # Parameters
    /// - `name`: The command name to check
    ///
    /// # Returns
    /// - `true` if command exists, `false` otherwise
    pub fn command_exists(&self, name: &str) -> Result<bool> {
        let command_path = self.get_command_path(name)?;
        Ok(command_path.exists())
    }
}

// ============================================================================
// AI Prompt Builder
// ============================================================================

/// Build AI prompt for command execution
///
/// This generates a prompt that includes:
/// - The markdown command template
/// - User-provided parameter values
/// - User profile context (if available)
/// - Instructions for CDP generation
///
/// # Parameters
/// - `command`: The Command to execute
/// - `params`: User-provided parameter values
/// - `user_profile`: Optional user profile markdown content
///
/// # Returns
/// - AI prompt string ready to send to LLM
pub fn build_ai_prompt(
    command: &Command,
    params: &HashMap<String, String>,
    user_profile: Option<&str>,
) -> String {
    let mut prompt = String::new();

    // Add system context
    prompt.push_str("# Task: Execute Browser Automation Command\n\n");
    prompt.push_str("You are a browser automation agent. Your task is to generate Chrome DevTools Protocol (CDP) commands based on the command template and user parameters below.\n\n");

    // Add user profile if provided
    if let Some(profile) = user_profile {
        prompt.push_str("## User Profile\n\n");
        prompt.push_str(profile);
        prompt.push_str("\n\n");
    }

    // Add command template
    prompt.push_str("## Command Template\n\n");
    prompt.push_str(&format!(
        "**Command**: {}\n",
        command.frontmatter.command_name
    ));
    prompt.push_str(&format!(
        "**Description**: {}\n\n",
        command.frontmatter.description
    ));

    // Add parameters with values
    prompt.push_str("### Parameters (User-Provided)\n\n");
    for param in &command.parameters {
        if let Some(value) = params.get(&param.name) {
            prompt.push_str(&format!(
                "- **{}**: {} (value: `{}`)\n",
                param.name, param.label, value
            ));
        } else if param.required {
            prompt.push_str(&format!(
                "- **{}**: {} (MISSING REQUIRED VALUE)\n",
                param.name, param.label
            ));
        }
    }
    prompt.push('\n');

    // Add rules
    prompt.push_str("### Rules and Constraints\n\n");
    for rule in &command.rules {
        prompt.push_str(&format!("- {}\n", rule));
    }
    prompt.push('\n');

    // Add checklist
    prompt.push_str("### Success Criteria\n\n");
    for item in &command.checklist {
        prompt.push_str(&format!("- {}\n", item));
    }
    prompt.push('\n');

    // Add CDP generation request
    prompt.push_str("## Output Required\n\n");
    prompt.push_str("Generate a JSON array of Chrome DevTools Protocol (CDP) commands to accomplish this task. Each command should have:\n");
    prompt.push_str(
        "- `method`: The CDP method name (e.g., \"Page.navigate\", \"Runtime.evaluate\")\n",
    );
    prompt.push_str("- `params`: Object with method parameters\n\n");
    prompt.push_str("Example format:\n");
    prompt.push_str("```json\n");
    prompt.push_str("[\n");
    prompt.push_str(
        "  {\"method\": \"Page.navigate\", \"params\": {\"url\": \"https://example.com\"}},\n",
    );
    prompt.push_str("  {\"method\": \"Page.loadEventFired\", \"params\": {}}\n");
    prompt.push_str("]\n");
    prompt.push_str("```\n\n");

    // If static CDP exists, mention it
    if command.cdp_script_template.is_some() {
        prompt.push_str("Note: A static CDP script template is available, but you may generate a better one based on the current parameters.\n\n");
    }

    prompt.push_str("Generate the CDP commands now:\n");

    prompt
}

// ============================================================================
// Command Executor (AI-driven)
// ============================================================================

/// Command executor with AI integration
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

    /// Build execution prompt for a command
    ///
    /// This creates the AI prompt but doesn't execute it yet.
    /// The caller can then send this to an AI service.
    ///
    /// # Parameters
    /// - `name`: Command name to execute
    /// - `params`: Parameter values
    /// - `user_profile`: Optional user profile content
    ///
    /// # Returns
    /// - AI prompt string
    pub fn build_execution_prompt(
        &self,
        name: &str,
        params: HashMap<String, String>,
        user_profile: Option<String>,
    ) -> Result<String> {
        // Load command
        let command = self.manager.load_command(name)?;

        // Validate required parameters
        for param in &command.parameters {
            if param.required && !params.contains_key(&param.name) {
                return Err(CommandError::MissingParameter(param.name.clone()));
            }
        }

        // Build AI prompt
        let prompt = build_ai_prompt(&command, &params, user_profile.as_deref());

        Ok(prompt)
    }

    /// Get static CDP script with parameter substitution
    ///
    /// Fallback method when AI generation is not available or fails.
    /// Uses the optional CDP Script Template section from markdown.
    ///
    /// # Parameters
    /// - `name`: Command name to execute
    /// - `params`: Parameter values
    ///
    /// # Returns
    /// - CDP script with parameters substituted
    ///
    /// # Errors
    /// - `CommandNotFound` if command doesn't exist
    /// - `AIGenerationError` if CDP template not available
    pub fn get_static_cdp_script(
        &self,
        name: &str,
        params: HashMap<String, String>,
    ) -> Result<String> {
        // Load command
        let command = self.manager.load_command(name)?;

        // Check if static CDP exists
        let cdp_template = command.cdp_script_template.ok_or_else(|| {
            CommandError::AIGenerationError(
                "No static CDP template available. AI generation is required.".to_string(),
            )
        })?;

        // Validate required parameters
        for param in &command.parameters {
            if param.required && !params.contains_key(&param.name) {
                return Err(CommandError::MissingParameter(param.name.clone()));
            }
        }

        // Substitute parameters
        let mut script = cdp_template;
        for (name, value) in params {
            let placeholder = format!("{{{{{}}}}}", name);
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

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{
        crypto::derive_key,
        types::{CommandFrontmatter, CommandParameter, ParameterType},
    };
    use chrono::Utc;
    use tempfile::TempDir;

    fn create_test_command() -> Command {
        Command {
            frontmatter: CommandFrontmatter {
                command_name: "test-command".to_string(),
                description: "A test command".to_string(),
                browser_profile: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                version: "1.0.0".to_string(),
                changelog: vec![],
            },
            parameters: vec![CommandParameter {
                name: "url".to_string(),
                param_type: ParameterType::TextInput,
                label: "URL to navigate".to_string(),
                placeholder: None,
                required: true,
                default: None,
            }],
            rules: vec!["URL must be valid".to_string()],
            checklist: vec!["Navigate to URL".to_string()],
            generative_ui: None,
            cdp_script_template: Some(
                r#"[{"method": "Page.navigate", "params": {"url": "{{url}}"}}]"#.to_string(),
            ),
        }
    }

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
    fn test_command_save_load_markdown() {
        let temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let manager = CommandManager::with_base_dir(
            "testuser".to_string(),
            key,
            temp_dir.path().to_path_buf(),
        );

        let command = create_test_command();

        // Save command
        manager.save_command(&command).unwrap();

        // Load command
        let loaded = manager.load_command("test-command").unwrap();

        assert_eq!(
            loaded.frontmatter.command_name,
            command.frontmatter.command_name
        );
        assert_eq!(
            loaded.frontmatter.description,
            command.frontmatter.description
        );
        assert_eq!(loaded.parameters.len(), 1);
        assert_eq!(loaded.rules.len(), 1);
        assert_eq!(loaded.checklist.len(), 1);
    }

    #[test]
    fn test_list_commands() {
        let temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let manager = CommandManager::with_base_dir(
            "testuser".to_string(),
            key,
            temp_dir.path().to_path_buf(),
        );

        // Save multiple commands
        let cmd1 = create_test_command();
        manager.save_command(&cmd1).unwrap();

        let mut cmd2 = create_test_command();
        cmd2.frontmatter.command_name = "another-command".to_string();
        manager.save_command(&cmd2).unwrap();

        // List commands
        let commands = manager.list_commands().unwrap();
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_delete_command() {
        let temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let manager = CommandManager::with_base_dir(
            "testuser".to_string(),
            key,
            temp_dir.path().to_path_buf(),
        );

        let command = create_test_command();
        manager.save_command(&command).unwrap();

        assert!(manager.command_exists("test-command").unwrap());

        manager.delete_command("test-command").unwrap();

        assert!(!manager.command_exists("test-command").unwrap());
    }

    #[test]
    fn test_build_ai_prompt() {
        let command = create_test_command();
        let mut params = HashMap::new();
        params.insert("url".to_string(), "https://example.com".to_string());

        let prompt = build_ai_prompt(&command, &params, None);

        assert!(prompt.contains("test-command"));
        assert!(prompt.contains("https://example.com"));
        assert!(prompt.contains("URL must be valid"));
        assert!(prompt.contains("CDP commands"));
    }

    #[test]
    fn test_get_static_cdp_script() {
        let _temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let executor = CommandExecutor::new("testuser".to_string(), key);

        // Create commands directory
        let commands_dir = executor.manager.get_commands_dir().unwrap();
        fs::create_dir_all(&commands_dir).unwrap();

        let command = create_test_command();
        executor.manager.save_command(&command).unwrap();

        let mut params = HashMap::new();
        params.insert("url".to_string(), "https://example.com".to_string());

        let script = executor
            .get_static_cdp_script("test-command", params)
            .unwrap();

        assert!(script.contains("https://example.com"));
        assert!(!script.contains("{{url}}"));
    }

    #[test]
    fn test_missing_required_parameter() {
        let _temp_dir = TempDir::new().unwrap();
        let (key, _) = derive_key("test_password", None).unwrap();

        let executor = CommandExecutor::new("testuser".to_string(), key);

        // Create commands directory
        let commands_dir = executor.manager.get_commands_dir().unwrap();
        fs::create_dir_all(&commands_dir).unwrap();

        let command = create_test_command();
        executor.manager.save_command(&command).unwrap();

        // Execute without required parameter
        let params = HashMap::new();
        let result = executor.build_execution_prompt("test-command", params, None);

        assert!(result.is_err());
        assert!(matches!(result, Err(CommandError::MissingParameter(_))));
    }
}
