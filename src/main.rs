use clap::Parser;
use std::error::Error;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about = "Reduce MP4 video quality to fit within a target size (50MB or 100MB) using FFMPEG", long_about = None)]
struct Args {
    /// Input video file (MP4)
    input: String,

    /// Output video file
    output: String,

    /// Target size in MB (must be either 50 or 100)
    #[arg(short, long, default_value_t = 100)]
    size: u64,
}

/// Uses ffprobe to get the duration (in seconds) of the input video.
fn get_video_duration(input: &str) -> Result<f64, Box<dyn Error>> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            input,
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("ffprobe failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let duration: f64 = stdout.trim().parse()?;
    Ok(duration)
}

/// Computes the video bitrate (in bits per second) needed so that:
///
///    (video_bitrate + audio_bitrate) * duration / 8 ≈ target file size in bytes.
///
/// If the computed video bitrate is too low, a minimum of 100_000 bps is used.
fn compute_video_bitrate(duration: f64, target_bytes: u64, audio_bitrate: u64) -> u64 {
    // Total bitrate (in bits per second) needed to hit the target file size.
    let total_bitrate = (target_bytes * 8) as f64 / duration;
    // Subtract the (assumed constant) audio bitrate.
    let video_bitrate = total_bitrate - (audio_bitrate as f64);
    // Use a minimum value if needed.
    let min_video_bitrate = 100_000.0;
    if video_bitrate < min_video_bitrate {
        min_video_bitrate as u64
    } else {
        video_bitrate as u64
    }
}

/// Reduces the quality of the input video to hit roughly the target file size (in MB).
///
/// This function:
/// 1. Obtains the video duration via ffprobe.
/// 2. Computes a target video bitrate (assuming a fixed 128kb/s for audio).
/// 3. Calls ffmpeg to re‑encode the video.
fn reduce_video(input: &str, output: &str, target_mb: u64) -> Result<(), Box<dyn Error>> {
    // Get video duration in seconds.
    let duration = get_video_duration(input)?;
    // Convert target size from MB to bytes (using 1 MB = 1024 * 1024 bytes).
    let target_bytes = target_mb * 1024 * 1024;
    // Assume a constant audio bitrate of 128 kb/s.
    let audio_bitrate = 128_000; // in bits per second

    let video_bitrate = compute_video_bitrate(duration, target_bytes, audio_bitrate);
    // ffmpeg accepts bitrates in a suffix form (e.g. "500k" for 500 kb/s). We convert bps -> kbps.
    let video_bitrate_str = format!("{}k", video_bitrate / 1000);

    println!("Video duration: {:.2} seconds", duration);
    println!("Target size: {} MB", target_mb);
    println!("Using video bitrate: {} ({} bps)", video_bitrate_str, video_bitrate);

    // Call ffmpeg to re-encode the video.
    // The command-line below tells ffmpeg to overwrite the output file (-y),
    // use libx264 for video encoding with our computed bitrate, and
    // encode audio using AAC at 128k.
    let status = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            input,
            "-c:v",
            "libx264",
            "-b:v",
            &video_bitrate_str,
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            output,
        ])
        .status()?;

    if !status.success() {
        return Err("ffmpeg failed during encoding".into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Validate that the provided size is either 50MB or 100MB.
    if args.size != 50 && args.size != 100 {
        return Err("Target size must be either 50 or 100 MB.".into());
    }

    reduce_video(&args.input, &args.output, args.size)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_video_bitrate() {
        // For a 100-second video and a target size of 100MB:
        // 100 MB = 104857600 bytes; total bits = 104857600 * 8 = 838860800.
        // Total bitrate = 838860800 / 100 = 8,388,608 bps.
        // Expected video bitrate = 8,388,608 - 128,000 = 8,260,608 bps (approximately).
        let duration = 100.0;
        let target_bytes = 100 * 1024 * 1024;
        let audio_bitrate = 128_000;
        let video_bitrate = compute_video_bitrate(duration, target_bytes, audio_bitrate);
        let expected = 8_388_608.0 - 128_000.0;
        assert!((video_bitrate as f64 - expected).abs() < 1_000.0);
    }

    #[test]
    fn test_minimum_video_bitrate() {
        // Create a scenario where the computed video bitrate would fall below the minimum.
        // With a very long duration, the computed video bitrate could be negative.
        let duration = 10_000.0; // very long video
        let target_bytes = 50 * 1024 * 1024; // 50 MB target
        let audio_bitrate = 128_000;
        let video_bitrate = compute_video_bitrate(duration, target_bytes, audio_bitrate);
        // In this case the computed video bitrate should be clamped to the minimum of 100_000.
        assert_eq!(video_bitrate, 100_000);
    }

    #[test]
    fn test_parse_duration() {
        // A simple test to parse a duration string.
        let example_output = "123.456\n";
        let duration: f64 = example_output.trim().parse().unwrap();
        assert!((duration - 123.456).abs() < 0.001);
    }
}
