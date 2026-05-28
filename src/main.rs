use anyhow::Result;
use clap::Parser;
use jwtk::command::Cli;
use jwtk::token::{decode_token, encode_token};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.cmd {
        jwtk::command::Cmd::Encode {
            secret,
            payload,
            expire,
        } => {
            if let Err(e) = encode_token(secret, *expire, payload) {
                eprintln!("Error encoding JWT: {}", e);
            }
        }
        jwtk::command::Cmd::Decode { token, secret } => {
            if let Err(e) = decode_token(secret.as_deref(), token) {
                eprintln!("Error decoding JWT: {}", e);
            }
        }
    }

    Ok(())
}
