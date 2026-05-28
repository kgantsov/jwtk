use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::Token;
use jwt::VerifyWithKey;
use serde_json::Value;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn encode_token(secret: &str, payload: &str) -> Result<()> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let mut claims = BTreeMap::new();

    let payload: Value = serde_json::from_str(payload)?;
    for (claim, value) in payload
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("payload must be a JSON object"))?
    {
        claims.insert(claim, value);
    }

    let token_str = claims.sign_with_key(&key)?;
    println!("Generated JWT token: {}", token_str);

    Ok(())
}

pub fn decode_token(secret: Option<&str>, token: &str) -> Result<()> {
    if let Some(secret) = secret {
        let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
        let token: Token<jwt::Header, BTreeMap<String, Value>, _> = token.verify_with_key(&key)?;
        print_token(token, true);
    } else {
        let token: Token<jwt::Header, BTreeMap<String, Value>, _> = Token::parse_unverified(token)?;
        print_token(token, false);
    }

    Ok(())
}

fn print_token<S>(token: Token<jwt::Header, BTreeMap<String, Value>, S>, validated: bool) {
    println!("\nHeaders:");
    println!(" {:.<25}: {:?}", "Validated", validated);
    println!(" {:.<25}: {:?}", "Algorithm", token.header().algorithm);

    println!("\nPayload:");
    for (claim, value) in token.claims() {
        println!(" {:.<25}: {}", claim, value);
    }
}
