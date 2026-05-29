use crate::printer::{print_expiration, print_header, print_kv, print_token};
use anyhow::Result;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn encode_token(algorithm: Algorithm, secret: &str, expire: u64, payload: &str) -> Result<()> {
    let mut claims: BTreeMap<String, Value> = BTreeMap::new();

    let payload: Value = serde_json::from_str(payload)?;
    for (claim, value) in payload
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("payload must be a JSON object"))?
    {
        claims.insert(claim.clone(), value.clone());
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let expire_in_sec = now + expire;
    claims.insert(
        "exp".to_string(),
        Value::Number(serde_json::Number::from(expire_in_sec)),
    );

    let header = Header::new(algorithm);
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))?;

    print_header("JWT:");
    print_kv("Algorithm", format!("{:?}", header.alg).as_str());
    print_kv("Payload", &payload.to_string());

    print_expiration(expire_in_sec)?;
    print_kv("Token", token.as_str());

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
