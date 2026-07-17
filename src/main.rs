mod duplicate;
mod menu;
mod report;
mod scanner;
mod utils;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

/// DiskDoctor — A CLI utility to analyze disk usage and find wasted space.
///
/// Scans a directory and identifies duplicates, large files, empty folders,
/// old installers, and archive files. Never deletes anything automatically.
#[derive(Parser)]
#[command(name = "diskdoctor")]
#[command(version = "0.1.0")]
#[command(about = "Analyze disk usage and find wasted space")]
struct Cli {
    /// The directory to scan. Defaults to the current directory.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Minimum file size (in MB) to consider as a "large file".
    #[arg(long, default_value = "500")]
    size: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Convert the size threshold from MB to bytes.
    let large_threshold = cli.size * 1024 * 1024;

    // Resolve the path so we display an absolute path in the output.
    let scan_path = cli
        .path
        .canonicalize()
        .with_context(|| format!("Cannot access directory: {}", cli.path.display()))?;

    // Print a header.
    println!();
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!(
        "{}{}{}",
        "║".cyan(),
        "          🩺 DiskDoctor v0.1.0           ".bold().white(),
        "║".cyan()
    );
    println!(
        "{}{}{}",
        "║".cyan(),
        "    Analyze first. Act later.             ".italic().white(),
        "║".cyan()
    );
    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!();
    println!(
        "  Scanning: {}",
        scan_path.display().to_string().green()
    );
    println!(
        "  Large file threshold: {}",
        utils::format_size(large_threshold).yellow()
    );
    println!();

    // Step 1: Scan the directory.
    let scan_result = scanner::scan_directory(&scan_path)
        .with_context(|| format!("Failed to scan directory: {}", scan_path.display()))?;

    // Step 2: Find duplicates.
    let duplicates = duplicate::find_duplicates(&scan_result.files)
        .context("Failed during duplicate detection")?;

    // Step 3: Print the summary report.
    report::print_summary(&scan_result, &duplicates, large_threshold);

    // Step 4: Launch the interactive menu.
    menu::run_menu(&scan_result, &duplicates, large_threshold)?;

    Ok(())
}
