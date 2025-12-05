//! Markdown command template parser
//!
//! This module handles parsing of markdown-based command templates with YAML frontmatter.
//! Commands are stored as markdown files that describe tasks for an AI agent to execute.
//!
//! # Markdown Format
//!
//! Commands consist of:
//! - YAML frontmatter: metadata (command_name, description, version, etc.)
//! - Markdown sections: Parameters, Rules, Checklist
//! - Optional sections: CDP Script Template, Generative UI
//!
//! # Example
//!
//! ```markdown
//! ---
//! command_name: navigate-to-url
//! description: Navigate to a specified URL
//! browser_profile: null
//! created_at: 2025-10-21T00:00:00Z
//! updated_at: 2025-10-21T00:00:00Z
//! version: 1.0.0
//! changelog: []
//! ---
//!
//! # Navigate to URL
//!
//! ## Parameters
//! - url (text, required): The URL to navigate to
//!
//! ## Rules
//! - URL must be a valid HTTP or HTTPS URL
//! - Wait for page load before continuing
//!
//! ## Checklist
//! - [ ] Navigate to the specified URL
//! - [ ] Confirm page has loaded successfully
//! ```

use crate::profiles::types::{Command, CommandFrontmatter, CommandParameter, ParameterType};
use pulldown_cmark::{Event, HeadingLevel, Parser as MarkdownParser, Tag};
use serde_yaml;
use thiserror::Error;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Error, Debug)]
pub enum MarkdownParseError {
    /// Missing or invalid YAML frontmatter
    #[error("Missing or invalid YAML frontmatter: {0}")]
    InvalidFrontmatter(String),

    /// YAML deserialization error
    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// Missing required section
    #[error("Missing required section: {0}")]
    MissingSection(String),

    /// Invalid parameter format
    #[error("Invalid parameter format: {0}")]
    InvalidParameter(String),

    /// Invalid JSON in optional sections
    #[error("Invalid JSON in {0} section: {1}")]
    InvalidJson(String, String),
}

pub type Result<T> = std::result::Result<T, MarkdownParseError>;

// ============================================================================
// Markdown Parser
// ============================================================================

/// Parse a markdown command template into a structured Command
///
/// # Parameters
/// - `markdown`: The markdown content with YAML frontmatter
///
/// # Returns
/// - `Command` struct with parsed frontmatter and sections
///
/// # Errors
/// - `InvalidFrontmatter` if frontmatter is missing or invalid
/// - `MissingSection` if required sections are missing
/// - `InvalidParameter` if parameter format is incorrect
pub fn parse_command_template(markdown: &str) -> Result<Command> {
    // Split frontmatter and body
    let (frontmatter_str, body) = extract_frontmatter(markdown)?;

    // Parse YAML frontmatter
    let frontmatter: CommandFrontmatter = serde_yaml::from_str(&frontmatter_str)?;

    // Parse markdown sections
    let sections = parse_sections(&body);

    // Extract parameters
    let parameters = parse_parameters_section(&sections)?;

    // Extract rules
    let rules = parse_list_section(&sections, "Rules")
        .ok_or_else(|| MarkdownParseError::MissingSection("Rules".to_string()))?;

    // Extract checklist
    let checklist = parse_list_section(&sections, "Checklist")
        .ok_or_else(|| MarkdownParseError::MissingSection("Checklist".to_string()))?;

    // Extract optional CDP script template
    let cdp_script_template = parse_code_block_section(&sections, "CDP Script Template");

    // For now, generative_ui is optional and not parsed (Phase 4 feature)
    let generative_ui = None;

    Ok(Command {
        frontmatter,
        parameters,
        rules,
        checklist,
        generative_ui,
        cdp_script_template,
    })
}

