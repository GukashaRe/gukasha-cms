use anyhow::{Result, anyhow};
use chrono::{DateTime, Duration, Utc};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

type VerifyMap = Lazy<Mutex<HashMap<String, (String, DateTime<Utc>)>>>;
// HashMap<email,(verify_code,generate_timestamp)>
static VERIFY_MAP: VerifyMap = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn add_verify_code(email: &str, code: &str) -> Result<()> {
    let mut map = VERIFY_MAP
        .lock()
        .map_err(|e| anyhow!("verify:Get Verify mapLock error {}", e))?;
    let expire = Utc::now() + Duration::seconds(300);
    map.insert(email.to_string(), (code.to_string(), expire));
    Ok(())
}

pub fn verify_code(email: &str, code: &str) -> Result<bool> {
    let mut map = VERIFY_MAP
        .lock()
        .map_err(|e| anyhow!("verify: Get Verify map lock error {}", e))?;
    let entry = map.get(email).cloned();
    if let Some((cached_code, expiry)) = entry {
        let success = cached_code == code && Utc::now() < expiry;
        map.remove(email);
        if success {
            return Ok(true);
        }
    }
    Ok(false)
}
