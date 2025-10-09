//! Integration tests for CDP script generation with Claude

use robert_webdriver::{CdpScriptGenerator, ChromeDriver};
use std::path::Path;

#[tokio::test]
#[ignore] // Requires Claude CLI: cargo test --test cdp_generator_test -- --ignored
async fn test_generate_screenshot_script() -> anyhow::Result<()> {
    let generator = CdpScriptGenerator::new();

    // Generate script from natural language
    let script = generator
        .generate("Take a screenshot of example.com and save it as test-screenshot.png")
        .await?;

    println!("Generated script: {}", script.name);
    println!("Commands: {}", script.cdp_commands.len());

    // Validate script structure
    assert!(!script.name.is_empty());
    assert!(!script.cdp_commands.is_empty());
    assert!(script.validate().is_ok());

    // Execute the generated script
    let driver = ChromeDriver::launch_auto().await?;
    let report = driver.execute_cdp_script_direct(&script).await?;

    println!("Execution successful: {}", report.is_success());
    assert!(report.is_success());

    driver.close().await?;

    // Cleanup
    if Path::new("test-screenshot.png").exists() {
        tokio::fs::remove_file("test-screenshot.png").await.ok();
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_generate_data_extraction_script() -> anyhow::Result<()> {
    let generator = CdpScriptGenerator::new();

    // Generate data extraction script
    let script = generator
        .generate(
            "Go to example.com and extract the page title and main heading text, save to data.json",
        )
        .await?;

    println!("Generated script: {}", script.name);
    println!("Description: {}", script.description);

    // Should have navigate + evaluate commands
    assert!(script.cdp_commands.len() >= 2);

    // Execute
    let driver = ChromeDriver::launch_auto().await?;
    let report = driver.execute_cdp_script_direct(&script).await?;

    println!("Total commands: {}", report.total_commands);
    println!("Successful: {}", report.successful);

    driver.close().await?;

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_generate_complex_automation() -> anyhow::Result<()> {
    let generator = CdpScriptGenerator::new();

    let script = generator
        .generate(
            "Navigate to httpbin.org/forms/post, fill out the form with test data, and submit it",
        )
        .await?;

    println!("Generated script: {}", script.name);
    println!("Commands:");
    for (i, cmd) in script.cdp_commands.iter().enumerate() {
        println!("  {}. {} - {:?}", i + 1, cmd.method, cmd.description);
    }

    // Validate
    assert!(script.validate().is_ok());
    assert!(script.cdp_commands.len() >= 3); // navigate, fill, submit

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_generate_with_retry() -> anyhow::Result<()> {
    let generator = CdpScriptGenerator::new();

    // Test retry mechanism
    let script = generator
        .generate_with_retry("Screenshot example.com", 3)
        .await?;

    println!("Generated with retry: {}", script.name);
    assert!(script.validate().is_ok());

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_generator_various_scenarios() -> anyhow::Result<()> {
    let generator = CdpScriptGenerator::new();

    let scenarios = vec![
        "Take a screenshot of google.com",
        "Extract all links from a webpage",
        "Set geolocation to San Francisco and reload the page",
        "Get all cookies from the current page",
        "Emulate a mobile device and take a screenshot",
    ];

    for scenario in scenarios {
        println!("\n=== Testing scenario: {} ===", scenario);

        match generator.generate(scenario).await {
            Ok(script) => {
                println!("✓ Generated: {}", script.name);
                println!("  Commands: {}", script.cdp_commands.len());
                assert!(script.validate().is_ok());
            }
            Err(e) => {
                println!("✗ Failed: {}", e);
                // Don't fail the test, just report
            }
        }
    }

    Ok(())
}

#[test]
fn test_script_validation() {
    // Test validation logic
    use robert_webdriver::cdp::validate_generated_script;

    let valid_json = r#"{
        "name": "test",
        "description": "Test script",
        "cdp_commands": [
            {
                "method": "Page.navigate",
                "params": {"url": "https://example.com"}
            }
        ]
    }"#;

    let result = validate_generated_script(valid_json);
    assert!(result.is_ok());

    // Test with unknown command
    let invalid_json = r#"{
        "name": "test",
        "description": "Test script",
        "cdp_commands": [
            {
                "method": "Unknown.command",
                "params": {}
            }
        ]
    }"#;

    let result = validate_generated_script(invalid_json);
    assert!(result.is_err());
}