/// Generate markdown template from a Command struct
///
/// # Parameters
/// - `command`: The Command to serialize
///
/// # Returns
/// - Markdown string with YAML frontmatter
pub fn generate_command_template(command: &Command) -> Result<String> {
    let mut output = String::new();

    // Generate YAML frontmatter
    output.push_str("---\n");
    let frontmatter_yaml = serde_yaml::to_string(&command.frontmatter)?;
    output.push_str(&frontmatter_yaml);
    output.push_str("---\n\n");

    // Generate title (use command name)
    output.push_str(&format!("# {}\n\n", command.frontmatter.command_name));

    // Generate Parameters section
    output.push_str("## Parameters\n");
    for param in &command.parameters {
        let type_str = param_type_to_string(&param.param_type);
        let required_str = if param.required {
            "required"
        } else {
            "optional"
        };
        output.push_str(&format!(
            "- {} ({}, {}): {}\n",
            param.name, type_str, required_str, param.label
        ));
    }
    output.push('\n');

    // Generate Rules section
    output.push_str("## Rules\n");
    for rule in &command.rules {
        output.push_str(&format!("- {}\n", rule));
    }
    output.push('\n');

    // Generate Checklist section
    output.push_str("## Checklist\n");
    for item in &command.checklist {
        output.push_str(&format!("- [ ] {}\n", item));
    }
    output.push('\n');

    // Generate optional CDP Script Template section
    if let Some(cdp_script) = &command.cdp_script_template {
        output.push_str("## CDP Script Template (Optional)\n");
        output.push_str("```json\n");
        output.push_str(cdp_script);
        output.push_str("\n```\n");
    }

    Ok(output)
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extract YAML frontmatter from markdown
///
/// Returns (frontmatter_content, remaining_markdown)
fn extract_frontmatter(markdown: &str) -> Result<(String, String)> {
    let trimmed = markdown.trim_start();

    if !trimmed.starts_with("---") {
        return Err(MarkdownParseError::InvalidFrontmatter(
            "Markdown must start with --- for frontmatter".to_string(),
        ));
    }

    // Find the closing ---
    let rest = &trimmed[3..];
    if let Some(end_pos) = rest.find("\n---\n") {
        let frontmatter = rest[..end_pos].trim().to_string();
        let body = rest[end_pos + 5..].trim().to_string();
        Ok((frontmatter, body))
    } else {
        Err(MarkdownParseError::InvalidFrontmatter(
            "Frontmatter closing --- not found".to_string(),
        ))
    }
}

/// Parse markdown into sections by h2 headers
fn parse_sections(markdown: &str) -> std::collections::HashMap<String, String> {
    let mut sections = std::collections::HashMap::new();
    let mut current_section: Option<String> = None;
    let mut current_content = String::new();

    let parser = MarkdownParser::new(markdown);

    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H2, ..)) => {
                // Save previous section if exists
                if let Some(section_name) = current_section.take() {
                    sections.insert(section_name, current_content.trim().to_string());
                    current_content.clear();
                }
            }
            Event::Text(text) => {
                if current_section.is_none() {
                    // This is the section header text
                    current_section = Some(text.to_string());
                } else {
                    // This is section content
                    current_content.push_str(&text);
                }
            }
            Event::End(Tag::Heading(HeadingLevel::H2, ..)) => {
                // Section header ended, content will follow
            }
            Event::Start(Tag::List(_)) | Event::End(Tag::List(_)) => {
                // List boundaries
            }
            Event::Start(Tag::Item) => {
                // List item start
            }
            Event::End(Tag::Item) => {
                current_content.push('\n');
            }
            Event::Start(Tag::CodeBlock(_)) => {
                current_content.push_str("```\n");
            }
            Event::End(Tag::CodeBlock(_)) => {
                current_content.push_str("```\n");
            }
            Event::Code(code) => {
                current_content.push('`');
                current_content.push_str(&code);
                current_content.push('`');
            }
            Event::SoftBreak | Event::HardBreak => {
                current_content.push('\n');
            }
            _ => {}
        }
    }

    // Save the last section
    if let Some(section_name) = current_section {
        sections.insert(section_name, current_content.trim().to_string());
    }

    sections
}

/// Parse parameters section into CommandParameter structs
fn parse_parameters_section(
    sections: &std::collections::HashMap<String, String>,
) -> Result<Vec<CommandParameter>> {
    let params_text = sections
        .get("Parameters")
        .ok_or_else(|| MarkdownParseError::MissingSection("Parameters".to_string()))?;

    let mut parameters = Vec::new();

    for line in params_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Expected format: "- name (type, required/optional): Label"
        if let Some(param) = parse_parameter_line(line)? {
            parameters.push(param);
        }
    }

    Ok(parameters)
}

