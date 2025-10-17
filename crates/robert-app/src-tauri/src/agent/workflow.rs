//! Workflow execution logic for different agent tasks

use super::config::AgentConfig;
use super::prompts::{PromptContext, PromptTemplate, PromptType};
use crate::claude::{ClaudeClient, ClaudeConfig, ClaudeInput, ClaudeResponse};
use anyhow::{Context, Result};
use robert_webdriver::ChromeDriver;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Planning response types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "response_type", rename_all = "snake_case")]
pub enum PlanningResponse {
    /// Agent is ready to proceed
    Ready {
        understanding: String,
        next_step: String,
    },
    /// Agent needs clarification
    ClarificationNeeded {
        questions: Vec<ClarificationQuestion>,
        understanding: String,
    },
}

/// A question that needs clarification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarificationQuestion {
    pub question: String,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

/// Type of workflow to execute
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowType {
    /// Generate and execute CDP script
    CdpAutomation,
    /// Update agent configuration
    ConfigUpdate,
}

/// Result of workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub success: bool,
    pub workflow_type: WorkflowType,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdp_script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_report: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Clarification questions if the agent needs more information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarification: Option<Vec<ClarificationQuestion>>,
    /// Agent's understanding of the request so far
    #[serde(skip_serializing_if = "Option::is_none")]
    pub understanding: Option<String>,
}

/// Workflow executor
pub struct WorkflowExecutor;

impl WorkflowExecutor {
    /// Create a new workflow executor
    pub fn new() -> Self {
        Self
    }

    /// Execute a workflow based on user input
    pub async fn execute(
        &self,
        workflow_type: WorkflowType,
        user_message: String,
        agent_config: &AgentConfig,
        screenshot_path: Option<PathBuf>,
        html_content: Option<String>,
        driver: Option<&ChromeDriver>,
    ) -> Result<WorkflowResult> {
        match workflow_type {
            WorkflowType::CdpAutomation => {
                self.execute_cdp_workflow(
                    user_message,
                    agent_config,
                    screenshot_path,
                    html_content,
                    driver,
                )
                .await
            }
            WorkflowType::ConfigUpdate => {
                self.execute_config_update_workflow(user_message, agent_config)
                    .await
            }
        }
    }

