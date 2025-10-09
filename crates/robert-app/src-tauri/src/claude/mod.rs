pub mod health;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub use health::{ClaudeHealthCheck, HealthStatus};

/// Configuration for Claude CLI invocation
#[derive(Debug, Clone, Default)]
pub struct ClaudeConfig {
    /// Path to the claude CLI executable (defaults to "claude" in PATH)
    pub claude_path: Option<PathBuf>,
    /// Whether to skip permissions (use only in trusted sandboxes)
    #[allow(dead_code)]
    pub skip_permissions: bool,
    /// Model to use (e.g., "sonnet", "opus", or full model name)
    pub model: Option<String>,
    /// Additional directories to allow tool access to
    #[allow(dead_code)]
    pub allowed_dirs: Vec<PathBuf>,
    /// Tools to allow (e.g., ["Bash(git:*)", "Edit"])
    #[allow(dead_code)]
    pub allowed_tools: Vec<String>,
    /// Tools to disallow
    #[allow(dead_code)]
    pub disallowed_tools: Vec<String>,
}

/// Input message for Claude CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeInput {
    /// Text prompt
    pub prompt: String,
    /// Optional image paths (screenshots)
    pub images: Vec<PathBuf>,
    /// Optional HTML content
    pub html: Option<String>,
}