/// Parse a single parameter line
///
/// Format: "- name (type, required/optional): Label"
fn parse_parameter_line(line: &str) -> Result<Option<CommandParameter>> {
    let line = line.trim_start_matches('-').trim();

    if line.is_empty() {
        return Ok(None);
    }

    // Split by ':'
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(MarkdownParseError::InvalidParameter(format!(
            "Parameter line must contain ':' separator: {}",
            line
        )));
    }

    let name_and_meta = parts[0].trim();
    let label = parts[1].trim().to_string();

    // Extract name and metadata
    if let Some(paren_pos) = name_and_meta.find('(') {
        let name = name_and_meta[..paren_pos].trim().to_string();
        let meta_end = name_and_meta.find(')').ok_or_else(|| {
            MarkdownParseError::InvalidParameter("Missing closing ')'".to_string())
        })?;
        let meta = &name_and_meta[paren_pos + 1..meta_end];

        // Parse metadata: "type, required/optional"
        let meta_parts: Vec<&str> = meta.split(',').map(|s| s.trim()).collect();
        if meta_parts.len() != 2 {
            return Err(MarkdownParseError::InvalidParameter(format!(
                "Parameter metadata must be 'type, required/optional': {}",
                line
            )));
        }

        let param_type = parse_parameter_type(meta_parts[0])?;
        let required = meta_parts[1] == "required";

        Ok(Some(CommandParameter {
            name,
            param_type,
            label,
            placeholder: None,
            required,
            default: None,
        }))
    } else {
        Err(MarkdownParseError::InvalidParameter(format!(
            "Parameter must have metadata in parentheses: {}",
            line
        )))
    }
}

/// Parse parameter type string into ParameterType enum
fn parse_parameter_type(type_str: &str) -> Result<ParameterType> {
    match type_str {
        "text" => Ok(ParameterType::TextInput),
        "short_text" => Ok(ParameterType::ShortText { max_length: None }),
        "checkbox" => Ok(ParameterType::Checkbox),
        "date" => Ok(ParameterType::DatePicker),
        "color" => Ok(ParameterType::ColorPicker),
        _ => Err(MarkdownParseError::InvalidParameter(format!(
            "Unknown parameter type: {}",
            type_str
        ))),
    }
}

/// Convert ParameterType to string for markdown generation
fn param_type_to_string(param_type: &ParameterType) -> &str {
    match param_type {
        ParameterType::TextInput => "text",
        ParameterType::ShortText { .. } => "short_text",
        ParameterType::Dropdown { .. } => "dropdown",
        ParameterType::Radio { .. } => "radio",
        ParameterType::Checkbox => "checkbox",
        ParameterType::Slider { .. } => "slider",
        ParameterType::ColorPicker => "color",
        ParameterType::DatePicker => "date",
    }
}

/// Parse a list section (Rules, Checklist) into Vec<String>
fn parse_list_section(
    sections: &std::collections::HashMap<String, String>,
    section_name: &str,
) -> Option<Vec<String>> {
    sections.get(section_name).map(|text| {
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                // Remove markdown list markers (-, *, [ ], [x])
                line.trim_start_matches('-')
                    .trim_start_matches('*')
                    .trim_start_matches("[ ]")
                    .trim_start_matches("[x]")
                    .trim()
                    .to_string()
            })
            .collect()
    })
}

