# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust computer vision application that provides real-time camera capture and display functionality using OpenCV. The project is a simple command-line executable that demonstrates basic video processing capabilities.

## Development Commands

```bash
# Build the project
cargo build

# Run the application (requires camera access)
cargo run

# Build in release mode for better performance
cargo build --release

# Check code without building
cargo check

# Clean build artifacts
cargo clean
```

## Architecture

The application follows a minimal single-file architecture:

- **Entry Point**: `src/main.rs` contains the entire application logic
- **Camera Input**: Uses OpenCV's `VideoCapture` to access camera device 0
- **Display**: Creates a fullscreen window using OpenCV's highgui module
- **Event Loop**: Simple synchronous loop that captures, displays, and checks for keyboard input

## Key Dependencies

- `opencv` (v0.97.2) with `clang-runtime` feature for OpenCV bindings

## Important Implementation Details

1. **Camera Device**: Hardcoded to use device index 0 (first available camera)
2. **Exit Condition**: Application exits when 'q' key (ASCII 113) is pressed
3. **Window Mode**: Uses fullscreen display mode
4. **Error Handling**: Uses Rust's Result type for error propagation
5. **Frame Processing**: Currently just displays frames without any processing

## Common Patterns

- Direct OpenCV API usage without abstractions
- Synchronous frame processing in a loop
- Minimal state management
- RAII-based resource cleanup through Rust's ownership system