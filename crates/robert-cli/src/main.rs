use clap::Parser;
use robert_webdriver::{ChromeDriver, ConnectionMode};

#[derive(Parser)]
#[command(name = "robert")]
#[command(version = "0.1.0")]
#[command(about = "Browser automation CLI prototype", long_about = None)]
struct Cli {
    /// URL to navigate to
    url: String,

    /// Connect to existing Chrome debug port (advanced mode)
    /// Example: --debug-port 9222
    #[arg(long)]
    debug_port: Option<u16>,

    /// Path to Chrome/Chromium executable (optional)
    /// Example: --chrome-path /usr/bin/chromium
    #[arg(long)]
    chrome_path: Option<String>,

    /// Output format: html or text
    #[arg(short = 'f', long, default_value = "html")]
    format: String,

    /// CSS selector for specific element (optional)
    #[arg(short = 's', long)]
    selector: Option<String>,

    /// Disable Chrome sandbox (required on some Linux systems with AppArmor restrictions)
    /// WARNING: Reduces security. Only use if Chrome fails to launch.
    #[arg(long)]
    no_sandbox: bool,

    /// Run Chrome in headless mode (no visible window)
    #[arg(long)]
    headless: bool,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Robert CLI v0.1.0");
    println!("================\n");

    // Detect CI environment
    let is_ci = std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("JENKINS_HOME").is_ok()
        || std::env::var("CIRCLECI").is_ok();

    // Auto-enable no-sandbox and headless in CI environments
    let no_sandbox = cli.no_sandbox || is_ci;
    let headless = cli.headless || is_ci;

    if is_ci {
        println!("🤖 CI environment detected - using headless mode with --no-sandbox\n");
    }

    // Connect to Chrome (sandboxed or debug port)
    let driver = if let Some(port) = cli.debug_port {
        println!("🔌 Connecting to Chrome debug port {}...", port);
        println!("   (Advanced Mode - using your existing Chrome browser)\n");
        ChromeDriver::connect_debug_port(port).await.map_err(|e| {
            format!(
                "Failed to connect to Chrome on port {}.\n\
                 Make sure Chrome is running with: --remote-debugging-port={}\n\
                 Error: {}",
                port, port, e
            )
        })?
    } else if let Some(path) = cli.chrome_path {
        println!("🚀 Launching Chrome in sandboxed mode...");
        println!("   (Using Chrome at: {})\n", path);
        if no_sandbox && !is_ci {
            println!("⚠️  WARNING: Running with --no-sandbox (reduced security)\n");
        }
        if headless {
            println!("👻 Running in headless mode (no visible window)\n");
        }
        ChromeDriver::launch_with_path(path, no_sandbox, headless)
            .await
            .map_err(|e| {
                format!(
                    "Failed to launch Chrome.\n\
                 Error: {}",
                    e
                )
            })?
    } else {
        println!("🚀 Launching Chrome in sandboxed mode...");
        println!("   (Using system Chrome - isolated session)\n");
        if no_sandbox && !is_ci {
            println!("⚠️  WARNING: Running with --no-sandbox (reduced security)\n");
        }
        if headless {
            println!("👻 Running in headless mode (no visible window)\n");
        }
        ChromeDriver::new(ConnectionMode::Sandboxed {
            chrome_path: None,
            no_sandbox,
            headless,
        })
        .await
        .map_err(|e| {
            format!(
                "Failed to launch Chrome.\n\
                 Error: {}",
                e
            )
        })?
    };

    // Normalize URL - add https:// if no protocol specified
    let url = if cli.url.starts_with("http://") || cli.url.starts_with("https://") {
        cli.url.clone()
    } else {
        format!("https://{}", cli.url)
    };

    // Navigate
    println!("🌐 Navigating to {}...", url);
    driver
        .navigate(&url)
        .await
        .map_err(|e| format!("Failed to navigate to {}:\n  Error: {}", url, e))?;

    // Get page info
    let title = driver.title().await?;
    println!("✅ Page loaded: {}\n", title);

    // Extract content
    let content = if let Some(selector) = cli.selector {
        println!("📍 Extracting content from: {}\n", selector);
        driver.get_element_text(&selector).await?
    } else {
        match cli.format.as_str() {
            "text" => driver.get_page_text().await?,
            _ => driver.get_page_source().await?,
        }
    };

    println!("{}", content);

    // Close browser
    driver.close().await?;

    Ok(())
}