/// Parse code block section (CDP Script Template)
fn parse_code_block_section(
    sections: &std::collections::HashMap<String, String>,
    section_name: &str,
) -> Option<String> {
    // Try exact match first
    if let Some(text) = sections.get(section_name) {
        return Some(
            text.trim()
                .trim_start_matches("```json")
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
                .to_string(),
        );
    }

    // Try with "(Optional)" suffix
    let optional_name = format!("{} (Optional)", section_name);
    sections.get(&optional_name).map(|text| {
        text.trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string()
    })
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    const EXAMPLE_MARKDOWN: &str = r#"---
command_name: navigate-to-url
description: Navigate to a specified URL
browser_profile: null
created_at: 2025-10-21T00:00:00Z
updated_at: 2025-10-21T00:00:00Z
version: 1.0.0
changelog: []
---

# Navigate to URL

## Parameters
- url (text, required): The URL to navigate to
- wait_time (short_text, optional): Seconds to wait after navigation

## Rules
- URL must be a valid HTTP or HTTPS URL
- Wait for page load before continuing
- Handle navigation errors gracefully

## Checklist
- [ ] Navigate to the specified URL
- [ ] Confirm page has loaded successfully
- [ ] Check for any error messages
"#;

    #[test]
    fn test_extract_frontmatter() {
        let result = extract_frontmatter(EXAMPLE_MARKDOWN);
        assert!(result.is_ok());

        let (frontmatter, body) = result.unwrap();
        assert!(frontmatter.contains("command_name: navigate-to-url"));
        assert!(body.contains("# Navigate to URL"));
    }

    #[test]
    fn test_extract_frontmatter_missing() {
        let markdown = "# No frontmatter\nJust content";
        let result = extract_frontmatter(markdown);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_sections() {
        let body = r#"
# Navigate to URL

## Parameters
- url (text, required): The URL

## Rules
- Must be valid URL

## Checklist
- [ ] Navigate
"#;
        let sections = parse_sections(body);

        assert!(sections.contains_key("Parameters"));
        assert!(sections.contains_key("Rules"));
        assert!(sections.contains_key("Checklist"));
    }

    #[test]
    fn test_parse_parameter_line() {
        let line = "- url (text, required): The URL to navigate to";
        let result = parse_parameter_line(line).unwrap();

        assert!(result.is_some());
        let param = result.unwrap();
        assert_eq!(param.name, "url");
        assert_eq!(param.label, "The URL to navigate to");
        assert!(param.required);
        assert!(matches!(param.param_type, ParameterType::TextInput));
    }

    #[test]
    fn test_parse_parameter_line_optional() {
        let line = "- timeout (short_text, optional): Timeout in seconds";
        let result = parse_parameter_line(line).unwrap();

        assert!(result.is_some());
        let param = result.unwrap();
        assert_eq!(param.name, "timeout");
        assert!(!param.required);
    }

    #[test]
    fn test_parse_command_template() {
        let result = parse_command_template(EXAMPLE_MARKDOWN);
        assert!(result.is_ok());

        let command = result.unwrap();
        assert_eq!(command.frontmatter.command_name, "navigate-to-url");
        assert_eq!(command.parameters.len(), 2);
        assert_eq!(command.rules.len(), 3);
        assert_eq!(command.checklist.len(), 3);
    }

    #[test]
    fn test_generate_command_template() {
        let command = Command {
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
                label: "URL to visit".to_string(),
                placeholder: None,
                required: true,
                default: None,
            }],
            rules: vec!["Must be valid URL".to_string()],
            checklist: vec!["Navigate to URL".to_string()],
            generative_ui: None,
            cdp_script_template: None,
        };

        let result = generate_command_template(&command);
        assert!(result.is_ok());

        let markdown = result.unwrap();
        assert!(markdown.contains("command_name: test-command"));
        assert!(markdown.contains("## Parameters"));
        assert!(markdown.contains("url (text, required)"));
    }

    #[test]
    fn test_roundtrip_parse_generate() {
        let result = parse_command_template(EXAMPLE_MARKDOWN);
        assert!(result.is_ok());

        let command = result.unwrap();
        let generated = generate_command_template(&command).unwrap();

        // Parse the generated markdown
        let reparsed = parse_command_template(&generated);
        assert!(reparsed.is_ok());

        let reparsed_command = reparsed.unwrap();
        assert_eq!(
            command.frontmatter.command_name,
            reparsed_command.frontmatter.command_name
        );
        assert_eq!(command.parameters.len(), reparsed_command.parameters.len());
        assert_eq!(command.rules.len(), reparsed_command.rules.len());
    }

    #[test]
    fn test_parse_with_cdp_script() {
        let markdown_with_cdp = r#"---
command_name: navigate-with-cdp
description: Navigate using CDP
browser_profile: null
created_at: 2025-10-21T00:00:00Z
updated_at: 2025-10-21T00:00:00Z
version: 1.0.0
changelog: []
---

# Navigate with CDP

## Parameters
- url (text, required): URL

## Rules
- Valid URL

## Checklist
- [ ] Navigate

## CDP Script Template (Optional)
```json
{
  "method": "Page.navigate",
  "params": {"url": "{{url}}"}
}
```
"#;

        let result = parse_command_template(markdown_with_cdp);
        assert!(result.is_ok());

        let command = result.unwrap();
        assert!(command.cdp_script_template.is_some());
        let cdp = command.cdp_script_template.unwrap();
        assert!(cdp.contains("Page.navigate"));
    }
}
