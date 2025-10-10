//! Agent and workflow commands

use crate::agent::{AgentConfig, WorkflowExecutor, WorkflowResult, WorkflowType};
use crate::events::*;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, State};

/// Request to process a chat message
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageRequest {
    /// User's message
    pub message: String,

    /// Type of workflow to execute
    pub workflow_type: WorkflowType,

    /// Name of the agent to use
    #[serde(default = "default_agent_name")]
    pub agent_name: String,

    /// Whether to include screenshot
    #[serde(default = "default_true")]
    pub include_screenshot: bool,

    /// Whether to include HTML
    #[serde(default = "default_true")]
    pub include_html: bool,
}

fn default_agent_name() -> String {
    "cdp-generator".to_string()
}

fn default_true() -> bool {
    true
}

/// Process a chat message and execute the appropriate workflow
#[tauri::command]
pub async fn process_chat_message(
    app: AppHandle,
    state: State<'_, AppState>,
    request: ChatMessageRequest,
) -> Result<WorkflowResult, String> {
    emit_info(&app, "Processing chat message...").ok();

    // Load agent configuration
    let agent_config = load_or_create_agent_config(&app, &request.agent_name).await?;

    // Get screenshot if requested and browser is available
    let screenshot_path = if request.include_screenshot {
        capture_screenshot_if_available(&app, &state).await
    } else {
        None
    };

    // Get HTML if requested and browser is available
    let html_content = if request.include_html {
        get_html_if_available(&app, &state).await
    } else {
        None
    };

    let executor = WorkflowExecutor::new();

    emit_claude_processing(&app, "Executing workflow...").ok();

    // Execute workflow with driver reference
    let result = {
        let driver_lock = state.driver.lock().await;
        let driver_ref = driver_lock.as_ref();

        executor
            .execute(
                request.workflow_type,
                request.message.clone(),
                &agent_config,
                screenshot_path,
                html_content,
                driver_ref,
            )
            .await
    };

    match result {
        Ok(result) => {
            if result.success {
                emit_success(&app, result.message.clone()).ok();
            } else {
                emit_error(
                    &app,
                    result.message.clone(),
                    result.error.clone(),
                )
                .ok();
            }
            Ok(result)
        }
        Err(e) => {
            let error_msg = format!("Workflow execution failed: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
}

/// Initialize default agent configs
#[tauri::command]
pub async fn init_agent_configs(app: AppHandle) -> Result<Vec<String>, String> {
    emit_info(&app, "Initializing agent configurations...").ok();

    let mut initialized = Vec::new();

    // Initialize CDP generator agent
    let cdp_agent = AgentConfig::default_cdp_agent();
    let cdp_path = AgentConfig::config_path(&cdp_agent.name)
        .map_err(|e| format!("Failed to get config path: {}", e))?;

    if !cdp_path.exists() {
        cdp_agent
            .save(&cdp_path)
            .await
            .map_err(|e| format!("Failed to save CDP agent config: {}", e))?;
        initialized.push(cdp_agent.name.clone());
    }

    // Initialize meta agent
    let meta_agent = AgentConfig::default_meta_agent();
    let meta_path = AgentConfig::config_path(&meta_agent.name)
        .map_err(|e| format!("Failed to get config path: {}", e))?;

    if !meta_path.exists() {
        meta_agent
            .save(&meta_path)
            .await
            .map_err(|e| format!("Failed to save meta agent config: {}", e))?;
        initialized.push(meta_agent.name.clone());
    }

    if !initialized.is_empty() {
        emit_success(
            &app,
            format!("Initialized {} agent config(s)", initialized.len()),
        )
        .ok();
    }

    Ok(initialized)
}

/// List all available agent configs
#[tauri::command]
pub async fn list_agent_configs(app: AppHandle) -> Result<Vec<String>, String> {
    let config_dir = AgentConfig::default_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;

    if !config_dir.exists() {
        return Ok(Vec::new());
    }

    let mut configs = Vec::new();

    let entries = tokio::fs::read_dir(&config_dir)
        .await
        .map_err(|e| format!("Failed to read config directory: {}", e))?;

    let mut entries = entries;
    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read directory entry: {}", e))?
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                configs.push(name.to_string());
            }
        }
    }

    emit_info(&app, format!("Found {} agent config(s)", configs.len())).ok();

    Ok(configs)
}

/// Get an agent configuration
#[tauri::command]
pub async fn get_agent_config(
    app: AppHandle,
    agent_name: String,
) -> Result<AgentConfig, String> {
    let config_path = AgentConfig::config_path(&agent_name)
        .map_err(|e| format!("Failed to get config path: {}", e))?;

    if !config_path.exists() {
        return Err(format!("Agent config '{}' not found", agent_name));
    }

    let config = AgentConfig::load(&config_path)
        .await
        .map_err(|e| format!("Failed to load agent config: {}", e))?;

    emit_info(&app, format!("Loaded config for agent '{}'", agent_name)).ok();

    Ok(config)
}

/// Update an agent configuration
#[tauri::command]
pub async fn update_agent_config(
    app: AppHandle,
    config: AgentConfig,
) -> Result<(), String> {
    let config_path = AgentConfig::config_path(&config.name)
        .map_err(|e| format!("Failed to get config path: {}", e))?;

    config
        .save(&config_path)
        .await
        .map_err(|e| format!("Failed to save agent config: {}", e))?;

    emit_success(&app, format!("Updated config for agent '{}'", config.name)).ok();

    Ok(())
}

