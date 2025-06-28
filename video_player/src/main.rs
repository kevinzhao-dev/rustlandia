use clap::Parser;
use std::process::{Command, Stdio};

/// A simple CLI video player that wraps ffplay
#[derive(Parser, Debug)]
#[command(name = "rvp", version, about)]
struct Args {
    /// Path to the video file to play
    video_path: String,
    /// Playback speed multiplier (e.g. 1.0 normal, 1.5 faster, 0.5 slower)
    #[arg(short = 'r', long, default_value_t = 1.0)]
    speed: f32,
    /// Start playback at given offset (seconds)
    #[arg(short = 's', long, default_value_t = 0.0)]
    start: f32,
}

fn main() {
    let args = Args::parse();

    // Build ffplay command with optional start offset and speed filters
    let mut cmd = Command::new("ffplay");
    // suppress initial banner, show info-level logs, inherit stderr for speed/seek messages
    cmd.args(&["-hide_banner", "-loglevel", "info"]);
    cmd.stderr(Stdio::inherit());
    if args.start > 0.0 {
        cmd.args(&["-ss", &args.start.to_string()]);
    }
    cmd.arg("-autoexit");
    if (args.speed - 1.0).abs() > f32::EPSILON {
        cmd.args(&["-vf", &format!("setpts=PTS/{:.3}", args.speed)]);
        cmd.args(&["-af", &format!("atempo={:.3}", args.speed)]);
    }
    cmd.arg(&args.video_path);
    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("Error launching ffplay: {}", e);
        std::process::exit(1);
    });

    std::process::exit(status.code().unwrap_or(1));
}
