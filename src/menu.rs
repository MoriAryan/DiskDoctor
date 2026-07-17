use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use dialoguer::Select;

use crate::duplicate::DuplicateGroup;
use crate::scanner::ScanResult;
use crate::utils::format_size;

/// Runs the interactive post-scan menu.
/// The user can browse findings and mark files for review.
pub fn run_menu(
    scan_result: &ScanResult,
    duplicates: &[DuplicateGroup],
    large_threshold: u64,
) -> Result<()> {
    // This list holds all files the user marks for deletion/review.
    let mut review_list: Vec<PathBuf> = Vec::new();

    loop {
        println!();
        let review_status = if review_list.is_empty() {
            String::new()
        } else {
            format!(" ({} files marked for review)", review_list.len())
        };

        println!(
            "{}{}",
            "── Select an option ──".cyan().bold(),
            review_status.yellow()
        );

        let options = vec![
            format!("View Duplicate Files ({})", duplicates.len()),
            format!(
                "View Large Files ({})",
                scan_result
                    .files
                    .iter()
                    .filter(|f| f.size >= large_threshold)
                    .count()
            ),
            format!("View Empty Directories ({})", scan_result.empty_dirs.len()),
            format!(
                "View Old Installers ({})",
                scan_result.files.iter().filter(|f| f.is_installer).count()
            ),
            format!(
                "View Archive Files ({})",
                scan_result.files.iter().filter(|f| f.is_archive).count()
            ),
            format!(
                "Export Review List ({})",
                review_list.len()
            ),
            "Exit".to_string(),
        ];

        let selection = Select::new()
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => view_duplicates(duplicates, &mut review_list)?,
            1 => view_large_files(scan_result, large_threshold, &mut review_list)?,
            2 => view_empty_dirs(scan_result),
            3 => view_installers(scan_result, &mut review_list)?,
            4 => view_archives(scan_result, &mut review_list)?,
            5 => export_review_list(&review_list)?,
            6 => {
                if !review_list.is_empty() {
                    println!(
                        "\n  {} You have {} files in your review list.",
                        "⚠".yellow(),
                        review_list.len().to_string().yellow()
                    );
                    println!("  Consider exporting before exiting.\n");

                    let confirm_options = vec!["Export and Exit", "Exit without exporting", "Go back"];
                    let choice = Select::new()
                        .items(&confirm_options)
                        .default(0)
                        .interact()?;

                    match choice {
                        0 => {
                            export_review_list(&review_list)?;
                            break;
                        }
                        1 => break,
                        _ => continue,
                    }
                } else {
                    println!("\n  👋 Goodbye!\n");
                    break;
                }
            }
            _ => break,
        }
    }

    Ok(())
}

/// Displays duplicate groups one by one and lets the user mark files for review.
fn view_duplicates(
    duplicates: &[DuplicateGroup],
    review_list: &mut Vec<PathBuf>,
) -> Result<()> {
    if duplicates.is_empty() {
        println!("\n  {} No duplicate files found.\n", "✓".green());
        return Ok(());
    }

    let mut index = 0;

    loop {
        if index >= duplicates.len() {
            println!("\n  {} End of duplicate groups.\n", "─".cyan());
            break;
        }

        let group = &duplicates[index];

        println!("\n{}", "──────────────────────────────────────".cyan());
        println!(
            "  {} {}/{}",
            "Duplicate Group".bold().cyan(),
            (index + 1).to_string().yellow(),
            duplicates.len().to_string().yellow()
        );
        println!("{}", "──────────────────────────────────────".cyan());
        println!(
            "  File size: {} | Recoverable: {}",
            format_size(group.file_size).white(),
            format_size(group.recoverable_space()).red().bold()
        );
        println!();

        for (i, path) in group.paths.iter().enumerate() {
            let already_marked = review_list.contains(path);
            let marker = if already_marked {
                "✗".red().to_string()
            } else {
                format!("{}", (i + 1))
            };
            println!("  [{}] {}", marker, path.display());
        }
        println!();

        // Build the options for this group.
        let mut options: Vec<String> = group
            .paths
            .iter()
            .enumerate()
            .map(|(i, path)| {
                if review_list.contains(path) {
                    format!("Unmark [{}] {}", i + 1, path.display())
                } else {
                    format!("Mark [{}] for review", i + 1)
                }
            })
            .collect();

        options.push("Next Group".to_string());
        if index > 0 {
            options.push("Previous Group".to_string());
        }
        options.push("Back to menu".to_string());

        let choice = Select::new().items(&options).default(0).interact()?;

        if choice < group.paths.len() {
            // Mark or unmark a file.
            let path = &group.paths[choice];
            if review_list.contains(path) {
                review_list.retain(|p| p != path);
                println!("  {} Unmarked: {}", "↩".yellow(), path.display());
            } else {
                review_list.push(path.clone());
                println!("  {} Marked for review: {}", "✓".green(), path.display());
            }
            // Stay on the same group so the user can mark more files.
            continue;
        }

        let action_index = choice - group.paths.len();
        match action_index {
            0 => index += 1, // Next Group
            1 if index > 0 => index -= 1, // Previous Group
            _ => break,     // Back to menu
        }
    }

    Ok(())
}

