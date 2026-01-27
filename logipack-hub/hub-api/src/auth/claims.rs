use serde::Deserialize;

/// Minimal Auth0-style JWT claims
/// Add more later if needed
#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub sub: String,

    // Issuer
    pub iss: String,

    // Audience can be string or array of strings in JWTs
    #[serde(default)]
    pub aud: Audience,

    // Expiry
    pub exp: i64,

    // Not-before optional
    #[serde(default)]
    pub nbf: Option<i64>,

    // Issued at optional
    #[serde(default)]
    pub iat: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(untagged)]
pub enum Audience {
    One(String),
    Many(Vec<String>),
    #[default]
    None,
}

impl Audience {
    pub fn contains(&self, wanted: &str) -> bool {
        match self {
            Audience::One(a) => a == wanted,
            Audience::Many(aud_vec) => aud_vec.iter().any(|x| x == wanted),
            Audience::None => false,
        }
    }
}
