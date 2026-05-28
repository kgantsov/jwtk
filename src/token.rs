use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn encode_token(secret: &str, expire: u64, payload: &str) -> Result<()> {
    let mut claims: BTreeMap<String, Value> = BTreeMap::new();

    let payload: Value = serde_json::from_str(payload)?;
    for (claim, value) in payload
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("payload must be a JSON object"))?
    {
        claims.insert(claim.clone(), value.clone());
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    claims.insert(
        "exp".to_string(),
        Value::Number(serde_json::Number::from(now + expire)),
    );
    println!("Expire in {} seconds (exp: {})", expire, now + expire);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    println!("Generated JWT token: {}", token);

    Ok(())
}

pub fn decode_token(secret: Option<&str>, token: &str) -> Result<()> {
    if let Some(secret) = secret {
        let mut validation = Validation::default();
        validation.leeway = 0;
        let token_data: TokenData<BTreeMap<String, Value>> = decode(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;
        print_token(&token_data.header, &token_data.claims, true);
    } else {
        let token_data: TokenData<BTreeMap<String, Value>> =
            jsonwebtoken::dangerous::insecure_decode(token)?;
        print_token(&token_data.header, &token_data.claims, false);
    }

    Ok(())
}

fn print_token(header: &Header, claims: &BTreeMap<String, Value>, validated: bool) {
    println!("\nHeaders:");
    println!(" {:.<25}: {:?}", "Validated", validated);
    println!(" {:.<25}: {:?}", "Algorithm", header.alg);

    claims.get(&"exp".to_string()).and_then(|exp| {
        if let Some(exp) = exp.as_u64() {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
            let remaining = exp.saturating_sub(now);

            // exp to datetime
            let exp = chrono::DateTime::from_timestamp(exp as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| exp.to_string());
            println!(
                " {:.<25}: {} ({} seconds remaining)",
                "Expiration (exp)", exp, remaining
            );
        }
        Some(())
    });

    println!("\nPayload:");
    for (claim, value) in claims {
        println!(" {:.<25}: {}", claim, value);
    }
}
