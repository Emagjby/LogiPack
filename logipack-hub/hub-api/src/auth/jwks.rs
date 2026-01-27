use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use jsonwebtoken::DecodingKey;

#[derive(Debug, Clone, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub kty: String,
    pub use_: Option<String>,
    pub alg: Option<String>,

    // RSA comps
    pub n: Option<String>,
    pub e: Option<String>,
}

impl Jwk {
    pub fn to_decoding_key(&self) -> anyhow::Result<DecodingKey> {
        if self.kty != "RSA" {
            anyhow::bail!("unsupported kty {}", self.kty);
        }
        let n = self
            .n
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("missing n for RSA key"))?;
        let e = self
            .e
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("missing e for RSA key"))?;

        Ok(DecodingKey::from_rsa_components(n, e)?)
    }
}

// Cache

#[derive(Debug)]
struct Cache {
    by_kid: HashMap<String, DecodingKey>,
    fetched_at: Option<Instant>,
}

static KEY_CACHE: Lazy<Arc<RwLock<Cache>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Cache {
        by_kid: HashMap::new(),
        fetched_at: None,
    }))
});

pub fn get_cached_key(kid: &str) -> Option<DecodingKey> {
    let guard = KEY_CACHE.read().ok()?;
    guard.by_kid.get(kid).cloned()
}

pub fn cache_keys(jwks: &Jwks) -> anyhow::Result<()> {
    let mut guard = KEY_CACHE
        .write()
        .map_err(|_| anyhow::anyhow!("jwks cache poisoned"))?;

    for k in &jwks.keys {
        if k.kty == "RSA" {
            let dk = k.to_decoding_key()?;
            guard.by_kid.insert(k.kid.clone(), dk);
        }
    }

    guard.fetched_at = Some(Instant::now());
    Ok(())
}

pub fn cache_is_fresh(ttl: Duration) -> bool {
    let guard = match KEY_CACHE.read() {
        Ok(g) => g,
        Err(_) => return false,
    };

    match guard.fetched_at {
        Some(t) => t.elapsed() < ttl,
        None => false,
    }
}

pub async fn load_jwks_from_url(url: &str) -> anyhow::Result<Jwks> {
    let jwks = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json::<Jwks>()
        .await?;
    Ok(jwks)
}

pub fn load_jwks_from_json(raw: &str) -> anyhow::Result<Jwks> {
    // Accept both JWKS ({"keys": [...]}) and a single JWK ({...}) for local fixtures.
    // Keeps the public struct as-is.
    let value: serde_json::Value = serde_json::from_str(raw)?;

    if value.get("keys").is_some() {
        Ok(serde_json::from_value::<Jwks>(value)?)
    } else {
        let jwk: Jwk = serde_json::from_value(value)?;
        Ok(Jwks { keys: vec![jwk] })
    }
}

pub fn load_jwks_from_path(path: &str) -> anyhow::Result<Jwks> {
    let raw = std::fs::read_to_string(path)?;
    load_jwks_from_json(&raw)
}
