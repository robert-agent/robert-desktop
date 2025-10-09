//! Integration tests for CDP script execution

use robert_webdriver::{CdpScript, ChromeDriver};
use std::path::Path;

#[tokio::test]
#[ignore] // Run manually: cargo test --test cdp_script_execution_test -- --ignored
async fn test_execute_basic_navigation_script() -> anyhow::Result<()> {
    // Launch browser
    let driver = ChromeDriver::launch_auto().await?;

    // Execute CDP script
    let script_path = Path::new("examples/cdp-scripts/basic-navigation.json");
    let report = driver.execute_cdp_script(script_path).await?;

    println!("Execution Report:");
    println!("  Script: {}", report.script_name);
    println!("  Total commands: {}", report.total_commands);
    println!("  Successful: {}", report.successful);
    println!("  Failed: {}", report.failed);
    println!("  Success rate: {:.1}%", report.success_rate());

    // Check results
    assert!(report.is_success(), "Script execution should succeed");
    assert_eq!(report.successful, 2, "Should execute 2 commands");

    // Verify screenshot was saved
    assert!(
        Path::new("example-homepage.png").exists(),
        "Screenshot should be saved"
    );

    // Cleanup
    driver.close().await?;
    tokio::fs::remove_file("example-homepage.png").await.ok();

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_execute_extract_title_script() -> anyhow::Result<()> {
    // Launch browser
    let driver = ChromeDriver::launch_auto().await?;

    // Execute CDP script
    let script_path = Path::new("examples/cdp-scripts/extract-title.json");
    let report = driver.execute_cdp_script(script_path).await?;

    println!("Execution Report:");
    println!("  Script: {}", report.script_name);
    println!("  Total commands: {}", report.total_commands);
    println!("  Successful: {}", report.successful);
    println!("  Duration: {:?}", report.total_duration);

    // Check results
    assert!(report.is_success(), "Script execution should succeed");
    assert_eq!(report.successful, 3, "Should execute 3 commands");

    // Verify extracted data was saved
    assert!(
        Path::new("page-title.json").exists(),
        "Title should be saved"
    );
    assert!(
        Path::new("main-heading.json").exists(),
        "Heading should be saved"
    );

    // Read and verify content
    let title_content = tokio::fs::read_to_string("page-title.json").await?;
    println!("Extracted title: {}", title_content);
    assert!(title_content.contains("Example"), "Title should contain 'Example'");

    // Cleanup
    driver.close().await?;
    tokio::fs::remove_file("page-title.json").await.ok();
    tokio::fs::remove_file("main-heading.json").await.ok();

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_execute_cdp_script_direct() -> anyhow::Result<()> {
    // Launch browser
    let driver = ChromeDriver::launch_auto().await?;

    // Create script programmatically
    let script = CdpScript {
        name: "programmatic-test".to_string(),
        description: "Test script created in code".to_string(),
        created: None,
        author: Some("Test".to_string()),
        tags: vec!["test".to_string()],
        cdp_commands: vec![
            robert_webdriver::CdpCommand {
                method: "Page.navigate".to_string(),
                params: serde_json::json!({
                    "url": "https://example.com"
                }),
                save_as: None,
                description: Some("Navigate to example".to_string()),
            },
            robert_webdriver::CdpCommand {
                method: "Runtime.evaluate".to_string(),
                params: serde_json::json!({
                    "expression": "document.title",
                    "returnByValue": true
                }),
                save_as: None,
                description: Some("Get title".to_string()),
            },
        ],
    };

    // Execute the script
    let report = driver.execute_cdp_script_direct(&script).await?;

    println!("Programmatic script execution:");
    println!("  Total commands: {}", report.total_commands);
    println!("  Successful: {}", report.successful);

    assert!(report.is_success(), "Script should succeed");

    driver.close().await?;

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_invalid_cdp_command() -> anyhow::Result<()> {
    let driver = ChromeDriver::launch_auto().await?;

    let script = CdpScript {
        name: "invalid-command-test".to_string(),
        description: "Test with invalid command".to_string(),
        created: None,
        author: None,
        tags: vec![],
        cdp_commands: vec![robert_webdriver::CdpCommand {
            method: "Invalid.command".to_string(),
            params: serde_json::json!({}),
            save_as: None,
            description: None,
        }],
    };

    let report = driver.execute_cdp_script_direct(&script).await?;

    // Should fail with unsupported command error
    assert!(!report.is_success(), "Invalid command should fail");
    assert_eq!(report.failed, 1, "Should have 1 failed command");
    assert!(
        report.results[0]
            .error
            .as_ref()
            .unwrap()
            .contains("Unsupported"),
        "Error should mention unsupported command"
    );

    driver.close().await?;

    Ok(())
}
