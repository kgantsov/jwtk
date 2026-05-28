use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Proportional resize to fit within max dimensions, no canvas padding
    Encode {
        /// JWT payload to encode (e.g., a JSON string)
        #[arg(short, long)]
        payload: String,
        /// Secret key for encoding (must be the same as the one used for decoding)
        #[arg(short, long)]
        secret: String,

        /// Expiration time in seconds (optional, default is 3600 seconds or 1 hour)
        #[arg(short, long, default_value_t = 3600)]
        expire: u64,
    },
    /// Smart Instagram sizing: 1080×1080 for landscape, 1080×1350 for portrait
    Decode {
        /// JWT token to decode
        #[arg(short, long)]
        token: String,
        /// Optional secret key for decoding (must be the same as the one used for encoding)
        /// If not provided, the tool will decode the payload without verifying the signature, which may be less secure.
        #[arg(short, long)]
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
