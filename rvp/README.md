# rvp

A simple CLI video player written in Rust that wraps `ffplay` (from FFmpeg).

## Features
- Play video with automatic exit
- Start playback at a given timestamp
- Adjust playback speed (slow down or speed up)

## Prerequisites
- [`ffplay`](https://ffmpeg.org/) must be installed and available in your PATH.

## Installation
Build and install the `rvp` binary locally:
```
cargo install --path .
```
This will put `rvp` into your Cargo bin directory (usually `~/.cargo/bin`).

## Usage
```
rvp [OPTIONS] <video_path>

Arguments:
  <video_path>         Path to the video file to play

Options:
  -r, --speed <f32>    Playback speed multiplier (default: 1.0)
  -s, --start <f32>    Start playback at given offset in seconds (default: 0.0)
  -h, --help           Print help information
  -V, --version        Print version information
```

Example:
```
# Play movie.mp4 starting at 10s, at 1.5x speed
rvp --start 10 --speed 1.5 movie.mp4
```