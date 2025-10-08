use clap::Parser;
use robert_webdriver::ChromeDriver;

#[derive(Parser)]
#[command(name = "robert")]
#[command(version = "0.1.0")]
#[command(about = "Browser automation CLI prototype", long_about = None)]
struct Cli {
    /// URL to navigate to
    url: String,

    /// Chromedriver port
    #[arg(short, long, default_value = "9515")]
    port: u16,

    /// Output format: html or text
    #[arg(short = 'f', long, default_value = "html")]
    format: String,

    /// CSS selector for specific element (optional)
    #[arg(short = 's', long)]
    selector: Option<String>,
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

    // Connect to Chrome
    println!("🔌 Connecting to Chrome on port {}...", cli.port);
    let driver = ChromeDriver::connect(cli.port)
        .await
        .map_err(|e| format!("Failed to connect to Chrome. Is chromedriver running on port {}?\n  Error: {}", cli.port, e))?;

    // Navigate
    println!("🌐 Navigating to {}...", cli.url);
    driver.navigate(&cli.url)
        .await
        .map_err(|e| format!("Failed to navigate to {}:\n  Error: {}", cli.url, e))?;

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

    Ok(())
}