    /// Execute CDP generation and automation workflow
    async fn execute_cdp_workflow(
        &self,
        user_message: String,
        agent_config: &AgentConfig,
        screenshot_path: Option<PathBuf>,
        html_content: Option<String>,
        driver: Option<&ChromeDriver>,
    ) -> Result<WorkflowResult> {
        log::info!("╔═══════════════════════════════════════════════════════════╗");
        log::info!("║  🤖 CDP AUTOMATION WORKFLOW                               ║");
        log::info!("╚═══════════════════════════════════════════════════════════╝");

        // Get current page context if driver is available
        let (current_url, page_title) = if let Some(driver) = driver {
            let url = driver.current_url().await.ok();
            let title = driver.title().await.ok();
            log::info!("🌐 Current URL: {}", url.as_deref().unwrap_or("N/A"));
            log::info!("📄 Page title: {}", title.as_deref().unwrap_or("N/A"));
            (url, title)
        } else {
            log::warn!("⚠️  No browser driver available");
            (None, None)
        };

        // PHASE 1: Planning - Check if clarification is needed
        log::info!("╔═══════════════════════════════════════════════════════════╗");
        log::info!("║  📋 PHASE 1: PLANNING                                     ║");
        log::info!("╚═══════════════════════════════════════════════════════════╝");
        log::info!("📝 Building planning prompt...");

        let template = PromptTemplate::new(PromptType::CdpGeneration);
        let planning_prompt = template.build_planning_prompt(
            &user_message,
            current_url.as_deref(),
            page_title.as_deref(),
            &agent_config.instructions,
        );

        log::info!("✓ Planning prompt created ({} chars)", planning_prompt.len());

        // Call Claude for planning
        let planning_response = self
            .call_claude(
                &planning_prompt,
                screenshot_path.clone(),
                html_content.clone(),
                &agent_config.settings.model,
            )
            .await
            .map_err(|e| {
                log::error!("❌ Claude planning call failed");
                log::error!("Error details: {:?}", e);
                e
            })?;

        // Parse planning response
        let cleaned_planning = planning_response.text().trim();
        let cleaned_planning = if cleaned_planning.starts_with("```json") {
            cleaned_planning
                .trim_start_matches("```json")
                .trim_end_matches("```")
                .trim()
        } else if cleaned_planning.starts_with("```") {
            cleaned_planning
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
        } else {
            cleaned_planning
        };

        log::debug!("Planning response: {}", cleaned_planning);

        let planning_result: PlanningResponse = match serde_json::from_str(cleaned_planning) {
            Ok(result) => result,
            Err(e) => {
                log::error!("❌ Failed to parse planning response as JSON");
                log::error!("Parse error: {}", e);
                log::error!("Response: {}", cleaned_planning);
                return Ok(WorkflowResult {
                    success: false,
                    workflow_type: WorkflowType::CdpAutomation,
                    message: "Failed to parse planning response".to_string(),
                    cdp_script: None,
                    execution_report: None,
                    error: Some(format!("Parse error: {}", e)),
                    clarification: None,
                    understanding: None,
                });
            }
        };

        // Check if clarification is needed
        match planning_result {
            PlanningResponse::ClarificationNeeded {
                questions,
                understanding,
            } => {
                log::info!("⚠️  Clarification needed - returning questions to user");
                log::info!("Understanding: {}", understanding);
                log::info!("Questions: {:?}", questions);

                return Ok(WorkflowResult {
                    success: false,
                    workflow_type: WorkflowType::CdpAutomation,
                    message: "Need clarification before proceeding".to_string(),
                    cdp_script: None,
                    execution_report: None,
                    error: None,
                    clarification: Some(questions),
                    understanding: Some(understanding),
                });
            }
            PlanningResponse::Ready {
                understanding,
                next_step,
            } => {
                log::info!("✓ Ready to proceed with CDP generation");
                log::info!("Understanding: {}", understanding);
                log::info!("Next step: {}", next_step);
                // Continue to Phase 2
            }
        }

        // PHASE 2: CDP Script Generation
        log::info!("╔═══════════════════════════════════════════════════════════╗");
        log::info!("║  ⚙️  PHASE 2: CDP SCRIPT GENERATION                       ║");
        log::info!("╚═══════════════════════════════════════════════════════════╝");
        log::info!("📝 Building CDP generation prompt...");

        let prompt = template.build_cdp_prompt(
            &user_message,
            current_url.as_deref(),
            page_title.as_deref(),
            &agent_config.instructions,
        );
        log::info!("✓ Template created ({} chars)", prompt.len());
        log::debug!(
            "📋 Prompt preview: {}...",
            &prompt.chars().take(150).collect::<String>()
        );

        // Call Claude to generate CDP script
        log::info!("╔═══════════════════════════════════════════════════════════╗");
        log::info!("║  🧠 SUBMITTING TO INFERENCE                               ║");
        log::info!("╚═══════════════════════════════════════════════════════════╝");
        log::info!(
            "🔮 Model: {}",
            agent_config.settings.model.as_deref().unwrap_or("default")
        );
        log::debug!(
            "📸 Screenshot: {}",
            if screenshot_path.is_some() {
                "✓"
            } else {
                "✗"
            }
        );
        log::debug!(
            "📄 HTML: {}",
            if html_content.is_some() { "✓" } else { "✗" }
        );

        let claude_response = self
            .call_claude(
                &prompt,
                screenshot_path.clone(),
                html_content.clone(),
                &agent_config.settings.model,
            )
            .await
            .map_err(|e| {
                log::error!("❌ Claude API call failed in workflow");
                log::error!("Error details: {:?}", e);
                e
            })?;

        log::info!("╔═══════════════════════════════════════════════════════════╗");
        log::info!("║  ✨ INFERENCE RESPONSE RECEIVED                           ║");
        log::info!("╚═══════════════════════════════════════════════════════════╝");
        log::info!("📏 Response length: {} chars", claude_response.text().len());
        log::debug!(
            "📋 Response preview: {}...",
            &claude_response.text().chars().take(200).collect::<String>()
        );

        // Parse the generated script
        log::info!("🔍 Parsing CDP script from JSON...");

        // Clean up the response - remove markdown code blocks if present
        let response_text = claude_response.text().trim();
        let cleaned_json = if response_text.starts_with("```json") {
            response_text
                .trim_start_matches("```json")
                .trim_end_matches("```")
                .trim()
        } else if response_text.starts_with("```") {
            response_text
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
        } else {
            response_text
        };

        log::debug!("Cleaned response length: {} chars", cleaned_json.len());
        log::debug!("Cleaned response preview: {}...", &cleaned_json.chars().take(200).collect::<String>());

        let cdp_script: robert_webdriver::CdpScript =
            match serde_json::from_str::<robert_webdriver::CdpScript>(cleaned_json) {
                Ok(script) => {
                    log::info!("✓ CDP script parsed successfully");
                    log::info!("📊 Commands in script: {}", script.cdp_commands.len());
                    script
                }
                Err(e) => {
                    log::error!("❌ Failed to parse CDP script JSON");
                    log::error!("⚠️  Parse error: {}", e);
                    log::error!("Response that failed to parse: {}", cleaned_json);

                    // Check if the response contains questions or non-JSON content
                    if cleaned_json.contains("?") && !cleaned_json.starts_with("{") {
                        log::error!("⚠️  Claude appears to have asked a question instead of generating JSON");
                        return Ok(WorkflowResult {
                            success: false,
                            workflow_type: WorkflowType::CdpAutomation,
                            message: "Claude asked for clarification instead of generating a script. Please provide more specific instructions.".to_string(),
                            cdp_script: None,
                            execution_report: None,
                            error: Some(format!("Non-JSON response: {}", cleaned_json)),
                            clarification: None,
                            understanding: None,
                        });
                    }

                    return Ok(WorkflowResult {
                        success: false,
                        workflow_type: WorkflowType::CdpAutomation,
                        message: "Failed to parse CDP script JSON".to_string(),
                        cdp_script: Some(claude_response.text().to_string()),
                        execution_report: None,
                        error: Some(format!("Parse error: {}", e)),
                        clarification: None,
                        understanding: None,
                    });
                }
            };

        // Validate the script
        log::info!("🔎 Validating CDP script...");
        if let Err(e) = cdp_script.validate() {
            log::error!("❌ CDP script validation failed");
            log::error!("⚠️  Validation error: {}", e);
            return Ok(WorkflowResult {
                success: false,
                workflow_type: WorkflowType::CdpAutomation,
                message: "CDP script validation failed".to_string(),
                cdp_script: Some(claude_response.text().to_string()),
                execution_report: None,
                error: Some(format!("Validation error: {}", e)),
                clarification: None,
                understanding: None,
            });
        }
        log::info!("✓ CDP script validation passed");

        // Execute the CDP script if driver is available
        if let Some(driver) = driver {
            log::info!("╔═══════════════════════════════════════════════════════════╗");
            log::info!("║  🎯 ATTEMPTING TO DRIVE WEBPAGE WITH CDP                 ║");
            log::info!("╚═══════════════════════════════════════════════════════════╝");
            log::info!(
                "📋 Total commands to execute: {}",
                cdp_script.cdp_commands.len()
            );

            for (i, cmd) in cdp_script.cdp_commands.iter().enumerate() {
                log::info!(
                    "  {}. {} - {}",
                    i + 1,
                    cmd.method,
                    cmd.description.as_deref().unwrap_or("")
                );
            }

            log::info!("🚀 Executing CDP script...");
            match driver.execute_cdp_script_direct(&cdp_script).await {
                Ok(report) => {
                    log::info!("╔═══════════════════════════════════════════════════════════╗");
                    log::info!("║  ✅ CDP EXECUTION SUCCESSFUL                              ║");
                    log::info!("╚═══════════════════════════════════════════════════════════╝");
                    log::info!("📊 Total commands: {}", report.total_commands);
                    log::info!("✓ Successful: {}", report.successful);
                    log::info!("✗ Failed: {}", report.failed);
                    log::info!("⏱️  Duration: {:?}", report.total_duration);

                    if !report.results.is_empty() {
                        log::debug!("📋 Command results:");
                        for result in &report.results {
                            use robert_webdriver::CommandStatus;
                            let status = match result.status {
                                CommandStatus::Success => "✓",
                                CommandStatus::Failed => "✗",
                                CommandStatus::Skipped => "⊘",
                            };
                            log::debug!("  {} Step {} - {}", status, result.step, result.method);
                            if let Some(err) = &result.error {
                                log::warn!("    ⚠️  Error: {}", err);
                            }
                        }
                    }

                    Ok(WorkflowResult {
                        success: true,
                        workflow_type: WorkflowType::CdpAutomation,
                        message: format!(
                            "Successfully executed {} commands",
                            report.total_commands
                        ),
                        cdp_script: Some(serde_json::to_string_pretty(&cdp_script)?),
                        execution_report: Some(serde_json::to_value(&report)?),
                        error: None,
                        clarification: None,
                        understanding: None,
                    })
                }
                Err(e) => {
                    log::error!("╔═══════════════════════════════════════════════════════════╗");
                    log::error!("║  ❌ CDP EXECUTION FAILED                                  ║");
                    log::error!("╚═══════════════════════════════════════════════════════════╝");
                    log::error!("⚠️  Error: {}", e);

                    Ok(WorkflowResult {
                        success: false,
                        workflow_type: WorkflowType::CdpAutomation,
                        message: "Failed to execute CDP script".to_string(),
                        cdp_script: Some(serde_json::to_string_pretty(&cdp_script)?),
                        execution_report: None,
                        error: Some(e.to_string()),
                        clarification: None,
                        understanding: None,
                    })
                }
            }
        } else {
            // No driver available, just return the generated script
            log::warn!("⚠️  Browser not available - CDP script generated but not executed");

            Ok(WorkflowResult {
                success: true,
                workflow_type: WorkflowType::CdpAutomation,
                message: "Generated CDP script (browser not available for execution)".to_string(),
                cdp_script: Some(serde_json::to_string_pretty(&cdp_script)?),
                execution_report: None,
                error: None,
                clarification: None,
                understanding: None,
            })
        }
    }

