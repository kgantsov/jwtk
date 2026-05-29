use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use jsonwebtoken::Algorithm;
use jwtk::command::Cli;
use jwtk::token::{decode_token, encode_token};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.cmd {
        jwtk::command::Cmd::Encode {
            secret,
            payload,
            expire,
            alg: algorithm,
        } => {
            let mut secret_input = String::new();
            let secret = match secret {
                Some(t) if t != "-" => t.as_str(),
                _ => {
                    // Read token from stdin
                    println!(
                        "{}",
                        "Please enter the secret (press Enter when done):".dimmed()
                    );
                    std::io::stdin().read_line(&mut secret_input)?;
                    secret_input.trim()
                }
            };

            if let Err(e) = encode_token(*algorithm, secret, *expire, payload) {
                eprintln!("Error encoding JWT: {}", e);
            }
        }
        jwtk::command::Cmd::Decode { token, secret } => {
            let mut secret_input = String::new();
            let secret = match secret {
                None => None,
                Some(t) if t != "-" => Some(t.as_str()),
                _ => {
                    // Read token from stdin
                    println!(
                        "{}",
                        "Please enter the secret (press Enter when done):".dimmed()
                    );
                    std::io::stdin().read_line(&mut secret_input)?;
                    Some(secret_input.trim())
                }
            };

            let mut token_input = String::new();
            let token = match token {
                Some(t) if t != "-" => t.as_str(),
                _ => {
                    // Read token from stdin
                    println!(
                        "{}",
                        "Please enter the JWT token to decode (press Enter when done):".dimmed()
                    );
                    std::io::stdin().read_line(&mut token_input)?;
                    token_input.trim()
                }
            };

            if let Err(e) = decode_token(secret, token) {
                eprintln!("Error decoding JWT: {}", e);
            }
        }
    }

    Ok(())
}
