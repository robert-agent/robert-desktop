use robert_webdriver::{ChromeDriver, ConnectionMode};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_navigate_and_get_title() {
    // Detect if we're in CI mode
    let is_ci = std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("JENKINS_HOME").is_ok()
        || std::env::var("CIRCLECI").is_ok();

    // Launch Chrome with auto-download, headless in CI, visible for local testing
    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: is_ci, // Use --no-sandbox in CI
        headless: is_ci,   // Headless in CI, visible window for local testing
    })
    .await
    .expect("Failed to launch Chrome");

    // Navigate to example.com
    driver
        .navigate("https://example.com")
        .await
        .expect("Failed to navigate");

    // Wait for page to load
    sleep(Duration::from_secs(2)).await;

    // Get page title
    let title = driver.title().await.expect("Failed to get title");
    println!("Page title: {}", title);
    assert!(title.contains("Example"));

    // Get page text
    let text = driver
        .get_page_text()
        .await
        .expect("Failed to get page text");
    println!("Page text: {}", text);
    assert!(text.contains("Example Domain"));

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
    let is_ci = std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("JENKINS_HOME").is_ok()
        || std::env::var("CIRCLECI").is_ok();

    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: is_ci,
        headless: is_ci,
    })
    .await
    .expect("Failed to launch Chrome");

    driver
        .navigate("https://example.com")
        .await
        .expect("Failed to navigate");

    // Wait for page to load
    sleep(Duration::from_secs(2)).await;

    // Get h1 element text
    let h1_text = driver
        .get_element_text("h1")
        .await
        .expect("Failed to get h1 text");
    println!("H1 text: {}", h1_text);
    assert!(h1_text.contains("Example Domain"));

    // Keep window open for 5 seconds (only visible in non-CI mode)
    if !is_ci {
        println!("Keeping window open for 5 seconds...");
        sleep(Duration::from_secs(5)).await;
    }

    driver.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_get_page_source() {
    let is_ci = std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("JENKINS_HOME").is_ok()
        || std::env::var("CIRCLECI").is_ok();

    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: is_ci,
        headless: is_ci,
    })
    .await
    .expect("Failed to launch Chrome");

    driver
        .navigate("https://example.com")
        .await
        .expect("Failed to navigate");

    // Wait for page to load
    sleep(Duration::from_secs(2)).await;

    // Get full page source
    let source = driver
        .get_page_source()
        .await
        .expect("Failed to get page source");
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
