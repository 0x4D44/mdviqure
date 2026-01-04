# mdviqure

`mdviqure` is a Rust CLI tool designed to reduce the size of MP4 video files to fit specific target constraints (currently 50MB or 100MB). It achieves this by calculating the optimal video bitrate based on the video's duration and a constant audio bitrate, then re-encoding the file using FFMPEG.

## Prerequisites

*   **Rust**: Ensure you have the Rust toolchain installed (cargo, rustc).
*   **FFMPEG**: The tool relies on `ffmpeg` and `ffprobe` executables being available in your system's PATH to analyze and process video files.

## Project Structure

*   `src/main.rs`: Contains the application entry point, argument parsing logic (via `clap`), video processing logic, and unit tests.
*   `Cargo.toml`: Defines project metadata and dependencies.

## Building and Running

### Build

To build the project in release mode:

```bash
cargo build --release
```

### Run

Run the tool using `cargo run`. You must specify the input file, output file, and optionally the target size (defaults to 100MB).

**Usage:**

```bash
cargo run -- <input_file> <output_file> [OPTIONS]
```

**Options:**

*   `--size <SIZE>`: Target size in MB. Must be either `50` or `100`. Default is `100`.

**Examples:**

Compress `input.mp4` to a target of 100MB:

```bash
cargo run -- input.mp4 output.mp4
```

Compress `large_video.mp4` to a target of 50MB:

```bash
cargo run -- large_video.mp4 small_video.mp4 --size 50
```

## Testing

The project includes unit tests for the bitrate calculation logic and duration parsing. Run the tests using:

```bash
cargo test
```

## Logic Overview

1.  **Duration Extraction**: Uses `ffprobe` to determine the length of the input video in seconds.
2.  **Bitrate Calculation**:
    *   Calculates the total available bits: `target_size_mb * 1024 * 1024 * 8`.
    *   Subtracts the estimated audio bitrate (constant 128kbps).
    *   The remainder is the target video bitrate (clamped to a minimum of 100kbps).
3.  **Encoding**: Invokes `ffmpeg` to re-encode the video stream using `libx264` at the calculated bitrate and the audio stream using `aac` at 128kbps.

## Development

*   **Argument Parsing**: Uses `clap` v4.0 with derive macros.
*   **Error Handling**: Basic `Result<Box<dyn Error>>` propagation.
*   **External Commands**: Uses `std::process::Command` to interact with FFMPEG tools.