/// Response from Claude CLI (JSON format)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaudeResponse {
    /// The response text from Claude
    pub text: String,
    /// Optional structured data
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Streaming response chunk from Claude CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ClaudeStreamChunk {
    /// Type of chunk (e.g., "content", "metadata")
    #[serde(rename = "type")]
    pub chunk_type: String,
    /// Content of the chunk
    pub content: Option<String>,
    /// Metadata for the chunk
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Claude CLI client for headless automation
pub struct ClaudeClient {
    config: ClaudeConfig,
}

impl ClaudeClient {
    /// Create a new Claude client with default configuration
    pub fn new() -> Self {
        Self {
            config: ClaudeConfig::default(),
        }
    }

    /// Create a new Claude client with custom configuration
    pub fn with_config(config: ClaudeConfig) -> Self {
        Self { config }
    }

    /// Execute Claude CLI with the given input and return the response
    pub async fn execute(&self, input: ClaudeInput) -> Result<ClaudeResponse> {
        let output = self.run_command(&input, false).await?;

        // Parse JSON response
        let response: ClaudeResponse =
            serde_json::from_str(&output).context("Failed to parse Claude CLI response as JSON")?;

        Ok(response)
    }

    /// Execute Claude CLI with streaming output
    #[allow(dead_code)]
    pub async fn execute_streaming<F>(&self, input: ClaudeInput, mut callback: F) -> Result<()>
    where
        F: FnMut(ClaudeStreamChunk) -> Result<()>,
    {
        let mut child = self.spawn_streaming_command(&input).await?;

        // Get stdout
        let stdout = child.stdout.take().context("Failed to capture stdout")?;

        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        // Process each line as it arrives
        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }

            // Parse each line as a JSON chunk
            match serde_json::from_str::<ClaudeStreamChunk>(&line) {
                Ok(chunk) => {
                    callback(chunk)?;
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to parse stream chunk: {} - Line: {}",
                        e, line
                    );
                }
            }
        }

        // Wait for the process to complete
        let status = child.wait().await?;
        if !status.success() {
            anyhow::bail!("Claude CLI exited with status: {}", status);
        }

        Ok(())
    }

    /// Build the prompt with images and HTML
    fn build_prompt(&self, input: &ClaudeInput) -> String {
        let mut prompt = String::new();

        // Add image references if provided
        if !input.images.is_empty() {
            prompt.push_str("I'm providing screenshots of a web page:\n\n");
            for (idx, image_path) in input.images.iter().enumerate() {
                prompt.push_str(&format!(
                    "Screenshot {}: {}\n",
                    idx + 1,
                    image_path.display()
                ));
            }
            prompt.push('\n');
        }

        // Add HTML content if provided
        if let Some(html) = &input.html {
            prompt.push_str("Here is the HTML content of the page:\n\n");
            prompt.push_str("```html\n");
            prompt.push_str(html);
            prompt.push_str("\n```\n");
            prompt.push('\n');
        }

        // Add the main prompt
        prompt.push_str(&input.prompt);

        prompt
    }

    /// Run the Claude CLI command and return the output
    async fn run_command(&self, input: &ClaudeInput, streaming: bool) -> Result<String> {
        let claude_cmd = self
            .config
            .claude_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "claude".to_string());

        let mut cmd = Command::new(&claude_cmd);

        // Add print mode for non-interactive use
        cmd.arg("--print");

        // Set output format
        if streaming {
            cmd.arg("--output-format").arg("stream-json");
        } else {
            cmd.arg("--output-format").arg("json");
        }

        // Add model if specified
        if let Some(model) = &self.config.model {
            cmd.arg("--model").arg(model);
        }

        // Add permission settings
        if self.config.skip_permissions {
            cmd.arg("--dangerously-skip-permissions");
        }

        // Add allowed directories
        if !self.config.allowed_dirs.is_empty() {
            cmd.arg("--add-dir");
            for dir in &self.config.allowed_dirs {
                cmd.arg(dir);
            }
        }

        // Add allowed tools
        if !self.config.allowed_tools.is_empty() {
            cmd.arg("--allowed-tools");
            for tool in &self.config.allowed_tools {
                cmd.arg(tool);
            }
        }

        // Add disallowed tools
        if !self.config.disallowed_tools.is_empty() {
            cmd.arg("--disallowed-tools");
            for tool in &self.config.disallowed_tools {
                cmd.arg(tool);
            }
        }

        // Build the full prompt
        let prompt = self.build_prompt(input);

        // Add prompt as the final argument
        cmd.arg(&prompt);

        // Execute the command
        let output = cmd.output().await.context("Failed to execute Claude CLI")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Claude CLI failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Spawn a streaming command
    #[allow(dead_code)]
    async fn spawn_streaming_command(&self, input: &ClaudeInput) -> Result<tokio::process::Child> {
        let claude_cmd = self
            .config
            .claude_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "claude".to_string());

        let mut cmd = Command::new(&claude_cmd);

        // Configure for streaming
        cmd.arg("--print");
        cmd.arg("--output-format").arg("stream-json");

        // Add model if specified
        if let Some(model) = &self.config.model {
            cmd.arg("--model").arg(model);
        }

        // Add permission settings
        if self.config.skip_permissions {
            cmd.arg("--dangerously-skip-permissions");
        }

        // Add allowed directories
        if !self.config.allowed_dirs.is_empty() {
            cmd.arg("--add-dir");
            for dir in &self.config.allowed_dirs {
                cmd.arg(dir);
            }
        }

        // Add allowed tools
        if !self.config.allowed_tools.is_empty() {
            cmd.arg("--allowed-tools");
            for tool in &self.config.allowed_tools {
                cmd.arg(tool);
            }
        }

        // Add disallowed tools
        if !self.config.disallowed_tools.is_empty() {
            cmd.arg("--disallowed-tools");
            for tool in &self.config.disallowed_tools {
                cmd.arg(tool);
            }
        }

        // Build the full prompt
        let prompt = self.build_prompt(input);
        cmd.arg(&prompt);

        // Spawn with stdout capture
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn().context("Failed to spawn Claude CLI")?;

        Ok(child)
    }
}

impl Default for ClaudeClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_prompt() {
        let client = ClaudeClient::new();
        let input = ClaudeInput {
            prompt: "What do you see in the screenshot?".to_string(),
            images: vec![PathBuf::from("/tmp/screenshot.png")],
            html: Some("<html><body>Hello</body></html>".to_string()),
        };

        let prompt = client.build_prompt(&input);
        assert!(prompt.contains("screenshot"));
        assert!(prompt.contains("HTML"));
        assert!(prompt.contains("What do you see"));
    }
}
