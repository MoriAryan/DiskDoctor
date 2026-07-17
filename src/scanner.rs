use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

use crate::utils;

/// Information about a single file found during scanning.
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub is_installer: bool,
    pub is_archive: bool,
}

/// The result of scanning a directory.
pub struct ScanResult {
    pub files: Vec<FileInfo>,
    pub empty_dirs: Vec<PathBuf>,
    pub file_count: usize,
    pub dir_count: usize,
    pub skipped_count: usize,
}

/// Scans a directory recursively, collecting file info and empty directories.
///
/// Shows a progress bar during scanning. Permission errors are skipped
/// with a warning message instead of crashing.
pub fn scan_directory(root: &Path) -> Result<ScanResult> {
    // First pass: count total entries so we can show a meaningful progress bar.
    // We silently skip errors in this pass — they'll be reported in the real scan.
    let total_entries = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .count() as u64;

    let progress = ProgressBar::new(total_entries);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{wide_bar:.cyan/dark_gray} {pos}/{len} ({percent}%)")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("██░"),
    );
    progress.set_message("Scanning...");

    let mut files = Vec::new();
    let mut empty_dirs = Vec::new();
    let mut file_count: usize = 0;
    let mut dir_count: usize = 0;
    let mut skipped_count: usize = 0;

    // Second pass: actually collect the data.
    for entry in WalkDir::new(root).into_iter() {
        progress.inc(1);

        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                // Permission denied or other access errors — skip gracefully.
                if let Some(path) = err.path() {
                    progress.suspend(|| {
                        eprintln!("  ⚠ Permission denied: Skipped {}", path.display());
                    });
                } else {
                    progress.suspend(|| {
                        eprintln!("  ⚠ Skipped entry: {}", err);
                    });
                }
                skipped_count += 1;
                continue;
            }
        };

        let path = entry.path();

        if entry.file_type().is_dir() {
            dir_count += 1;

            // Check if directory is empty.
            let is_empty = std::fs::read_dir(path)
                .map(|mut entries| entries.next().is_none())
                .unwrap_or(false);

            if is_empty {
                empty_dirs.push(path.to_path_buf());
            }
        } else if entry.file_type().is_file() {
            file_count += 1;

            let metadata = entry
                .metadata()
                .with_context(|| format!("Failed to read metadata for {}", path.display()))?;

            let size = metadata.len();

            files.push(FileInfo {
                path: path.to_path_buf(),
                size,
                is_installer: utils::is_installer(path),
                is_archive: utils::is_archive(path),
            });
        }
    }

    progress.finish_with_message("Scan complete!");

    Ok(ScanResult {
        files,
        empty_dirs,
        file_count,
        dir_count,
        skipped_count,
    })
}
