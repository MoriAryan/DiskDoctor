# DiskDoctor

I was about to download a large file when Windows politely reminded me that my C: drive had other plans.

After digging through folders manually and trying a few disk cleanup tools, I realized most of them either felt cluttered or jumped straight to deleting things. I wanted something simpler: a tool that tells me **what deserves my attention**, while leaving the final decision to me.

That's how **DiskDoctor** was born.

DiskDoctor is a lightweight Rust CLI that scans your storage, finds duplicate files, large files, empty folders, installers, and archives, then generates a report with files worth reviewing.

> **Analyze first. Act later.**

---

## Demo

<video src="assets/demo.mp4" width="600" muted loop autoplay controls ></video>

---

## Features

- Recursive directory scanning
- Duplicate file detection using SHA-256 hashing
- Large file detection (configurable threshold)
- Empty directory detection
- Installer file detection (`.exe`, `.msi`, `.pkg`, `.deb`, `.rpm`)
- Archive file detection (`.zip`, `.rar`, `.7z`, `.tar`, `.gz`)
- Interactive terminal menu
- Export review list
- Progress indicators while scanning
- Graceful error handling

---

## Getting Started

### Prerequisites

- Rust (1.70+)

Install Rust from:

https://www.rust-lang.org/tools/install

Clone the repository:

```bash
git clone https://github.com/MoriAryan/DiskDoctor.git
cd DiskDoctor
```

Build the project:

```bash
cargo build --release
```

Run it from the project directory using:

```bash
cargo run
```

or

```bash
cargo run -- "C:\Users\YourName\Downloads"(i.e path to the directory you want to scan)
```

You can also execute the compiled binary:

```bash
target/release/diskdoctor
```

(On Windows, use `target\release\diskdoctor.exe`.)

DiskDoctor works in any terminal such as:

- VS Code Terminal
- Windows Command Prompt (CMD)
- PowerShell
- Windows Terminal

---

## Example

```bash
cargo run -- "C:\Users\Aryan\Downloads"
```

or

```bash
cargo run -- ~/Downloads
```

For a custom large file threshold:

```bash
cargo run -- ~/Downloads --size 100
```

---

## How Duplicate Detection Works

Instead of hashing every file immediately, DiskDoctor first groups files by their size.

Only files that share the same size are hashed using SHA-256 and compared.

This significantly reduces unnecessary hashing while still ensuring exact duplicate detection.

---

## Safety First

DiskDoctor **never deletes your files automatically.**

It only reports findings and lets you export a review list so you can decide what should be removed.

---

## Future Improvements

- Desktop GUI
- Safe deletion by moving files to the system Recycle Bin/Trash
- Scan multiple directories
- File age analysis
- Faster parallel scanning

---

Made with ❤️ by Aryan