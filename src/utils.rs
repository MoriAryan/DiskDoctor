use std::path::Path;

/// Formats a byte count into a human-readable string (e.g., "4.6 GB").
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Known installer file extensions.
const INSTALLER_EXTENSIONS: &[&str] = &["exe", "msi", "pkg", "deb", "rpm"];

/// Known archive file extensions.
const ARCHIVE_EXTENSIONS: &[&str] = &["zip", "rar", "7z", "tar", "gz"];

/// Checks if a file path has an installer extension.
pub fn is_installer(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => INSTALLER_EXTENSIONS.contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}

/// Checks if a file path has an archive extension.
pub fn is_archive(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ARCHIVE_EXTENSIONS.contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}
