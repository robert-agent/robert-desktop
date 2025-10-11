//! Step Frame Capture for Browser Workflows
//!
//! This module provides functionality to capture detailed "step frames" during browser automation.
//! Each frame represents a moment in time with a screenshot, DOM state, and action context.
//!
//! Based on the Step Frame Schema specification in agent-formats/specs/STEP_FRAME_SCHEMA.md

use crate::error::{BrowserError, Result};
use crate::ChromeDriver;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// ===== STEP FRAME STRUCTS =====

/// A complete step frame capturing a moment in a browser workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepFrame {
    /// Unique frame identifier (sequential starting from 0)
    pub frame_id: usize,

    /// ISO 8601 timestamp when frame was captured
    pub timestamp: String,

    /// Milliseconds elapsed since workflow start
    pub elapsed_ms: u64,

    /// Visual state (screenshot)
    pub screenshot: ScreenshotInfo,

    /// DOM state
    pub dom: DomInfo,

    /// User/Agent action being performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionInfo>,

    /// Natural language transcript
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<TranscriptInfo>,
}

/// Screenshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotInfo {
    /// Relative or absolute path to screenshot file
    pub path: String,

    /// Image format (png, jpeg, webp)
    pub format: String,

    /// File size in bytes
    pub size_bytes: usize,

    /// Image dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,

    /// SHA-256 hash for deduplication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

/// Image or viewport dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// DOM state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomInfo {
    /// Current page URL
    pub url: String,

    /// Page title
    pub title: String,

    /// Path to saved HTML file (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_path: Option<String>,

    /// SHA-256 hash of HTML content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_hash: Option<String>,

    /// Interactive elements on the page (optional, can be expensive to collect)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive_elements: Option<Vec<InteractiveElement>>,
}

/// An interactive element on the page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveElement {
    pub selector: String,
    pub tag: String,
    pub text: String,
    pub is_visible: bool,
    pub is_enabled: bool,
}

/// Action being performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionInfo {
    /// Action type
    pub action_type: String,

    /// High-level description of intent
    pub intent: String,

    /// CSS selector or description of target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Natural language transcript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptInfo {
    /// Description of what is happening
    pub action_description: String,

    /// Why this action was chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,

    /// What should happen next
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_outcome: Option<String>,
}

// ===== CAPTURE OPTIONS =====

/// Options for capturing a step frame
#[derive(Debug, Clone)]
pub struct CaptureOptions {
    /// Directory to save screenshots
    pub screenshot_dir: PathBuf,

    /// Directory to save DOM HTML files (optional)
    pub dom_dir: Option<PathBuf>,

    /// Screenshot format (png, jpeg)
    pub screenshot_format: ScreenshotFormat,

    /// Whether to save the HTML DOM
    pub save_html: bool,

    /// Whether to compute SHA-256 hashes
    pub compute_hashes: bool,

