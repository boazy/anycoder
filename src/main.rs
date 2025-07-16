use std::io::{Read, stdin, stdout, Write};
use clap::{Args, Parser};
use eyre::bail;
use decode::DecodeFormat;
use encode::EncodeFormat;

mod encode;
mod decode;

#[derive(Parser)]
struct Options {
    #[command(flatten)]
    action: ActionArgs,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct ActionArgs {
    #[clap(short, long, value_name = "FORMAT")]
    decode: Option<DecodeFormat>,

    #[clap(short, long, value_name = "FORMAT")]
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