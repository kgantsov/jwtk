use clap::Parser;
use clap::Subcommand;
use jsonwebtoken::Algorithm;

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Encode a payload as a signed JWT
    Encode {
        /// JSON payload to encode (e.g. '{"sub":"1234","name":"Alice"}')
        #[arg(short, long)]
        payload: String,
        /// HMAC secret key. Omit the value (or pass `-`) to read from stdin
        #[arg(short, long, num_args = 0..=1, default_missing_value = "-")]
        secret: Option<String>,

        /// Token lifetime in seconds from now (default: 3600). The `exp` claim is added automatically
        #[arg(short, long, default_value_t = 3600)]
        expire: u64,

        /// Algorithm to use for signing (default: HS256). For example: HS256, HS384, HS512, RS256, RS384, RS512
        #[arg(short, long, default_value = "HS256")]
        alg: Algorithm,
    },
    /// Decode a JWT and optionally verify its signature
    Decode {
        /// JWT token to decode. Omit the value (or pass `-`) to read from stdin
        #[arg(short, long, default_missing_value = "-")]
        token: Option<String>,
        /// HMAC secret for signature verification. Omit to skip verification.
        /// Omit the value (or pass `-`) to read from stdin
        #[arg(short, long, num_args = 0..=1, default_missing_value = "-")]
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