    /// Whether to extract interactive elements (expensive)
    pub extract_interactive_elements: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ScreenshotFormat {
    Png,
    Jpeg,
}

impl Default for CaptureOptions {
    fn default() -> Self {
        Self {
            screenshot_dir: PathBuf::from("./screenshots"),
            dom_dir: Some(PathBuf::from("./dom")),
            screenshot_format: ScreenshotFormat::Png,
            save_html: true,
            compute_hashes: true,
            extract_interactive_elements: false,
        }
    }
}

// ===== CAPTURE FUNCTION =====

/// Captures a step frame from the current browser state
///
/// # Arguments
///
/// * `driver` - Reference to the ChromeDriver
/// * `frame_id` - Sequential frame identifier
/// * `elapsed_ms` - Milliseconds since workflow start
/// * `options` - Capture options
/// * `user_instruction` - Optional user instruction text
/// * `action_info` - Optional action being performed
///
/// # Returns
///
/// A `StepFrame` with all captured information
///
/// # Errors
///
/// Returns error if:
/// - Cannot access current browser page (fail fast)
/// - Screenshot capture fails
/// - DOM retrieval fails
/// - File I/O fails
///
/// # Example
///
/// ```no_run
/// use robert_webdriver::{ChromeDriver, ConnectionMode};
/// use robert_webdriver::step_frame::{capture_step_frame, CaptureOptions, ActionInfo};
///
/// # async fn example() -> anyhow::Result<()> {
/// let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
///     chrome_path: None,
///     no_sandbox: true,
///     headless: true,
/// }).await?;
///
/// driver.navigate("https://example.com").await?;
///
/// let options = CaptureOptions::default();
/// let action = Some(ActionInfo {
///     action_type: "navigate".to_string(),
///     intent: "Navigate to example.com".to_string(),
///     target: None,
/// });
///
/// let frame = capture_step_frame(&driver, 0, 0, &options, None, action).await?;
/// println!("Captured frame: {:?}", frame);
/// # Ok(())
/// # }
/// ```
pub async fn capture_step_frame(
    driver: &ChromeDriver,
    frame_id: usize,
    elapsed_ms: u64,
    options: &CaptureOptions,
    user_instruction: Option<String>,
    action_info: Option<ActionInfo>,
) -> Result<StepFrame> {
    log::info!("╔═══════════════════════════════════════════════════════════╗");
    log::info!(
        "║  📸 CAPTURING STEP FRAME {}                              ║",
        frame_id
    );
    log::info!("╚═══════════════════════════════════════════════════════════╝");

    if let Some(ref instruction) = user_instruction {
        log::info!("📝 User instruction: {}", instruction);
    }
    if let Some(ref action) = action_info {
        log::info!("🎯 Action: {} - {}", action.action_type, action.intent);
    }
    log::info!("⏱️  Elapsed: {}ms", elapsed_ms);

    // 1. FAIL FAST: Access current page to verify connection
    log::debug!("🔍 Verifying browser connection...");
    let page = driver.current_page().await.map_err(|e| {
        log::error!("❌ Failed to access browser page: {}", e);
        BrowserError::Other(format!(
            "Failed to access browser page (connection failed): {}",
            e
        ))
    })?;

    // Verify page is accessible by getting URL
    let _ = page.url().await.map_err(|e| {
        log::error!("❌ Failed to get page URL: {}", e);
        BrowserError::Other(format!(
            "Failed to get page URL (browser not responding): {}",
            e
        ))
    })?;

    log::debug!("✓ Browser connection verified");

    // 2. TAKE SCREENSHOT
    log::info!("📸 Capturing screenshot...");
    let screenshot_filename = format!(
        "frame_{:04}.{}",
        frame_id,
        format_extension(options.screenshot_format)
    );
    let screenshot_path = options.screenshot_dir.join(&screenshot_filename);
    log::debug!("Screenshot path: {:?}", screenshot_path);

    // Ensure screenshot directory exists
    tokio::fs::create_dir_all(&options.screenshot_dir)
        .await
        .map_err(|e| {
            log::error!("❌ Failed to create screenshot directory: {}", e);
            BrowserError::Other(format!("Failed to create screenshot directory: {}", e))
        })?;

    // Capture screenshot
    driver.screenshot_to_file(&screenshot_path).await?;
    log::info!("✓ Screenshot captured: {}", screenshot_filename);

    // Get screenshot file size
    let screenshot_metadata = tokio::fs::metadata(&screenshot_path)
        .await
        .map_err(|e| BrowserError::Other(format!("Failed to read screenshot metadata: {}", e)))?;

    let screenshot_size = screenshot_metadata.len() as usize;

    // Optionally compute screenshot hash
    let screenshot_hash = if options.compute_hashes {
        Some(compute_file_hash(&screenshot_path).await?)
    } else {
        None
    };

    // 3. SAVE DOM
    log::info!("📄 Extracting DOM...");
    let url = driver.current_url().await?;
    let title = driver.title().await?;
    log::debug!("URL: {}", url);
    log::debug!("Title: {}", title);
    let html_content = driver.get_page_source().await?;
    log::info!("✓ DOM extracted ({} KB)", html_content.len() / 1024);

    let (html_path, html_hash) = if options.save_html {
        if let Some(dom_dir) = &options.dom_dir {
            // Ensure DOM directory exists
            tokio::fs::create_dir_all(dom_dir).await.map_err(|e| {
                BrowserError::Other(format!("Failed to create DOM directory: {}", e))
            })?;

            let html_filename = format!("frame_{:04}.html", frame_id);
            let html_file_path = dom_dir.join(&html_filename);

            // Save HTML to file
            tokio::fs::write(&html_file_path, &html_content)
                .await
                .map_err(|e| BrowserError::Other(format!("Failed to write HTML file: {}", e)))?;

            // Compute hash if requested
            let hash = if options.compute_hashes {
                Some(compute_string_hash(&html_content))
            } else {
                None
            };

            (Some(html_file_path.to_string_lossy().to_string()), hash)
        } else {
            // No DOM directory specified, just compute hash if requested
            let hash = if options.compute_hashes {
                Some(compute_string_hash(&html_content))
            } else {
                None
            };
            (None, hash)
        }
    } else {
        (None, None)
    };

    // 4. EXTRACT INTERACTIVE ELEMENTS (optional, expensive)
    let interactive_elements = if options.extract_interactive_elements {
        log::info!("🔍 Extracting interactive elements...");
        let elements = extract_interactive_elements_from_page(driver).await?;
        log::info!("✓ Found {} interactive elements", elements.len());
        Some(elements)
    } else {
        None
    };

    // 5. BUILD TRANSCRIPT
    let transcript = if let Some(instruction) = user_instruction {
        Some(TranscriptInfo {
            action_description: instruction.clone(),
            reasoning: None,
            expected_outcome: None,
        })
    } else {
        action_info.as_ref().map(|action| TranscriptInfo {
            action_description: action.intent.clone(),
            reasoning: None,
            expected_outcome: None,
        })
    };

    // 6. CONSTRUCT STEP FRAME
    log::info!("✅ Step frame {} captured successfully", frame_id);
    log::info!("   Screenshot: {} KB", screenshot_size / 1024);
    log::info!("   DOM: {} KB", html_content.len() / 1024);
    log::info!("   URL: {}", url);

    Ok(StepFrame {
        frame_id,
        timestamp: chrono::Utc::now().to_rfc3339(),
        elapsed_ms,
        screenshot: ScreenshotInfo {
            path: screenshot_path.to_string_lossy().to_string(),
            format: format_string(options.screenshot_format),
            size_bytes: screenshot_size,
            dimensions: None, // Could be extracted from image metadata
            hash: screenshot_hash,
        },
        dom: DomInfo {
            url,
            title,
            html_path,
            html_hash,
            interactive_elements,
        },
        action: action_info,
        transcript,
    })
}

// ===== HELPER FUNCTIONS =====

fn format_extension(format: ScreenshotFormat) -> &'static str {
    match format {
        ScreenshotFormat::Png => "png",
        ScreenshotFormat::Jpeg => "jpg",
    }
}