/// Displays large files sorted by size (largest first).
fn view_large_files(
    scan_result: &ScanResult,
    threshold: u64,
    review_list: &mut Vec<PathBuf>,
) -> Result<()> {
    let mut large_files: Vec<_> = scan_result
        .files
        .iter()
        .filter(|f| f.size >= threshold)
        .collect();

    if large_files.is_empty() {
        println!(
            "\n  {} No files larger than {} found.\n",
            "✓".green(),
            format_size(threshold)
        );
        return Ok(());
    }

    // Sort largest first.
    large_files.sort_by_key(|f| std::cmp::Reverse(f.size));

    println!("\n{}", "── Large Files ──".cyan().bold());
    println!(
        "  Showing files larger than {}\n",
        format_size(threshold).yellow()
    );

    for (i, file) in large_files.iter().enumerate() {
        let marked = if review_list.contains(&file.path) {
            " [MARKED]".red().to_string()
        } else {
            String::new()
        };
        println!(
            "  {:>3}. {} — {}{}",
            i + 1,
            format_size(file.size).yellow(),
            file.path.display(),
            marked
        );
    }
    println!();

    // Let the user mark files from the list.
    loop {
        let mut options: Vec<String> = large_files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                if review_list.contains(&f.path) {
                    format!("Unmark #{} ({})", i + 1, f.path.display())
                } else {
                    format!("Mark #{} for review ({})", i + 1, format_size(f.size))
                }
            })
            .collect();
        options.push("Back to menu".to_string());

        let choice = Select::new().items(&options).default(options.len() - 1).interact()?;

        if choice < large_files.len() {
            let path = &large_files[choice].path;
            if review_list.contains(path) {
                review_list.retain(|p| p != path);
                println!("  {} Unmarked: {}", "↩".yellow(), path.display());
            } else {
                review_list.push(path.clone());
                println!("  {} Marked for review: {}", "✓".green(), path.display());
            }
        } else {
            break;
        }
    }

    Ok(())
}

/// Displays empty directories.
fn view_empty_dirs(scan_result: &ScanResult) {
    if scan_result.empty_dirs.is_empty() {
        println!("\n  {} No empty directories found.\n", "✓".green());
        return;
    }

    println!("\n{}", "── Empty Directories ──".cyan().bold());
    println!();

    for (i, dir) in scan_result.empty_dirs.iter().enumerate() {
        println!("  {:>3}. {}", i + 1, dir.display());
    }
    println!(
        "\n  Total: {} empty directories\n",
        scan_result.empty_dirs.len().to_string().yellow()
    );
}

