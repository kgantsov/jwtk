use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Proportional resize to fit within max dimensions, no canvas padding
    Encode {
        /// JWT payload to encode (e.g., a JSON string)
        #[arg(long)]
        payload: String,
        /// Secret key for encoding (must be the same as the one used for decoding)
        #[arg(long)]
        secret: String,
    },
    /// Smart Instagram sizing: 1080×1080 for landscape, 1080×1350 for portrait
    Decode {
        /// JWT token to decode
        #[arg(long)]
        token: String,
        /// Optional secret key for decoding (must be the same as the one used for encoding)
        /// If not provided, the tool will decode the payload without verifying the signature, which may be less secure.
        #[arg(long)]
        secret: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(
    name = "jwtk",
    version,
    about = "A simple tool for encoding and decoding JWT tokens"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}