fn format_string(format: ScreenshotFormat) -> String {
    match format {
        ScreenshotFormat::Png => "png".to_string(),
        ScreenshotFormat::Jpeg => "jpeg".to_string(),
    }
}

/// Compute SHA-256 hash of a file
async fn compute_file_hash(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};

    let contents = tokio::fs::read(path)
        .await
        .map_err(|e| BrowserError::Other(format!("Failed to read file for hashing: {}", e)))?;

    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let hash = hasher.finalize();

    Ok(format!("{:x}", hash))
}

/// Compute SHA-256 hash of a string
fn compute_string_hash(content: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let hash = hasher.finalize();

    format!("{:x}", hash)
}

/// Extract interactive elements from the current page
async fn extract_interactive_elements_from_page(
    driver: &ChromeDriver,
) -> Result<Vec<InteractiveElement>> {
    // JavaScript to extract interactive elements
    let js_code = r#"
        (() => {
            const selectors = ['button', 'a', 'input', 'select', 'textarea'];
            const elements = [];

            selectors.forEach(tag => {
                const nodes = document.querySelectorAll(tag);
                nodes.forEach((el, idx) => {
                    if (idx < 50) { // Limit to first 50 of each type
                        const rect = el.getBoundingClientRect();
                        const isVisible = rect.width > 0 && rect.height > 0;
                        elements.push({
                            selector: `${tag}:nth-of-type(${idx + 1})`,
                            tag: tag,
                            text: el.textContent ? el.textContent.trim().substring(0, 100) : '',
                            is_visible: isVisible,
                            is_enabled: !el.disabled
                        });
                    }
                });
            });

            return elements;
        })()
    "#;

    let result = driver.execute_script(js_code).await?;

    // Parse the result
    let elements: Vec<InteractiveElement> = serde_json::from_value(result).unwrap_or_default();

    Ok(elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capture_options() {
        let options = CaptureOptions::default();
        assert_eq!(options.screenshot_dir, PathBuf::from("./screenshots"));
        assert_eq!(options.dom_dir, Some(PathBuf::from("./dom")));
        assert!(options.save_html);
        assert!(options.compute_hashes);
        assert!(!options.extract_interactive_elements);
    }

    #[test]
    fn test_format_extension() {
        assert_eq!(format_extension(ScreenshotFormat::Png), "png");
        assert_eq!(format_extension(ScreenshotFormat::Jpeg), "jpg");
    }

    #[test]
    fn test_format_string() {
        assert_eq!(format_string(ScreenshotFormat::Png), "png");
        assert_eq!(format_string(ScreenshotFormat::Jpeg), "jpeg");
    }

    #[test]
    fn test_compute_string_hash() {
        let hash1 = compute_string_hash("hello world");
        let hash2 = compute_string_hash("hello world");
        let hash3 = compute_string_hash("different");

        // Same input should produce same hash
        assert_eq!(hash1, hash2);

        // Different input should produce different hash
        assert_ne!(hash1, hash3);

        // Hash should be 64 hex characters (SHA-256)
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_step_frame_serialization() {
        let frame = StepFrame {
            frame_id: 0,
            timestamp: "2025-10-11T12:00:00Z".to_string(),
            elapsed_ms: 0,
            screenshot: ScreenshotInfo {
                path: "./screenshots/frame_0000.png".to_string(),
                format: "png".to_string(),
                size_bytes: 12345,
                dimensions: Some(Dimensions {
                    width: 1920,
                    height: 1080,
                }),
                hash: Some("abc123".to_string()),
            },
            dom: DomInfo {
                url: "https://example.com".to_string(),
                title: "Example".to_string(),
                html_path: Some("./dom/frame_0000.html".to_string()),
                html_hash: Some("def456".to_string()),
                interactive_elements: None,
            },
            action: Some(ActionInfo {
                action_type: "navigate".to_string(),
                intent: "Navigate to example.com".to_string(),
                target: None,
            }),
            transcript: Some(TranscriptInfo {
                action_description: "Navigating to example.com".to_string(),
                reasoning: Some("User requested navigation".to_string()),
                expected_outcome: Some("Page should load".to_string()),
            }),
        };

        // Test serialization
        let json = serde_json::to_string_pretty(&frame).unwrap();
        assert!(json.contains("frame_id"));
        assert!(json.contains("screenshot"));
        assert!(json.contains("dom"));

        // Test deserialization
        let deserialized: StepFrame = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.frame_id, 0);
        assert_eq!(deserialized.screenshot.size_bytes, 12345);
    }
}
