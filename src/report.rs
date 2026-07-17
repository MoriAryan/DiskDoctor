use colored::Colorize;

use crate::duplicate::DuplicateGroup;
use crate::scanner::ScanResult;
use crate::utils::format_size;

/// Prints the summary report after scanning completes.
pub fn print_summary(
    scan_result: &ScanResult,
    duplicates: &[DuplicateGroup],
    large_threshold: u64,
) {
    let large_files_count = scan_result
        .files
        .iter()
        .filter(|f| f.size >= large_threshold)
        .count();

    let installer_count = scan_result.files.iter().filter(|f| f.is_installer).count();
    let archive_count = scan_result.files.iter().filter(|f| f.is_archive).count();

    let total_recoverable: u64 = duplicates.iter().map(|g| g.recoverable_space()).sum();

    let separator = "══════════════════════════════════════════".cyan();

    println!();
    println!("{}", separator);
    println!(
        "{}",
        "          DiskDoctor Report              ".bold().cyan()
    );
    println!("{}", separator);
    println!();
    println!(
        "  {} {:>10}",
        "Files scanned:".white(),
        scan_result.file_count.to_string().green()
    );
    println!(
        "  {} {:>10}",
        "Folders scanned:".white(),
        scan_result.dir_count.to_string().green()
    );

    if scan_result.skipped_count > 0 {
        println!(
            "  {} {:>10}",
            "Skipped (errors):".white(),
            scan_result.skipped_count.to_string().yellow()
        );
    }

    println!();
    println!(
        "  {} {:>10}",
        "Duplicate Groups:".white(),
        duplicates.len().to_string().yellow()
    );
    println!(
        "  {} {:>10}",
        "Potential Space Recoverable:".white(),
        format_size(total_recoverable).red().bold()
    );
    println!();
    println!(
        "  {} {:>10}",
        format!("Large Files (>{}):", format_size(large_threshold)).white(),
        large_files_count.to_string().yellow()
    );
    println!(
        "  {} {:>10}",
        "Empty Directories:".white(),
        scan_result.empty_dirs.len().to_string().yellow()
    );
    println!(
        "  {} {:>10}",
        "Old Installers:".white(),
        installer_count.to_string().yellow()
    );
    println!(
        "  {} {:>10}",
        "Archive Files:".white(),
        archive_count.to_string().yellow()
    );
    println!();
    println!("{}", separator);
    println!(
        "  {}",
        "No files were modified or deleted.".green().italic()
    );
    println!("{}", separator);
    println!();
}
