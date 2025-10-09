//! Integration tests designed to run headlessly in CI/CD environments
//!
//! Note: These tests must run sequentially due to Chrome profile directory conflicts.
//! Use: cargo test --test headless_integration -- --test-threads=1

use robert_webdriver::{CdpScript, CdpCommand, ChromeDriver, ConnectionMode};
use std::time::Duration;
use tokio::time::sleep;

/// Helper to create a headless driver for testing
async fn create_headless_driver() -> anyhow::Result<ChromeDriver> {
    ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: true,  // Required for CI environments
        headless: true,    // Always headless for these tests
    })
    .await
    .map_err(|e| anyhow::anyhow!("Failed to launch Chrome: {}", e))
}

#[tokio::test]
#[ignore] // Flaky when run with other tests due to Chrome resource contention
async fn test_basic_navigation_headless() -> anyhow::Result<()> {
    let driver = create_headless_driver().await?;

    // Navigate to example.com
    driver.navigate("https://example.com").await?;

    // Wait for page to load - give it more time for headless mode
    sleep(Duration::from_secs(3)).await;

    // Try multiple times to get the correct title
    let mut attempts = 0;
    let max_attempts = 5;
    let mut title = String::new();

    while attempts < max_attempts {
        title = driver.title().await?;
        println!("✅ Attempt {}: Page title: {}", attempts + 1, title);

        if title.to_lowercase().contains("example") {
            println!("✅ Title check passed!");
            driver.close().await?;
            return Ok(());
        }

        attempts += 1;
        if attempts < max_attempts {
            sleep(Duration::from_secs(2)).await;
        }
    }

    // If we get here, the test failed
    driver.close().await?;
    anyhow::bail!("Expected title to contain 'example' but got: {}", title)
}

#[tokio::test]
async fn test_cdp_script_execution_headless() -> anyhow::Result<()> {
    let driver = create_headless_driver().await?;

    // Create a simple CDP script
    let script = CdpScript {
        name: "headless-test".to_string(),
        description: "Test CDP script execution in headless mode".to_string(),
        created: None,
        author: Some("Test".to_string()),
        tags: vec!["test".to_string(), "headless".to_string()],
        cdp_commands: vec![
            CdpCommand {
                method: "Page.navigate".to_string(),
                params: serde_json::json!({
                    "url": "https://example.com"
                }),
                save_as: None,
                description: Some("Navigate to example.com".to_string()),
            },
            CdpCommand {
                method: "Runtime.evaluate".to_string(),
                params: serde_json::json!({
                    "expression": "document.title",
                    "returnByValue": true
                }),
                save_as: Some("test-title.json".to_string()),
                description: Some("Extract page title".to_string()),
            },
        ],
    };

    // Execute the script
    let report = driver.execute_cdp_script_direct(&script).await?;

    println!("📊 Execution Report:");
    println!("   Script: {}", report.script_name);
    println!("   Total commands: {}", report.total_commands);
    println!("   Successful: {}", report.successful);
    println!("   Failed: {}", report.failed);
    println!("   Success rate: {:.1}%", report.success_rate());
    println!("   Duration: {:?}", report.total_duration);

    // Verify execution
    assert!(report.is_success(), "Script execution should succeed");
    assert_eq!(report.total_commands, 2, "Should have 2 commands");
    assert_eq!(report.successful, 2, "Both commands should succeed");

    // Cleanup
    driver.close().await?;
    if std::path::Path::new("test-title.json").exists() {
        tokio::fs::remove_file("test-title.json").await.ok();
    }

    Ok(())
}

