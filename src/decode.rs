use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use clap::ValueEnum;
use eyre::Context;

#[derive(Clone, Copy, Debug,ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum DecodeFormat {
    Base64,
    Base64Url,
    Hex,
}

impl DecodeFormat {
    pub fn decode(&self, input: &[u8]) -> eyre::Result<Vec<u8>> {
        match self {
            DecodeFormat::Base64 =>
                BASE64_STANDARD.decode(input)
                    .context("Failed to decode base64"),
            DecodeFormat::Base64Url =>
                BASE64_URL_SAFE_NO_PAD.decode(input)
                    .context("Failed to decode base64url"),
            DecodeFormat::Hex => hex::decode(input)
                .context("Failed to decode hex"),
        }
    }
}