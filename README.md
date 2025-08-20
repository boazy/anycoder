# AnyCoder

A fast, lightweight command-line utility for encoding and decoding data between different formats. AnyCoder supports
base64, base64url, and hexadecimal formats, making it perfect for shell pipelines and automation scripts.

## Features

- **Multiple formats**: Supports base64, base64url, and hex encoding/decoding
- **Pipeline-friendly**: Reads from stdin and writes to stdout
- **Fast**: Written in Rust for optimal performance
- **Simple**: Clean, intuitive command-line interface
- **Error handling**: Clear error messages with context

## Installation

### From Source

```bash
git clone https://github.com/boazyaniv/anycoder.git
cd anycoder
cargo build --release
```

The binary will be available at `target/release/anycoder`.

## Usage

AnyCoder uses mutually exclusive `--encode` and `--decode` flags:

```bash
anycoder --encode <FORMAT>
anycoder --decode <FORMAT>
```

### Supported Formats

- `base64` - Standard Base64 encoding (RFC 4648)
- `base64url` - URL-safe Base64 encoding without padding
- `hex` - Hexadecimal encoding

### Examples

#### Basic Encoding

```bash
echo "Hello, World!" | anycoder --encode base64
# Output: SGVsbG8sIFdvcmxkIQo=

echo "Hello, World!" | anycoder --encode hex
# Output: 48656c6c6f2c20576f726c64210a
```

#### Basic Decoding

```bash
echo "SGVsbG8sIFdvcmxkIQo=" | anycoder --decode base64
# Output: Hello, World!

echo "48656c6c6f2c20576f726c64210a" | anycoder --decode hex
# Output: Hello, World!
```

#### Chaining Operations

```bash
# Convert hex to base64
echo "deadbeef" | anycoder --decode hex | anycoder --encode base64
# Output: 3q2+7w==

# Convert base64 to base64url
echo "SGVsbG8sIFdvcmxkIQ==" | anycoder --decode base64 | anycoder --encode base64url
# Output: SGVsbG8sIFdvcmxkIQ
```

#### Working with Files
```bash
# Encode a file
cat file.txt | anycoder --encode base64 > encoded.txt

# Decode a file
cat encoded.txt | anycoder --decode base64 > decoded.txt
```

## Help

Get help with the `--help` flag:

```bash
anycoder --help
```

## Development

### Prerequisites

- Rust 1.70 or later

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
cargo clippy
cargo fmt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
