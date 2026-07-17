use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};

use crate::scanner::FileInfo;

/// A group of files that are exact duplicates of each other.
pub struct DuplicateGroup {
    pub paths: Vec<PathBuf>,
    pub file_size: u64,
}

impl DuplicateGroup {
    /// Space that could be recovered by keeping only one copy.
    pub fn recoverable_space(&self) -> u64 {
        // Keep one file, the rest are "recoverable".
        self.file_size * (self.paths.len() as u64 - 1)
    }
}

/// Finds duplicate files using a size-first, then SHA-256 hash approach.
///
/// Step 1: Group files by size. Files with unique sizes can't be duplicates.
/// Step 2: For same-size groups, hash the files and group by hash.
///
/// This avoids hashing every single file, which would be very slow.
pub fn find_duplicates(files: &[FileInfo]) -> Result<Vec<DuplicateGroup>> {
    // Step 1: Group by file size.
    let mut size_groups: HashMap<u64, Vec<&FileInfo>> = HashMap::new();
    for file in files {
        // Skip empty files — they're trivially "identical" but not useful duplicates.
        if file.size > 0 {
            size_groups.entry(file.size).or_default().push(file);
        }
    }

    // Filter to only groups with 2+ files (potential duplicates).
    let candidates: Vec<_> = size_groups
        .into_values()
        .filter(|group| group.len() > 1)
        .collect();

    let total_to_hash: usize = candidates.iter().map(|g| g.len()).sum();

    if total_to_hash == 0 {
        return Ok(Vec::new());
    }

    let progress = ProgressBar::new(total_to_hash as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{wide_bar:.magenta/dark_gray} {pos}/{len}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("██░"),
    );
    progress.set_message("Hashing files for duplicate detection...");

    // Step 2: Hash each candidate group and look for true duplicates.
    let mut duplicates = Vec::new();

    for group in &candidates {
        let mut hash_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for file in group {
            progress.inc(1);

            match compute_hash(&file.path) {
                Ok(hash) => {
                    hash_groups
                        .entry(hash)
                        .or_default()
                        .push(file.path.clone());
                }
                Err(err) => {
                    progress.suspend(|| {
                        eprintln!("  ⚠ Could not hash {}: {}", file.path.display(), err);
                    });
                }
            }
        }

        // Collect groups where 2+ files share the same hash.
        let file_size = group[0].size;
        for (_hash, paths) in hash_groups {
            if paths.len() > 1 {
                duplicates.push(DuplicateGroup { paths, file_size });
            }
        }
    }

    progress.finish_with_message("Hashing complete!");

    // Sort by recoverable space (largest first) for a nicer report.
    duplicates.sort_by_key(|d| std::cmp::Reverse(d.recoverable_space()));

    Ok(duplicates)
}

/// Computes the SHA-256 hash of a file by reading it in chunks.
/// This keeps memory usage low even for very large files.
fn compute_hash(path: &PathBuf) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // 8 KB chunks

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}
