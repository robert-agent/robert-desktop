// E2E tests - These test Chrome automation with downloaded Chrome for Testing
// Uses local HTTP server for fast, reliable, network-independent testing

mod test_server;

use robert_webdriver::{CdpCommand, CdpScript, ChromeDriver, ConnectionMode};
use std::time::Duration;
use test_server::TestServer;
use tokio::time::sleep;

#[tokio::test]
async fn test_navigate_and_get_title() {
    // Start local test server on random port
    let server = TestServer::start().await;
    server.wait_ready().await.expect("Server failed to start");
    let url = server.url();

    // Launch Chrome with auto-download
    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: true,  // Required for Ubuntu 23.10+ sandbox restrictions
        headless: true,    // Always headless (no display server required)
    })
    .await
    .expect("Failed to launch Chrome");

    // Use CDP script for reliable navigation and data extraction
    let script = CdpScript {
        name: "navigate-and-title-test".to_string(),
        description: "Navigate and get title and text".to_string(),
        created: None,
        author: Some("Test".to_string()),
        tags: vec!["navigation".to_string()],
        cdp_commands: vec![
            CdpCommand {
                method: "Page.navigate".to_string(),
                params: serde_json::json!({
                    "url": url
                }),
                save_as: None,
                description: Some("Navigate to test server".to_string()),
            },
            CdpCommand {
                method: "Runtime.evaluate".to_string(),
                params: serde_json::json!({
                    "expression": "document.title",
                    "returnByValue": true
                }),
                save_as: Some("test-nav-title.json".to_string()),
                description: Some("Get page title".to_string()),
            },
            CdpCommand {
                method: "Runtime.evaluate".to_string(),
                params: serde_json::json!({
                    "expression": "document.body.textContent",
                    "returnByValue": true
                }),
                save_as: Some("test-nav-text.json".to_string()),
                description: Some("Get page text".to_string()),
            },
        ],
    };

    // Execute the script
    println!("Navigating to: {}", url);
    let report = driver.execute_cdp_script_direct(&script).await.expect("Script execution failed");

    println!("📊 Navigation and Title Test:");
    println!("   Commands: {}/{}", report.successful, report.total_commands);
    println!("   Success rate: {:.1}%", report.success_rate());

    assert!(report.is_success(), "Script execution should succeed");

    // Read the extracted title
    let title_data = tokio::fs::read_to_string("test-nav-title.json")
        .await
        .expect("Failed to read title file");
    println!("✅ Page title: {}", title_data);
    assert!(title_data.contains("Example Domain"), "Expected title to contain 'Example Domain'");

    // Read the extracted text
    let text_data = tokio::fs::read_to_string("test-nav-text.json")
        .await
        .expect("Failed to read text file");
    println!("✅ Page text extracted");
    assert!(text_data.contains("Example Domain"), "Expected text to contain 'Example Domain'");

    println!("✅ Navigation and title check passed!");

    // Cleanup
    driver.close().await.expect("Failed to close browser");
    tokio::fs::remove_file("test-nav-title.json").await.ok();
    tokio::fs::remove_file("test-nav-text.json").await.ok();
}

#[tokio::test]
async fn test_get_element_text() {
    // Start local test server on random port
    let server = TestServer::start().await;
    server.wait_ready().await.expect("Server failed to start");
    let url = server.url();

    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: true,  // Required for Ubuntu 23.10+ sandbox restrictions
        headless: true,    // Always headless (no display server required)
    })
    .await
    .expect("Failed to launch Chrome");

    // Use CDP script for reliable navigation and element extraction
    let script = CdpScript {
        name: "element-text-test".to_string(),
        description: "Navigate and get element text".to_string(),
        created: None,
        author: Some("Test".to_string()),
        tags: vec!["element".to_string()],
        cdp_commands: vec![
            CdpCommand {
                method: "Page.navigate".to_string(),
                params: serde_json::json!({
                    "url": url
                }),
                save_as: None,
                description: Some("Navigate to test server".to_string()),
            },
            CdpCommand {
                method: "Runtime.evaluate".to_string(),
                params: serde_json::json!({
                    "expression": "document.querySelector('h1').textContent",
                    "returnByValue": true
                }),
                save_as: Some("test-element-text.json".to_string()),
                description: Some("Get h1 text".to_string()),
            },
        ],
    };

    // Execute the script
    println!("Navigating to: {}", url);
    let report = driver.execute_cdp_script_direct(&script).await.expect("Script execution failed");

    println!("📊 Element Text Test:");
    println!("   Commands: {}/{}", report.successful, report.total_commands);
    println!("   Success rate: {:.1}%", report.success_rate());

    assert!(report.is_success(), "Script execution should succeed");

    // Read the extracted element text
    let element_data = tokio::fs::read_to_string("test-element-text.json")
        .await
        .expect("Failed to read element text file");
    println!("✅ Extracted h1 text: {}", element_data);

    assert!(element_data.contains("Example Domain"), "Expected h1 to contain 'Example Domain'");

    println!("✅ Element text check passed!");

    // Cleanup
    driver.close().await.expect("Failed to close browser");
    tokio::fs::remove_file("test-element-text.json").await.ok();
}

#[tokio::test]
async fn test_get_page_source() {
    // Start local test server on random port
    let server = TestServer::start().await;
    server.wait_ready().await.expect("Server failed to start");
    let url = server.url();

    let is_ci = std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("JENKINS_HOME").is_ok()
        || std::env::var("CIRCLECI").is_ok();

    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: true,  // Required for Ubuntu 23.10+ sandbox restrictions
        headless: true,    // Always headless (no display server required)
    })
    .await
    .expect("Failed to launch Chrome");

    driver
        .navigate(&url)
        .await
        .expect("Failed to navigate");

    // Give navigation time to complete before checking
    sleep(Duration::from_secs(3)).await;

    // Check URL to verify navigation actually happened
    let current_url = driver.current_url().await.expect("Failed to get URL");
    println!("Current URL: {}", current_url);

    // Wait for page to load with retry logic
    let mut attempts = 0;
    let max_attempts = 10;
    let mut source = String::new();

    println!("⏳ Waiting for page to load...");
    while attempts < max_attempts {
        source = driver.get_page_source().await.expect("Failed to get page source");

        if source.to_lowercase().contains("example domain") {
            println!("✅ Page loaded successfully!");
            break;
        }

        println!("  Attempt {}: Page not loaded yet (length: {} bytes, URL: {})", attempts + 1, source.len(), driver.current_url().await.unwrap_or_else(|_| "unknown".to_string()));
        attempts += 1;
        if attempts < max_attempts {
            sleep(Duration::from_secs(1)).await;
        }
    }

    println!("Page source length: {} bytes", source.len());
    println!("Page source preview: {}", &source[..500.min(source.len())]);
    assert!(source.contains("<html") || source.contains("<HTML"));
    assert!(source.to_lowercase().contains("example domain"), "Page source should contain 'Example Domain'");

    // Keep window open for 5 seconds (only visible in non-CI mode)
    if !is_ci {
        println!("Keeping window open for 5 seconds...");
        sleep(Duration::from_secs(5)).await;
    }

    driver.close().await.expect("Failed to close browser");
}
