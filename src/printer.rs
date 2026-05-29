use anyhow::Result;
use colored::Colorize;
use jsonwebtoken::Header;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn print_header(header: &str) {
    println!("{}", header.bold());
}

pub fn print_kv(key: &str, value: &str) {
    println!(" {:.<25}: {}", key.cyan(), value);
}

pub fn print_expiration(exp: u64) -> Result<()> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let remaining = exp.saturating_sub(now);

    // exp to datetime
    let exp = chrono::DateTime::from_timestamp(exp as i64, 0)
        .map(|dt| {
            dt.with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        })
        .unwrap_or_else(|| exp.to_string());

    let remaining = time_remaining(remaining);
    println!(
        " {:.<25}: {} ({})",
        "Expiration (exp)".cyan(),
        exp,
        remaining
    );

    Ok(())
}

pub fn print_token(header: &Header, claims: &BTreeMap<String, Value>, validated: bool) {
    print_header("\nHeaders:");

    let validated = if validated { "Yes".green() } else { "No".red() };

    print_kv("Validated", validated.to_string().as_str());
    print_kv("Algorithm", format!("{:?}", header.alg).as_str());

    claims.get(&"exp".to_string()).and_then(|exp| {
        if let Some(exp) = exp.as_u64() {
            print_expiration(exp).ok()?;
        }
        Some(())
    });

    print_header("\nPayload:");
    for (claim, value) in claims {
        print_kv(claim, &value.to_string());
    }
}

fn time_remaining(remaining: u64) -> String {
    match remaining {
        0 => "expired".red().to_string(),
        1..=60 => format!("{} seconds remaining", remaining)
            .yellow()
            .to_string(),
        61..=3600 => format!("{} minutes remaining", remaining / 60).to_string(),
        3601..=86400 => format!("{} hours remaining", remaining / 3600).to_string(),
        _ => format!("{} days remaining", remaining / 86400).to_string(),
    }
}
