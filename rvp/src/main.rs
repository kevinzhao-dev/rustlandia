use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, terminal,
};
use std::{
    io::{self, Write},
    process::{Command, Stdio},
    time::Duration,
};

/// A simple CLI video player that wraps ffplay with keyboard controls
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

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Build ffplay command with optional start offset and speed filters
    let mut cmd = Command::new("ffplay");
    // suppress initial banner, show info-level logs, inherit stderr for speed/seek messages
    cmd.args(&["-hide_banner", "-loglevel", "info"]);
    cmd.stderr(Stdio::inherit());
    cmd.stdout(Stdio::null()); // Hide ffplay output
    if args.start > 0.0 {
        cmd.args(&["-ss", &args.start.to_string()]);
    }
    cmd.arg("-autoexit");
    if (args.speed - 1.0).abs() > f32::EPSILON {
        cmd.args(&["-vf", &format!("setpts=PTS/{:.3}", args.speed)]);
        cmd.args(&["-af", &format!("atempo={:.3}", args.speed)]);
    }
    cmd.arg(&args.video_path);

    // Start ffplay process
    let mut child = cmd.spawn().unwrap_or_else(|e| {
        eprintln!("Error launching ffplay: {}", e);
        std::process::exit(1);
    });

    // Setup terminal for raw mode
    terminal::enable_raw_mode()?;

    // Print controls
    println!("Video Player Controls:");
    println!("  Space - Play/Pause");
    println!("  Left/Right - Seek 10 seconds");
    println!("  Up/Down - Seek 1 minute");
    println!("  Q - Quit");
    println!("  ? - Show this help");
    println!();

    let mut current_position = args.start;
    let mut is_paused = false;

    loop {
        // Check if ffplay process is still running
        match child.try_wait() {
            Ok(Some(status)) => {
                terminal::disable_raw_mode()?;
                std::process::exit(status.code().unwrap_or(0));
            }
            Ok(None) => {
                // Process is still running, continue
            }
            Err(e) => {
                eprintln!("Error checking process status: {}", e);
                break;
            }
        }

        // Check for keyboard input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        // Quit
                        break;
                    }
                    KeyCode::Char(' ') => {
                        // Play/Pause
                        is_paused = !is_paused;
                        if is_paused {
                            // For now, we'll just print the pause action
                            // In a full implementation, you'd need to handle this differently
                            println!("Paused (note: ffplay doesn't support pause via signals)");
                        } else {
                            println!("Playing");
                        }
                    }
                    KeyCode::Left => {
                        // Seek backward 10 seconds
                        current_position = (current_position - 10.0).max(0.0);
                        seek_to_position(
                            &mut child,
                            current_position,
                            &args.video_path,
                            args.speed,
                        );
                        println!("Seeked -10s (position: {:.1}s)", current_position);
                    }
                    KeyCode::Right => {
                        // Seek forward 10 seconds
                        current_position += 10.0;
                        seek_to_position(
                            &mut child,
                            current_position,
                            &args.video_path,
                            args.speed,
                        );
                        println!("Seeked +10s (position: {:.1}s)", current_position);
                    }
                    KeyCode::Up => {
                        // Seek forward 1 minute
                        current_position += 60.0;
                        seek_to_position(
                            &mut child,
                            current_position,
                            &args.video_path,
                            args.speed,
                        );
                        println!("Seeked +1m (position: {:.1}s)", current_position);
                    }
                    KeyCode::Down => {
                        // Seek backward 1 minute
                        current_position = (current_position - 60.0).max(0.0);
                        seek_to_position(
                            &mut child,
                            current_position,
                            &args.video_path,
                            args.speed,
                        );
                        println!("Seeked -1m (position: {:.1}s)", current_position);
                    }
                    KeyCode::Char('?') => {
                        // Show help
                        println!("\nVideo Player Controls:");
                        println!("  Space - Play/Pause");
                        println!("  Left/Right - Seek 10 seconds");
                        println!("  Up/Down - Seek 1 minute");
                        println!("  Q - Quit");
                        println!("  ? - Show this help");
                        println!();
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    terminal::disable_raw_mode()?;

    // Kill ffplay process
    let _ = child.kill();

    Ok(())
}

fn seek_to_position(child: &mut std::process::Child, position: f32, video_path: &str, speed: f32) {
    // Kill the current ffplay process
    let _ = child.kill();

    // Wait for it to terminate
    let _ = child.wait();

    // Build new ffplay command with the new position
    let mut cmd = Command::new("ffplay");
    cmd.args(&["-hide_banner", "-loglevel", "info"]);
    cmd.stderr(Stdio::inherit());
    cmd.stdout(Stdio::null());
    cmd.args(&["-ss", &position.to_string()]);
    cmd.arg("-autoexit");
    if (speed - 1.0).abs() > f32::EPSILON {
        cmd.args(&["-vf", &format!("setpts=PTS/{:.3}", speed)]);
        cmd.args(&["-af", &format!("atempo={:.3}", speed)]);
    }
    cmd.arg(video_path);

    // Start the new process
    *child = cmd.spawn().unwrap_or_else(|e| {
        eprintln!("Error restarting ffplay: {}", e);
        std::process::exit(1);
    });
}