// Helper functions

async fn load_or_create_agent_config(
    app: &AppHandle,
    agent_name: &str,
) -> Result<AgentConfig, String> {
    let config_path = AgentConfig::config_path(agent_name)
        .map_err(|e| format!("Failed to get config path: {}", e))?;

    if config_path.exists() {
        AgentConfig::load(&config_path)
            .await
            .map_err(|e| format!("Failed to load agent config: {}", e))
    } else {
        // Create default config based on agent name
        let config = match agent_name {
            "cdp-generator" => AgentConfig::default_cdp_agent(),
            "meta-agent" => AgentConfig::default_meta_agent(),
            _ => {
                return Err(format!(
                    "Unknown agent '{}'. Available agents: cdp-generator, meta-agent",
                    agent_name
                ));
            }
        };

        // Save for future use
        config
            .save(&config_path)
            .await
            .map_err(|e| format!("Failed to save agent config: {}", e))?;

        emit_info(app, format!("Created default config for '{}'", agent_name)).ok();

        Ok(config)
    }
}

async fn capture_screenshot_if_available(
    app: &AppHandle,
    state: &State<'_, AppState>,
) -> Option<PathBuf> {
    let driver_lock = state.driver.lock().await;

    if let Some(driver) = driver_lock.as_ref() {
        let temp_dir = std::env::temp_dir().join("robert-chat");
        if tokio::fs::create_dir_all(&temp_dir).await.is_ok() {
            let timestamp = chrono::Utc::now().timestamp();
            let screenshot_path = temp_dir.join(format!("chat-screenshot-{}.png", timestamp));

            if driver.screenshot_to_file(&screenshot_path).await.is_ok() {
                emit_info(app, "Captured screenshot for AI context").ok();
                return Some(screenshot_path);
            }
        }
    }

    None
}

async fn get_html_if_available(
    app: &AppHandle,
    state: &State<'_, AppState>,
) -> Option<String> {
    let driver_lock = state.driver.lock().await;

    if let Some(driver) = driver_lock.as_ref() {
        if let Ok(html) = driver.get_page_source().await {
            emit_info(app, format!("Extracted {} KB of HTML", html.len() / 1024)).ok();
            return Some(html);
        }
    }

    None
}

/// Feedback for an action
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionFeedback {
    /// ID of the action (for tracking)
    pub action_id: String,

    /// Whether the action was successful (thumbs up = true, thumbs down = false)
    pub positive: bool,

    /// Optional text feedback from the user
    pub comment: Option<String>,

    /// Agent that performed the action
    pub agent_name: String,

    /// Original user request
    pub original_request: String,

    /// What went wrong (for negative feedback)
    pub error_description: Option<String>,
}

/// Submit feedback and optionally update agent configuration
#[tauri::command]
pub async fn submit_action_feedback(
    app: AppHandle,
    feedback: ActionFeedback,
) -> Result<String, String> {
    emit_info(
        &app,
        format!(
            "Processing {} feedback for {}",
            if feedback.positive {
                "positive"
            } else {
                "negative"
            },
            feedback.agent_name
        ),
    )
    .ok();

    // If negative feedback, trigger config update workflow
    if !feedback.positive {
        // Build feedback message for meta-agent
        let mut feedback_message = format!(
            "The agent failed to complete the following request:\n\nOriginal request: {}\n",
            feedback.original_request
        );

        if let Some(comment) = &feedback.comment {
            feedback_message.push_str(&format!("\nUser feedback: {}\n", comment));
        }

        if let Some(error) = &feedback.error_description {
            feedback_message.push_str(&format!("\nError: {}\n", error));
        }

        feedback_message.push_str("\nPlease update the agent's instructions to prevent this issue in the future.");

        // Load the original agent config (not used directly but kept for validation)
        let _agent_config = load_or_create_agent_config(&app, &feedback.agent_name).await?;

        // Use meta-agent to update config
        let meta_agent = load_or_create_agent_config(&app, "meta-agent").await?;

        let executor = WorkflowExecutor::new();

        emit_claude_processing(&app, "Analyzing feedback and updating agent config...").ok();

        match executor
            .execute(
                WorkflowType::ConfigUpdate,
                feedback_message,
                &meta_agent,
                None,
                None,
                None,
            )
            .await
        {
            Ok(result) => {
                if result.success {
                    emit_success(
                        &app,
                        format!(
                            "Agent '{}' configuration updated based on feedback",
                            feedback.agent_name
                        ),
                    )
                    .ok();
                    Ok(format!(
                        "Successfully updated {} configuration",
                        feedback.agent_name
                    ))
                } else {
                    emit_error(
                        &app,
                        "Failed to update agent configuration".to_string(),
                        result.error.clone(),
                    )
                    .ok();
                    Err(result.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to process feedback: {}", e);
                emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
                Err(error_msg)
            }
        }
    } else {
        // Positive feedback - just log it
        emit_success(
            &app,
            format!(
                "Positive feedback recorded for {}",
                feedback.agent_name
            ),
        )
        .ok();
        Ok("Thank you for your feedback!".to_string())
    }
}