#[tokio::test]
async fn test_screenshot_capture_headless() -> anyhow::Result<()> {
    let driver = create_headless_driver().await?;

    // Create screenshot script
    let script = CdpScript {
        name: "screenshot-test".to_string(),
        description: "Capture screenshot in headless mode".to_string(),
        created: None,
        author: Some("Test".to_string()),
        tags: vec!["screenshot".to_string()],
        cdp_commands: vec![
            CdpCommand {
                method: "Page.navigate".to_string(),
                params: serde_json::json!({
                    "url": "https://example.com"
                }),
                save_as: None,
                description: Some("Navigate to example.com".to_string()),
            },
            CdpCommand {
                method: "Page.captureScreenshot".to_string(),
                params: serde_json::json!({
                    "format": "png",
                    "captureBeyondViewport": true
                }),
                save_as: Some("test-screenshot.png".to_string()),
                description: Some("Capture screenshot".to_string()),
            },
        ],
    };

    // Execute
    let report = driver.execute_cdp_script_direct(&script).await?;

    println!("📸 Screenshot Test:");
    println!("   Success: {}", report.is_success());
    println!("   Commands: {}/{}", report.successful, report.total_commands);

    // Verify
    assert!(report.is_success(), "Screenshot script should succeed");

    // Verify file exists
    let screenshot_path = std::path::Path::new("test-screenshot.png");
    assert!(screenshot_path.exists(), "Screenshot file should be created");

    // Check file size
    let metadata = std::fs::metadata(screenshot_path)?;
    println!("   Screenshot size: {} bytes", metadata.len());
    assert!(metadata.len() > 1000, "Screenshot should be at least 1KB");

    // Cleanup
    driver.close().await?;
    tokio::fs::remove_file("test-screenshot.png").await.ok();

    Ok(())
}

#[tokio::test]
async fn test_data_extraction_headless() -> anyhow::Result<()> {
    let driver = create_headless_driver().await?;

    // Create data extraction script
    let script = CdpScript {
        name: "extract-data-test".to_string(),
        description: "Extract data in headless mode".to_string(),
        created: None,
        author: Some("Test".to_string()),
        tags: vec!["extraction".to_string()],
        cdp_commands: vec![
            CdpCommand {
                method: "Page.navigate".to_string(),
                params: serde_json::json!({
                    "url": "https://example.com"
                }),
                save_as: None,
                description: Some("Navigate".to_string()),
            },
            CdpCommand {
                method: "Runtime.evaluate".to_string(),
                params: serde_json::json!({
                    "expression": "JSON.stringify({title: document.title, heading: document.querySelector('h1').textContent})",
                    "returnByValue": true
                }),
                save_as: Some("test-extracted-data.json".to_string()),
                description: Some("Extract title and heading".to_string()),
            },
        ],
    };

    // Execute
    let report = driver.execute_cdp_script_direct(&script).await?;

    println!("📦 Data Extraction Test:");
    println!("   Success: {}", report.is_success());

    // Verify
    assert!(report.is_success(), "Extraction should succeed");

    // Verify extracted data file
    let data_path = std::path::Path::new("test-extracted-data.json");
    assert!(data_path.exists(), "Extracted data file should exist");

    let content = tokio::fs::read_to_string(data_path).await?;
    println!("   Extracted: {}", content);
    assert!(content.to_lowercase().contains("example"), "Data should contain 'example'");

    // Cleanup
    driver.close().await?;
    tokio::fs::remove_file("test-extracted-data.json").await.ok();

    Ok(())
}

#[tokio::test]
async fn test_multiple_commands_headless() -> anyhow::Result<()> {
    let driver = create_headless_driver().await?;

    // Create script with multiple diverse commands
    let script = CdpScript {
        name: "multi-command-test".to_string(),
        description: "Test multiple CDP commands".to_string(),
        created: None,
        author: Some("Test".to_string()),
        tags: vec!["multi".to_string()],
        cdp_commands: vec![
            CdpCommand {
                method: "Page.navigate".to_string(),
                params: serde_json::json!({"url": "https://example.com"}),
                save_as: None,
                description: Some("Navigate".to_string()),
            },
            CdpCommand {
                method: "Runtime.evaluate".to_string(),
                params: serde_json::json!({
                    "expression": "document.title",
                    "returnByValue": true
                }),
                save_as: None,
                description: Some("Get title".to_string()),
            },
            CdpCommand {
                method: "Page.captureScreenshot".to_string(),
                params: serde_json::json!({
                    "format": "png",
                    "captureBeyondViewport": true
                }),
                save_as: Some("test-multi-screenshot.png".to_string()),
                description: Some("Screenshot".to_string()),
            },
        ],
    };

    // Execute
    let report = driver.execute_cdp_script_direct(&script).await?;

    println!("🔄 Multi-Command Test:");
    println!("   Commands: {}/{}", report.successful, report.total_commands);
    println!("   Duration: {:?}", report.total_duration);

    // Verify all commands succeeded
    assert_eq!(report.total_commands, 3, "Should have 3 commands");
    assert_eq!(report.successful, 3, "All commands should succeed");
    assert_eq!(report.failed, 0, "No commands should fail");

    // Cleanup
    driver.close().await?;
    tokio::fs::remove_file("test-multi-screenshot.png").await.ok();

    Ok(())
}
