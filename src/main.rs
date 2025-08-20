use std::io::{Read, stdin, stdout, Write};
use clap::{Args, Parser};
use eyre::bail;
use decode::DecodeFormat;
use encode::EncodeFormat;

mod encode;
mod decode;

#[derive(Parser)]
#[command(
    name = "anycoder",
    about = "Encode and decode data between different formats",
    long_about = "A command-line utility for encoding and decoding data between base64, base64url, and hex formats.\nReads from stdin and writes to stdout, making it perfect for shell pipelines.",
    after_help = "EXAMPLES:\n    echo 'Hello, World!' | anycoder --encode base64\n    echo 'SGVsbG8sIFdvcmxkIQ==' | anycoder --decode base64\n    echo 'deadbeef' | anycoder --decode hex | anycoder --encode base64url"
)]
struct Options {
    #[command(flatten)]
    action: ActionArgs,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct ActionArgs {
    #[clap(
        short, 
        long, 
        value_name = "FORMAT", 
        help = "Decode input from the specified format",
        long_help = "Decode input data from the specified format. Reads from stdin and outputs the decoded bytes to stdout."
    )]
    decode: Option<DecodeFormat>,

    #[clap(
        short, 
        long, 
        value_name = "FORMAT", 
        help = "Encode input to the specified format",
        long_help = "Encode input data to the specified format. Reads from stdin and outputs the encoded string to stdout."
    )]
    encode: Option<EncodeFormat>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let options = Options::parse();
    let action = &options.action;
    let output = if let Some(format) = action.encode {
        format.encode(read_input()?.as_slice())?
    } else if let Some(format) = action.decode {
        format.decode(read_input()?.as_slice())?
    } else {
        bail!("No action specified")
    };

    stdout().write(&output)?;

    Ok(())
}

fn read_input() -> eyre::Result<Vec<u8>> {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;
    Ok(input)
}