    /// Execute config update workflow
    async fn execute_config_update_workflow(
        &self,
        user_feedback: String,
        agent_config: &AgentConfig,
    ) -> Result<WorkflowResult> {
        // Serialize current config to TOML
        let current_config =
            toml::to_string_pretty(agent_config).context("Failed to serialize config")?;

        // Build prompt using template
        let template = PromptTemplate::new(PromptType::ConfigUpdate);
        let context = PromptContext {
            agent_name: agent_config.name.clone(),
            current_config: current_config.clone(),
            user_feedback: user_feedback.clone(),
            ..Default::default()
        };
        let prompt = template.build(context);

        // Call Claude to generate updated config
        let claude_response = self
            .call_claude(&prompt, None, None, &agent_config.settings.model)
            .await?;

        // Try to parse the response as TOML
        let updated_config_text = claude_response.text().trim();

        // Remove markdown code blocks if present
        let cleaned_toml = if updated_config_text.starts_with("```toml") {
            updated_config_text
                .trim_start_matches("```toml")
                .trim_end_matches("```")
                .trim()
        } else if updated_config_text.starts_with("```") {
            updated_config_text
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
        } else {
            updated_config_text
        };

        match toml::from_str::<AgentConfig>(cleaned_toml) {
            Ok(updated_config) => {
                // Save the updated config
                let config_path = AgentConfig::config_path(&agent_config.name)?;
                updated_config.save(&config_path).await?;

                Ok(WorkflowResult {
                    success: true,
                    workflow_type: WorkflowType::ConfigUpdate,
                    message: format!("Successfully updated {} configuration", agent_config.name),
                    cdp_script: None,
                    execution_report: Some(serde_json::json!({
                        "config_path": config_path.to_string_lossy(),
                        "updated_config": cleaned_toml,
                    })),
                    error: None,
                    clarification: None,
                    understanding: None,
                })
            }
            Err(e) => Ok(WorkflowResult {
                success: false,
                workflow_type: WorkflowType::ConfigUpdate,
                message: "Failed to parse updated configuration".to_string(),
                cdp_script: None,
                execution_report: None,
                error: Some(format!(
                    "Parse error: {}\n\nGenerated config:\n{}",
                    e, cleaned_toml
                )),
                clarification: None,
                understanding: None,
            }),
        }
    }

    /// Call Claude API
    async fn call_claude(
        &self,
        prompt: &str,
        screenshot_path: Option<PathBuf>,
        html_content: Option<String>,
        model: &Option<String>,
    ) -> Result<ClaudeResponse> {
        let images = screenshot_path.map(|p| vec![p]).unwrap_or_default();

        let input = ClaudeInput {
            prompt: prompt.to_string(),
            images,
            html: html_content,
        };

        let config = ClaudeConfig {
            model: model.clone(),
            skip_permissions: true,
            ..Default::default()
        };

        let client = ClaudeClient::with_config(config);
        client
            .execute(input)
            .await
            .context("Claude API call failed")
    }
}
