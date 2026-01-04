# mdviqure

`mdviqure` is a robust Rust command-line tool designed to intelligently reduce the size of MP4 video files to fit specific constraints (typically 50MB or 100MB). It automates the complex calculation of bitrates required to achieve a target file size without manual guesswork.

## 🚀 Features

*   **Smart Bitrate Calculation**: Automatically calculates the optimal video bitrate based on the file duration and a standard audio overhead (128kbps).
*   **Targeted Compression**: Supports specific target sizes (default 100MB, optionally 50MB) ideal for upload limits on various platforms (e.g., Discord, Email).
*   **FFmpeg Integration**: Leverages the industry-standard `ffmpeg` and `ffprobe` for high-quality encoding (libx264/aac).
*   **Safety**: Validates inputs and clamps bitrates to a minimum usable threshold (100kbps) to prevent corruption on extremely long videos.
*   **Testable Architecture**: Built with a decoupled design using dependency injection, ensuring high reliability and test coverage.

## 📋 Prerequisites

Before running `mdviqure`, ensure you have the following installed:

1.  **Rust Toolchain**: To build the project.
    *   [Install Rust](https://www.rust-lang.org/tools/install)
2.  **FFmpeg**: The core processing engine.
    *   **Windows**: `winget install "FFmpeg (Essentials)"` or download from [gyan.dev](https://www.gyan.dev/ffmpeg/builds/).
    *   **macOS**: `brew install ffmpeg`
    *   **Linux**: `sudo apt install ffmpeg`

*Note: `ffmpeg` and `ffprobe` must be in your system's `PATH`.*

## 🛠️ Installation & Building

Clone the repository and build the release binary:

```bash
git clone https://github.com/your-username/mdviqure.git
cd mdviqure
cargo build --release
```

The compiled binary will be located at `target/release/mdviqure`.

## 💻 Usage

```bash
mdviqure <INPUT> <OUTPUT> [OPTIONS]
```

### Arguments

*   `<INPUT>`: Path to the source MP4 video file.
*   `<OUTPUT>`: Path where the compressed video will be saved.

### Options

*   `-s, --size <SIZE>`: Target file size in Megabytes (MB).
    *   Allowed values: `50`, `100`
    *   Default: `100`
*   `-h, --help`: Print help information.
*   `-V, --version`: Print version information.

### Examples

**Standard Compression (Target 100MB)**
```bash
cargo run --release -- input.mp4 output_100mb.mp4
```

**Aggressive Compression (Target 50MB)**
```bash
cargo run --release -- input.mp4 output_50mb.mp4 --size 50
```

## 🏗️ Architecture & Development

The codebase is structured for maintainability and testability:

*   **`VideoTool` Trait**: Abstracts the external system calls to `ffmpeg` and `ffprobe`. This allows the core logic to be tested without needing actual video files or the FFmpeg runtime.
*   **Dependency Injection**: The main application flow receives a `VideoTool` implementation. In production, this is `FfmpegTool`; in tests, it is `MockVideoTool`.

### Running Tests

The project maintains high test coverage (>90%) covering business logic, argument parsing, and workflow orchestration.

```bash
cargo test
```

### Checking Coverage

To generate a coverage report (requires `cargo-llvm-cov`):

```bash
cargo llvm-cov
```

## 📄 License

[MIT License](LICENSE)
