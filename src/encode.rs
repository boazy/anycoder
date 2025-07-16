use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum EncodeFormat {
    Base64,
    Base64Url,
    Hex,
}

impl EncodeFormat {
    pub fn encode(&self, input: &[u8]) -> eyre::Result<Vec<u8>> {
        Ok(match self {
            EncodeFormat::Base64 =>
                BASE64_STANDARD.encode(input).into_bytes(),
            EncodeFormat::Base64Url =>
                BASE64_URL_SAFE_NO_PAD.encode(input).into_bytes(),
            EncodeFormat::Hex =>
                hex::encode(input).into_bytes(),
        })
    }
}
