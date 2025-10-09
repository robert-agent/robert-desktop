// E2E tests - These test Chrome automation with downloaded Chrome for Testing
// Uses local HTTP server for fast, reliable, network-independent testing

mod test_server;

use robert_webdriver::{ChromeDriver, ConnectionMode};
use std::time::Duration;
use test_server::TestServer;
use tokio::time::sleep;

#[tokio::test]
async fn test_navigate_and_get_title() {
    // Start local test server on random port
    let server = TestServer::start().await;
    server.wait_ready().await.expect("Server failed to start");
    let url = server.url();

    // Detect if we're in CI mode
    let is_ci = std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("JENKINS_HOME").is_ok()
        || std::env::var("CIRCLECI").is_ok();

    // Launch Chrome with auto-download
    // Note: no_sandbox=true is required for Ubuntu 23.10+ and CI environments
    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: true,  // Required for Ubuntu 23.10+ sandbox restrictions
        headless: true,    // Always headless (no display server required)
    })
    .await
    .expect("Failed to launch Chrome");

    // Navigate to local test server
    driver
        .navigate(&url)
        .await
        .expect("Failed to navigate");

    // Give navigation time to complete before checking
    sleep(Duration::from_secs(1)).await;

    // Check URL to debug
    let current_url = driver.current_url().await.expect("Failed to get URL");
    println!("Current URL after navigation: {}", current_url);

    // Wait for page to load with retry logic
    let mut attempts = 0;
    let max_attempts = 10;
    let mut title = String::new();

    println!("⏳ Waiting for page to load...");
    while attempts < max_attempts {
        title = driver.title().await.expect("Failed to get title");
        println!("  Attempt {}: title = '{}'", attempts + 1, title);

        if title.to_lowercase().contains("example") && !title.contains("New Tab") {
            println!("✅ Page loaded successfully!");
            break;
        }

        attempts += 1;
        if attempts < max_attempts {
            sleep(Duration::from_secs(1)).await;
        }
    }

    // Verify we got the correct title
    assert!(title.contains("Example"), "Expected title to contain 'Example' but got: {}", title);
    assert!(!title.contains("New Tab"), "Still on 'New Tab' - page didn't load");

    // Get page text with retry
    let mut text = String::new();
    for _ in 0..5 {
        text = driver.get_page_text().await.expect("Failed to get page text");
        if text.contains("Example Domain") {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }

    println!("Page text: {}", text);
    assert!(text.contains("Example Domain"), "Expected text to contain 'Example Domain'");

    // Keep window open for 5 seconds (only visible in non-CI mode)
    if !is_ci {
        println!("Keeping window open for 5 seconds...");
        sleep(Duration::from_secs(5)).await;
    }

    // Close browser
    driver.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_get_element_text() {
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

    // First, wait for page to load by checking title
    let mut attempts = 0;
    let max_attempts = 10;
    let mut page_loaded = false;

    println!("⏳ Waiting for page to load...");
    while attempts < max_attempts {
        let title = driver.title().await.expect("Failed to get title");
        println!("  Attempt {}: title = '{}'", attempts + 1, title);

        if title.to_lowercase().contains("example") && !title.contains("New Tab") {
            println!("✅ Page loaded successfully!");
            page_loaded = true;
            break;
        }

        attempts += 1;
        if attempts < max_attempts {
            sleep(Duration::from_secs(1)).await;
        }
    }

    assert!(page_loaded, "Page failed to load after {} attempts", max_attempts);

    // Now wait for h1 element
    let mut h1_attempts = 0;
    let max_h1_attempts = 10;
    let mut h1_text = String::new();

    println!("⏳ Waiting for h1 element...");
    while h1_attempts < max_h1_attempts {

        match driver.get_element_text("h1").await {
            Ok(text) => {
                h1_text = text;
                println!("  Attempt {}: h1 = '{}'", h1_attempts + 1, h1_text);

                if h1_text.contains("Example Domain") {
                    println!("✅ Element loaded successfully!");
                    break;
                }
            }
            Err(e) => {
                println!("  Attempt {}: Error getting h1 - {}", h1_attempts + 1, e);
            }
        }

        h1_attempts += 1;
        if h1_attempts < max_h1_attempts {
            sleep(Duration::from_secs(1)).await;
        }
    }

    println!("H1 text: {}", h1_text);
    assert!(h1_text.contains("Example Domain"), "Expected h1 to contain 'Example Domain' but got: {}", h1_text);

    // Keep window open for 5 seconds (only visible in non-CI mode)
    if !is_ci {
        println!("Keeping window open for 5 seconds...");
        sleep(Duration::from_secs(5)).await;
    }

    driver.close().await.expect("Failed to close browser");
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
