//! Local HTTP server for tests
//!
//! This module provides a simple HTTP server that serves static HTML pages
//! for testing Chrome automation without relying on external websites.
//!
//! Each server instance runs on a random available port for perfect test isolation.

use std::net::SocketAddr;
use tokio::sync::oneshot;
use warp::Filter;

/// Test server that serves simple HTML pages
pub struct TestServer {
    addr: SocketAddr,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl TestServer {
    /// Start a new test server on a random available port
    pub async fn start() -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        // Routes
        let index = warp::path::end().map(|| {
            warp::reply::html(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <title>Example Domain</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <div>
        <h1>Example Domain</h1>
        <p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p>
        <p><a href="https://iana.org/domains/example">Learn more</a></p>
    </div>
</body>
</html>"#,
            )
        });

        let routes = index;

        // Bind to random port
        let (addr, server) = warp::serve(routes)
            .bind_with_graceful_shutdown(([127, 0, 0, 1], 0), async {
                shutdown_rx.await.ok();
            });

        // Spawn server in background
        tokio::spawn(server);

        Self {
            addr,
            shutdown_tx: Some(shutdown_tx),
        }
    }

    /// Get the base URL for this server (e.g., "http://127.0.0.1:12345")
    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// Wait for the server to be ready by making a test request
    pub async fn wait_ready(&self) -> anyhow::Result<()> {
        let url = self.url();
        let max_attempts = 10;

        for attempt in 1..=max_attempts {
            match reqwest::get(&url).await {
                Ok(response) if response.status().is_success() => {
                    println!("✅ Test server ready on: {}", url);
                    return Ok(());
                }
                Ok(response) => {
                    println!("⚠️ Attempt {}: Server returned status {}", attempt, response.status());
                }
                Err(e) => {
                    println!("⚠️ Attempt {}: Server not ready - {}", attempt, e);
                }
            }

            if attempt < max_attempts {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        anyhow::bail!("Server did not become ready after {} attempts", max_attempts)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        // Signal server to shutdown
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_starts() {
        let server = TestServer::start().await;
        assert!(server.addr.port() > 0);
        println!("Test server running on: {}", server.url());
    }

    #[tokio::test]
    async fn test_server_serves_html() {
        let server = TestServer::start().await;
        let url = server.url();

        // Make HTTP request to verify it works
        let response = reqwest::get(&url).await.unwrap();
        assert!(response.status().is_success());

        let body = response.text().await.unwrap();
        assert!(body.contains("Example Domain"));
        assert!(body.contains("<h1>"));
    }

    #[tokio::test]
    async fn test_multiple_servers_different_ports() {
        let server1 = TestServer::start().await;
        let server2 = TestServer::start().await;

        // Each server should have a different port
        assert_ne!(server1.addr.port(), server2.addr.port());
    }
}