/// Displays files with installer extensions.
fn view_installers(
    scan_result: &ScanResult,
    review_list: &mut Vec<PathBuf>,
) -> Result<()> {
    let installers: Vec<_> = scan_result
        .files
        .iter()
        .filter(|f| f.is_installer)
        .collect();

    if installers.is_empty() {
        println!("\n  {} No installer files found.\n", "✓".green());
        return Ok(());
    }

    println!("\n{}", "── Old Installers (Review Recommended) ──".cyan().bold());
    println!();

    for (i, file) in installers.iter().enumerate() {
        let marked = if review_list.contains(&file.path) {
            " [MARKED]".red().to_string()
        } else {
            String::new()
        };
        println!(
            "  {:>3}. {} — {}{}",
            i + 1,
            format_size(file.size).yellow(),
            file.path.display(),
            marked
        );
    }
    println!();

    loop {
        let mut options: Vec<String> = installers
            .iter()
            .enumerate()
            .map(|(i, f)| {
                if review_list.contains(&f.path) {
                    format!("Unmark #{} ({})", i + 1, f.path.display())
                } else {
                    format!("Mark #{} for review", i + 1)
                }
            })
            .collect();
        options.push("Back to menu".to_string());

        let choice = Select::new().items(&options).default(options.len() - 1).interact()?;

        if choice < installers.len() {
            let path = &installers[choice].path;
            if review_list.contains(path) {
                review_list.retain(|p| p != path);
                println!("  {} Unmarked: {}", "↩".yellow(), path.display());
            } else {
                review_list.push(path.clone());
                println!("  {} Marked for review: {}", "✓".green(), path.display());
            }
        } else {
            break;
        }
    }

    Ok(())
}

/// Displays archive files.
fn view_archives(
    scan_result: &ScanResult,
    review_list: &mut Vec<PathBuf>,
) -> Result<()> {
    let archives: Vec<_> = scan_result
        .files
        .iter()
        .filter(|f| f.is_archive)
        .collect();

    if archives.is_empty() {
        println!("\n  {} No archive files found.\n", "✓".green());
        return Ok(());
    }

    println!(
        "\n{}",
        "── Archive Files (Review Recommended) ──".cyan().bold()
    );
    println!();

    for (i, file) in archives.iter().enumerate() {
        let marked = if review_list.contains(&file.path) {
            " [MARKED]".red().to_string()
        } else {
            String::new()
        };
        println!(
            "  {:>3}. {} — {}{}",
            i + 1,
            format_size(file.size).yellow(),
            file.path.display(),
            marked
        );
    }
    println!();

    loop {
        let mut options: Vec<String> = archives
            .iter()
            .enumerate()
            .map(|(i, f)| {
                if review_list.contains(&f.path) {
                    format!("Unmark #{} ({})", i + 1, f.path.display())
                } else {
                    format!("Mark #{} for review", i + 1)
                }
            })
            .collect();
        options.push("Back to menu".to_string());

        let choice = Select::new().items(&options).default(options.len() - 1).interact()?;

        if choice < archives.len() {
            let path = &archives[choice].path;
            if review_list.contains(path) {
                review_list.retain(|p| p != path);
                println!("  {} Unmarked: {}", "↩".yellow(), path.display());
            } else {
                review_list.push(path.clone());
                println!("  {} Marked for review: {}", "✓".green(), path.display());
            }
        } else {
            break;
        }
    }

    Ok(())
}

/// Exports the review list to a text file.
fn export_review_list(review_list: &[PathBuf]) -> Result<()> {
    if review_list.is_empty() {
        println!(
            "\n  {} Review list is empty. Mark some files first.\n",
            "!".yellow()
        );
        return Ok(());
    }

    let mut content = String::new();
    content.push_str("# DiskDoctor Review List\n");
    content.push_str("# Generated by DiskDoctor\n");
    content.push_str("# Review each entry carefully before taking action.\n");
    content.push_str("# DiskDoctor does NOT delete files automatically.\n\n");

    for path in review_list {
        content.push_str(&format!("DELETE  {}\n", path.display()));
    }

    let output_path = "review_list.txt";
    fs::write(output_path, &content)?;

    println!("\n  {} Review list exported to:", "✓".green().bold());
    println!("  {}", output_path.to_string().cyan());
    println!(
        "  {} files listed.\n",
        review_list.len().to_string().yellow()
    );

    Ok(())
}
