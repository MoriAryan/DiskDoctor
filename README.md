# 🩺 DiskDoctor

**DiskDoctor** is a command-line utility written in Rust that scans directories and helps you understand where your storage is being wasted.

> **Philosophy:** "Analyze first. Act later."

DiskDoctor **never automatically deletes** any files. It only analyzes your disk and makes recommendations. You stay in control.

---

## Features

- **Recursive Directory Scanning** — scans any folder and all its subfolders
- **Duplicate File Detection** — finds exact duplicates using SHA-256 hashing
- **Large File Detection** — identifies files above a configurable size threshold
- **Empty Directory Detection** — finds folders with nothing in them
- **Installer Detection** — flags `.exe`, `.msi`, `.pkg`, `.deb`, `.rpm` files for review
- **Archive Detection** — flags `.zip`, `.rar`, `.7z`, `.tar`, `.gz` files for review
- **Interactive Menu** — browse findings with arrow-key navigation
- **Review List Export** — mark files and export a `review_list.txt` for manual review
- **Progress Indicators** — visual progress bars during scanning and hashing
- **Friendly Error Handling** — permission errors are skipped gracefully, never crashes

---

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later recommended)

### Build from Source

```bash
git clone https://github.com/yourusername/diskdoctor.git
cd diskdoctor
cargo build --release
```

The binary will be at `target/release/diskdoctor` (or `diskdoctor.exe` on Windows).

---

## Usage

### Basic Scan (current directory)

```bash
diskdoctor
```

### Scan a Specific Directory

```bash
diskdoctor ~/Downloads
```

### Custom Large File Threshold

By default, files larger than **500 MB** are flagged. Change it with `--size`:

```bash
diskdoctor ~/Downloads --size 100   # Flag files larger than 100 MB
```

### Example Output

```
══════════════════════════════════════════
          DiskDoctor Report
══════════════════════════════════════════

  Files scanned:          4,832
  Folders scanned:          731
  Duplicate Groups:          17
  Potential Space Recoverable: 8.6 GB

  Large Files (>500.0 MB):    11
  Empty Directories:          24
  Old Installers:              6
  Archive Files:               9

══════════════════════════════════════════
  No files were modified or deleted.
══════════════════════════════════════════
```

After the report, an interactive menu lets you browse each category, mark files for review, and export a list.

---

## Project Structure

```
src/
├── main.rs        # Entry point, CLI parsing
├── scanner.rs     # Directory traversal + progress bar
├── duplicate.rs   # SHA-256 duplicate detection
├── report.rs      # Summary report printing
├── menu.rs        # Interactive post-scan menu
└── utils.rs       # Helper functions (file size formatting, extension checks)
```

---

## How It Works

### Duplicate Detection

1. **Group by size** — files with unique sizes can't be duplicates, so they're skipped immediately.
2. **Hash same-size files** — only files that share the same size are hashed with SHA-256.
3. **Group by hash** — files with identical hashes are true duplicates.

This two-step approach avoids hashing every file, making it much faster on large directories.

### File Categorization

Files are categorized by extension during the initial scan:
- **Installers:** `.exe`, `.msi`, `.pkg`, `.deb`, `.rpm`
- **Archives:** `.zip`, `.rar`, `.7z`, `.tar`, `.gz`

These are flagged for *review only* — DiskDoctor never assumes they're safe to delete.

---

## Future Improvements

- [ ] Add a `--output` flag to specify the review list output path
- [ ] Support for more archive and installer formats
- [ ] Option to scan multiple directories at once
- [ ] File age analysis (e.g., "not accessed in 1+ year")
- [ ] TUI dashboard with real-time statistics
- [ ] Configuration file support (`.diskdoctorrc`)
- [ ] Parallel hashing for faster duplicate detection
- [ ] Integration with system trash instead of direct deletion

---

## Learning Notes

This project was built as an internship project to learn Rust. Here are some key concepts it demonstrates:

- **Ownership & Borrowing** — passing references to scan results and review lists through the menu system
- **Error Handling** — using `anyhow::Result` and the `?` operator for clean error propagation
- **Pattern Matching** — `match` expressions for menu selections and file type checks
- **Iterators** — `.filter()`, `.map()`, `.collect()` chains for data processing
- **Structs** — `FileInfo`, `ScanResult`, `DuplicateGroup` for organizing data
- **Modules** — splitting code across files with `mod` declarations
- **External Crates** — integrating the Rust ecosystem (clap, walkdir, sha2, etc.)

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| [clap](https://crates.io/crates/clap) | CLI argument parsing |
| [walkdir](https://crates.io/crates/walkdir) | Recursive directory traversal |
| [sha2](https://crates.io/crates/sha2) | SHA-256 hashing |
| [indicatif](https://crates.io/crates/indicatif) | Progress bars |
| [colored](https://crates.io/crates/colored) | Colored terminal output |
| [dialoguer](https://crates.io/crates/dialoguer) | Interactive menus |
| [anyhow](https://crates.io/crates/anyhow) | Error handling |

---

## License

This project is open source and available under the [MIT License](LICENSE).
