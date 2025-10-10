//! Prompt templates for different agent workflows

use serde::{Deserialize, Serialize};

/// Type of prompt/workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromptType {
    /// Generate CDP automation script
    CdpGeneration,
    /// Update agent config based on feedback
    ConfigUpdate,
}

/// Prompt template builder
pub struct PromptTemplate {
    prompt_type: PromptType,
}

impl PromptTemplate {
    /// Create a new prompt template
    pub fn new(prompt_type: PromptType) -> Self {
        Self { prompt_type }
    }

    /// Build CDP generation prompt
    pub fn build_cdp_prompt(
        &self,
        user_request: &str,
        current_url: Option<&str>,
        page_title: Option<&str>,
        agent_instructions: &str,
    ) -> String {
        let context = if let (Some(url), Some(title)) = (current_url, page_title) {
            format!("\nCURRENT PAGE:\n- URL: {}\n- Title: {}\n", url, title)
        } else {
            String::new()
        };

        format!(
            r#"{agent_instructions}

{context}
USER REQUEST: {user_request}

Generate a JSON CDP script that accomplishes this task.

Use the CDP command reference below:

AVAILABLE CDP COMMANDS:

1. Page.navigate - Navigate to URL
   {{"method": "Page.navigate", "params": {{"url": "https://example.com"}}}}

2. Page.captureScreenshot - Take screenshot
   {{"method": "Page.captureScreenshot", "params": {{"format": "png", "captureBeyondViewport": true}}, "save_as": "screenshot.png"}}

3. Runtime.evaluate - Execute JavaScript
   {{"method": "Runtime.evaluate", "params": {{"expression": "document.title", "returnByValue": true}}, "save_as": "result.json"}}

4. Input.insertText - Type text
   {{"method": "Input.insertText", "params": {{"text": "Hello World"}}}}

5. Input.dispatchMouseEvent - Mouse actions
   {{"method": "Input.dispatchMouseEvent", "params": {{"type": "mousePressed", "x": 100, "y": 200, "button": "left", "clickCount": 1}}}}

6. Input.dispatchKeyEvent - Keyboard actions
   {{"method": "Input.dispatchKeyEvent", "params": {{"type": "keyDown", "key": "Enter"}}}}

OUTPUT FORMAT (JSON only, no markdown):

{{
  "name": "descriptive-name",
  "description": "What this script does",
  "cdp_commands": [
    {{
      "method": "Page.navigate",
      "params": {{"url": "..."}},
      "description": "Navigate to page"
    }},
    {{
      "method": "Runtime.evaluate",
      "params": {{"expression": "...", "returnByValue": true}},
      "save_as": "optional-output.json",
      "description": "Perform action"
    }}
  ]
}}

Generate the CDP script now. Output ONLY valid JSON."#,
            agent_instructions = agent_instructions,
            context = context,
            user_request = user_request
        )
    }

    /// Build config update prompt
    pub fn build_config_update_prompt(
        &self,
        agent_name: &str,
        current_config: &str,
        user_feedback: &str,
        failure_context: Option<&str>,
    ) -> String {
        let failure_section = if let Some(context) = failure_context {
            format!("\nFAILURE CONTEXT:\n{}\n", context)
        } else {
            String::new()
        };

        format!(
            r#"You are updating the configuration for the "{agent_name}" agent based on user feedback.

CURRENT CONFIGURATION:
```toml
{current_config}
```

USER FEEDBACK:
{user_feedback}
{failure_section}
TASK:
1. Analyze what went wrong based on the feedback
2. Update the agent's instructions to fix the issue
3. Output the COMPLETE updated TOML configuration

RULES:
- Keep all existing fields
- Only modify the 'instructions' field and 'settings' if necessary
- Make minimal, targeted changes
- Ensure the TOML is valid
- Add specific guidance to prevent this error in the future

Output the updated TOML configuration now:"#,
            agent_name = agent_name,
            current_config = current_config,
            user_feedback = user_feedback,
            failure_section = failure_section
        )
    }

    /// Build a prompt based on the template type
    pub fn build(&self, context: PromptContext) -> String {
        match self.prompt_type {
            PromptType::CdpGeneration => self.build_cdp_prompt(
                &context.user_request,
                context.current_url.as_deref(),
                context.page_title.as_deref(),
                &context.agent_instructions,
            ),
            PromptType::ConfigUpdate => self.build_config_update_prompt(
                &context.agent_name,
                &context.current_config,
                &context.user_feedback,
                context.failure_context.as_deref(),
            ),
        }
    }
}

/// Context for building prompts
#[derive(Debug, Clone, Default)]
pub struct PromptContext {
    pub user_request: String,
    pub current_url: Option<String>,
    pub page_title: Option<String>,
    pub agent_instructions: String,
    pub agent_name: String,
    pub current_config: String,
    pub user_feedback: String,
    pub failure_context: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdp_generation_prompt() {
        let template = PromptTemplate::new(PromptType::CdpGeneration);
        let prompt = template.build_cdp_prompt(
            "Click the login button",
            Some("https://example.com"),
            Some("Example Page"),
            "You are an automation expert",
        );

        assert!(prompt.contains("Click the login button"));
        assert!(prompt.contains("example.com"));
        assert!(prompt.contains("CDP COMMANDS"));
    }

    #[test]
    fn test_config_update_prompt() {
        let template = PromptTemplate::new(PromptType::ConfigUpdate);
        let prompt = template.build_config_update_prompt(
            "test-agent",
            "name = \"test\"\ninstructions = \"old\"",
            "The agent failed to handle errors",
            Some("Error: element not found"),
        );

        assert!(prompt.contains("test-agent"));
        assert!(prompt.contains("failed to handle errors"));
        assert!(prompt.contains("element not found"));
    }
}
