// Test CDP command execution with spider_chrome
// This test verifies we can access the Page API

use robert_webdriver::browser::ChromeDriver;

#[tokio::test]
#[ignore] // Run manually: cargo test --test cdp_execution_test -- --ignored
async fn test_cdp_page_access() -> anyhow::Result<()> {
    // This test verifies we can get the Page for CDP commands
    let driver = ChromeDriver::launch_auto().await?;

    // Get the underlying Page which has execute() method
    let page = driver.current_page().await?;

    // Page implements Command trait execution
    println!("Successfully got page: {}", std::any::type_name_of_val(&page));

    driver.close().await?;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_basic_navigation() -> anyhow::Result<()> {
    // Test using our existing high-level API
    let driver = ChromeDriver::launch_auto().await?;

    driver.navigate("https://example.com").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let url = driver.current_url().await?;
    println!("Current URL: {}", url);

    assert!(url.contains("example.com"));

    driver.close().await?;
    Ok(())
}
