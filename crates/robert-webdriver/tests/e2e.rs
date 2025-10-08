use bollard::container::{Config, CreateContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::models::{HostConfig, PortBinding};
use bollard::Docker;
use futures_util::StreamExt;
use robert_webdriver::ChromeDriver;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;
use tokio::task::JoinHandle;
use warp::Filter;

/// Shared Selenium container info that persists across all tests
static SELENIUM_CONTAINER: OnceCell<Arc<(String, u16)>> = OnceCell::const_new();

/// Initialize the Selenium container (called once for all tests)
async fn init_selenium_container() -> Arc<(String, u16)> {
    SELENIUM_CONTAINER
        .get_or_init(|| async {
            let docker = Docker::connect_with_local_defaults()
                .expect("Failed to connect to Docker");

            // Pull image if not present
            let image = "selenium/standalone-chrome:latest";
            let mut stream = docker.create_image(
                Some(CreateImageOptions {
                    from_image: image,
                    ..Default::default()
                }),
                None,
                None,
            );

            while let Some(result) = stream.next().await {
                result.expect("Failed to pull image");
            }

            // Create container with port binding
            let mut port_bindings = HashMap::new();
            port_bindings.insert(
                "4444/tcp".to_string(),
                Some(vec![PortBinding {
                    host_ip: Some("127.0.0.1".to_string()),
                    host_port: Some("0".to_string()), // Random port
                }]),
            );

            let env = vec![
                "SE_NODE_MAX_SESSIONS=5",
                "SE_NODE_SESSION_TIMEOUT=300",
                "SE_VNC_NO_PASSWORD=1",
            ];

            let extra_hosts = vec!["host.docker.internal:host-gateway".to_string()];

            let host_config = HostConfig {
                port_bindings: Some(port_bindings),
                shm_size: Some(2_000_000_000),
                extra_hosts: Some(extra_hosts),
                ..Default::default()
            };

            let config = Config {
                image: Some(image),
                env: Some(env),
                host_config: Some(host_config),
                ..Default::default()
            };

            let container_name = format!("selenium-test-{}", std::process::id());
            let container = docker
                .create_container(
                    Some(CreateContainerOptions {
                        name: container_name.clone(),
                        ..Default::default()
                    }),
                    config,
                )
                .await
                .expect("Failed to create container");

            docker
                .start_container::<String>(&container.id, None)
                .await
                .expect("Failed to start container");

            // Get the assigned port
            let inspect = docker
                .inspect_container(&container.id, None)
                .await
                .expect("Failed to inspect container");

            let port = inspect
                .network_settings
                .and_then(|ns| ns.ports)
                .and_then(|ports| ports.get("4444/tcp").cloned())
                .flatten()
                .and_then(|bindings| bindings.first().cloned())
                .and_then(|binding| binding.host_port)
                .expect("Failed to get port")
                .parse::<u16>()
                .expect("Invalid port");

            // Wait for Selenium to be ready
            tokio::time::sleep(Duration::from_secs(5)).await;

            Arc::new((container.id, port))
        })
        .await
        .clone()
}

/// Helper to connect to the Selenium browser
async fn connect_browser() -> ChromeDriver {
    let container_info = init_selenium_container().await;
    let (_container_id, port) = container_info.as_ref();
    let url = format!("http://127.0.0.1:{}", port);
    ChromeDriver::connect_url(&url, true)
        .await
        .expect("Failed to connect to Selenium")
}

/// Helper to start a test HTTP server with warp
async fn start_test_server() -> (SocketAddr, JoinHandle<()>) {
    // HTML test page
    let index = warp::path::end().map(|| {
        warp::reply::html(
            r#"<!DOCTYPE html>
<html>
<head><title>Test Page</title></head>
<body>
    <h1>Test Page</h1>
    <p>This is a test page for end-to-end testing.</p>
    <div class="content">
        <button id="test-button">Click Me</button>
        <input id="test-input" type="text" placeholder="Enter text">
        <div id="result"></div>
    </div>
</body>
</html>"#,
        )
    });

    // API endpoint
    let api = warp::path("api")
        .and(warp::path("data"))
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok", "message": "API response"})));

    let routes = index.or(api);

    // Bind to 0.0.0.0 so it's accessible from Docker containers
    let (addr, server) = warp::serve(routes).bind_ephemeral(([0, 0, 0, 0], 0));

    let handle = tokio::spawn(server);

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    (addr, handle)
}

/// Get the host address accessible from the browser
/// Chrome runs in a Docker container and needs to access the host test server
fn get_host_address() -> String {
    // Use host.docker.internal (works on Docker Desktop)
    // On Linux with newer Docker versions, this is also supported
    "host.docker.internal".to_string()
}

#[tokio::test]
async fn test_headless_chrome_navigation() {
    let driver = connect_browser().await;

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

    driver.close().await.expect("Failed to close driver");
}

#[tokio::test]
async fn test_headless_chrome_with_local_server() {
    let (addr, _server_handle) = start_test_server().await;
    let host = get_host_address();
    let url = format!("http://{}:{}", host, addr.port());

    let driver = connect_browser().await;

    driver
        .navigate(&url)
        .await
        .expect("Failed to navigate to test server");

    let title = driver.title().await.expect("Failed to get title");
    assert_eq!(title, "Test Page");

    let text = driver
        .get_page_text()
        .await
        .expect("Failed to get page text");
    assert!(text.contains("Test Page"));
    assert!(text.contains("This is a test page for end-to-end testing"));

    // Test element text extraction
    let h1_text = driver
        .get_element_text("h1")
        .await
        .expect("Failed to get h1 text");
    assert_eq!(h1_text, "Test Page");

    // Test current URL (Chrome may add a trailing slash)
    let current_url = driver.current_url().await.expect("Failed to get URL");
    assert!(current_url == url || current_url == format!("{}/", url));

    // Test page source
    let source = driver
        .get_page_source()
        .await
        .expect("Failed to get page source");
    assert!(source.contains("<h1>"));
    assert!(source.contains("Test Page"));

    driver.close().await.expect("Failed to close driver");
}

#[tokio::test]
async fn test_element_not_found() {
    let (addr, _server_handle) = start_test_server().await;
    let host = get_host_address();
    let url = format!("http://{}:{}", host, addr.port());

    let driver = connect_browser().await;

    driver.navigate(&url).await.expect("Failed to navigate");

    // Try to get text from non-existent element
    let result = driver.get_element_text(".non-existent-class").await;
    assert!(result.is_err());

    driver.close().await.expect("Failed to close driver");
}

#[tokio::test]
async fn test_multiple_pages() {
    let (addr, _server_handle) = start_test_server().await;
    let host = get_host_address();
    let base_url = format!("http://{}:{}", host, addr.port());

    let driver = connect_browser().await;

    // Navigate to test page
    driver
        .navigate(&base_url)
        .await
        .expect("Failed to navigate");
    let title = driver.title().await.expect("Failed to get title");
    assert_eq!(title, "Test Page");

    // Navigate to example.com
    driver
        .navigate("https://example.com")
        .await
        .expect("Failed to navigate to example.com");
    let title = driver.title().await.expect("Failed to get title");
    assert_eq!(title, "Example Domain");

    driver.close().await.expect("Failed to close driver");
}
