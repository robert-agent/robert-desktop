//! CLI tool for generating CDP automation scripts using Claude

use clap::Parser;
use robert_webdriver::CdpScriptGenerator;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "robert-generate")]
#[command(version = "0.1.0")]
#[command(about = "Generate browser automation scripts using Claude AI", long_about = None)]
struct Cli {
    /// Natural language description of what you want to automate
    ///
    /// Examples:
    ///   "Take a screenshot of example.com"
    ///   "Navigate to github.com and extract all repository names"
    ///   "Fill out a form and submit it"
    description: String,

    /// Output file path for the generated script (defaults to generated-script.json)
    #[arg(short = 'o', long, default_value = "generated-script.json")]
    output: PathBuf,

    /// Claude model to use (e.g., "sonnet", "opus")
    #[arg(long)]
    model: Option<String>,

    /// Number of retries if generation fails
    #[arg(long, default_value = "3")]
    retries: u32,

    /// Path to Claude CLI executable (defaults to "claude" in PATH)
    #[arg(long)]
    claude_path: Option<String>,

    /// Execute the script immediately after generation
    #[arg(long)]
    execute: bool,

    /// Chrome path (if --execute is used)
    #[arg(long)]
    chrome_path: Option<String>,

    /// No sandbox mode (if --execute is used)
    #[arg(long)]
    no_sandbox: bool,

    /// Headless mode (if --execute is used)
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

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("🤖 Generating CDP script with Claude AI...");
    println!("📝 Description: {}", cli.description);

    // Create generator
    let mut generator = CdpScriptGenerator::new();

    if let Some(path) = cli.claude_path {
        generator = generator.with_claude_path(path);
    }

    if let Some(model) = cli.model {
        generator = generator.with_model(model);
    }

    // Generate script with retries
    let script = generator
        .generate_with_retry(&cli.description, cli.retries)
        .await?;

    println!("✅ Script generated successfully!");
    println!("   Name: {}", script.name);
    println!("   Description: {}", script.description);
    println!("   Commands: {}", script.cdp_commands.len());
    println!();

    // Show commands
    println!("📋 Commands:");
    for (i, cmd) in script.cdp_commands.iter().enumerate() {
        println!("  {}. {}", i + 1, cmd.method);
        if let Some(desc) = &cmd.description {
            println!("     {}", desc);
        }
    }
    println!();

    // Save to file
    script.to_file(&cli.output).await?;
    println!("💾 Saved to: {}", cli.output.display());

    // Execute if requested
    if cli.execute {
        println!();
        println!("🚀 Executing script...");

        use robert_webdriver::{ChromeDriver, ConnectionMode};

        let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
            chrome_path: cli.chrome_path,
            no_sandbox: cli.no_sandbox,
            headless: cli.headless,
        })
        .await?;

        let report = driver.execute_cdp_script_direct(&script).await?;

        println!();
        println!("📊 Execution Report:");
        println!("   Total commands: {}", report.total_commands);
        println!("   Successful: {}", report.successful);
        println!("   Failed: {}", report.failed);
        println!("   Success rate: {:.1}%", report.success_rate());
        println!("   Duration: {:?}", report.total_duration);

        if !report.is_success() {
            println!();
            println!("❌ Some commands failed:");
            for result in &report.results {
                if let Some(error) = &result.error {
                    println!("   Step {}: {}", result.step, error);
                }
            }
        }

        driver.close().await?;
    }

    Ok(())
}
