use robert_webdriver::ChromeDriver;
use std::net::SocketAddr;
use tokio::task::JoinHandle;
use warp::Filter;

/// Helper to start a test HTTP server with warp
async fn start_test_server() -> (SocketAddr, JoinHandle<()>) {
    // HTML test page
    let index = warp::path::end().map(|| {
        warp::reply::html(
            r#"<!DOCTYPE html>
<html>
<head><title>Example Domain</title></head>
<body>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents.</p>
    <div class="info">Additional information can be found here.</div>
</body>
</html>"#,
        )
    });

    // API endpoint
    let api = warp::path("api")
        .and(warp::path("data"))
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    let routes = index.or(api);

    // Bind to a random port
    let (addr, server) = warp::serve(routes).bind_ephemeral(([127, 0, 0, 1], 0));

    let handle = tokio::spawn(server);

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (addr, handle)
}

#[tokio::test]
#[ignore] // Run manually with chromedriver running
async fn test_basic_navigation() {
    let driver = ChromeDriver::connect(9515)
        .await
        .expect("Failed to connect to chromedriver");

    driver
        .navigate("https://example.com")
        .await
        .expect("Failed to navigate");

    let title = driver.title().await.expect("Failed to get title");
    assert_eq!(title, "Example Domain");

    let text = driver
        .get_page_text()
        .await
        .expect("Failed to get page text");
    assert!(text.contains("Example Domain"));
}

#[tokio::test]
#[ignore] // Run manually with chromedriver running
async fn test_with_local_server() {
    let (addr, _server_handle) = start_test_server().await;
    let url = format!("http://{}", addr);

    let driver = ChromeDriver::connect(9515)
        .await
        .expect("Failed to connect to chromedriver");

    driver
        .navigate(&url)
        .await
        .expect("Failed to navigate to test server");

    let title = driver.title().await.expect("Failed to get title");
    assert_eq!(title, "Example Domain");

    let text = driver
        .get_page_text()
        .await
        .expect("Failed to get page text");
    assert!(text.contains("Example Domain"));
    assert!(text.contains("This domain is for use in illustrative examples"));

    // Test element text extraction
    let h1_text = driver
        .get_element_text("h1")
        .await
        .expect("Failed to get h1 text");
    assert_eq!(h1_text, "Example Domain");

    // Test current URL
    let current_url = driver.current_url().await.expect("Failed to get URL");
    assert_eq!(current_url, url);

    // Test page source
    let source = driver
        .get_page_source()
        .await
        .expect("Failed to get page source");
    assert!(source.contains("<h1>"));
    assert!(source.contains("Example Domain"));
}

#[tokio::test]
#[ignore] // Run manually with chromedriver running
async fn test_element_not_found() {
    let (addr, _server_handle) = start_test_server().await;
    let url = format!("http://{}", addr);

    let driver = ChromeDriver::connect(9515)
        .await
        .expect("Failed to connect");

    driver.navigate(&url).await.expect("Failed to navigate");

    // Try to get text from non-existent element
    let result = driver.get_element_text(".non-existent-class").await;
    assert!(result.is_err());
}
