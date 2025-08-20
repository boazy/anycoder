# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AnyCoder is a command-line utility for encoding and decoding data between different formats (base64, base64url, hex). It reads from stdin and writes to stdout, making it suitable for shell pipelines.

## Development Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run` - Build and run the binary
- `cargo run -- --help` - Show CLI help
- `cargo run -- --encode base64` - Encode stdin to base64
- `cargo run -- --decode hex` - Decode hex from stdin

### Testing and Quality
- `cargo test` - Run all tests
- `cargo check` - Check code for errors without building
- `cargo clippy` - Run linter for code quality checks
- `cargo fmt` - Format code according to Rust standards

### Release Build
- `cargo build --release` - Build optimized release binary
- `cargo run --release` - Run optimized binary

## Code Architecture

The project follows a modular structure with clear separation of concerns:

### Core Modules
- **main.rs** (`src/main.rs`): CLI argument parsing using clap, coordinates encode/decode operations, handles stdin/stdout I/O
- **encode.rs** (`src/encode.rs`): Defines `EncodeFormat` enum and encoding implementations for base64, base64url, and hex
- **decode.rs** (`src/decode.rs`): Defines `DecodeFormat` enum and decoding implementations with proper error handling

### Dependencies
- **clap**: Command-line argument parsing with derive macros
- **color-eyre/eyre**: Enhanced error handling and reporting
- **base64**: Base64 encoding/decoding with multiple variants
- **hex**: Hexadecimal encoding/decoding

### Error Handling
The project uses `eyre::Result` throughout for consistent error handling. All decode operations include contextual error messages for better debugging.

### CLI Design
The tool uses mutually exclusive `--encode` and `--decode` flags, each accepting format arguments. This design ensures clear operation modes and prevents conflicting operations.