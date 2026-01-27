#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMode {
    DevSecret,
    Auth0,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,

    // dev-secret mode
    pub dev_secret: String,

    // auth
    pub auth_mode: AuthMode,
    pub auth0_issuer: Option<String>,
    pub auth0_audience: Option<String>,
    pub auth0_jwks_url: Option<String>,
    pub auth0_jwks_path: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HUB_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = std::env::var("HUB_API_PORT")
            .ok()
            .and_then(|raw| raw.parse::<u16>().ok())
            .unwrap_or(3000);

        let auth_mode = match std::env::var("LOGIPACK_AUTH_MODE").as_deref() {
            Ok("auth0") => AuthMode::Auth0,
            _ => AuthMode::DevSecret,
        };

        let dev_secret = std::env::var("LOGIPACK_DEV_SECRET").unwrap_or_else(|_| "dev".to_string());

        let auth0_issuer = std::env::var("AUTH0_ISSUER").ok();
        let auth0_audience = std::env::var("AUTH0_AUDIENCE").ok();
        let auth0_jwks_url = std::env::var("AUTH0_JWKS_URL").ok();
        let auth0_jwks_path = std::env::var("AUTH0_JWKS_PATH").ok();

        Self {
            host,
            port,
            dev_secret,
            auth_mode,
            auth0_issuer,
            auth0_audience,
            auth0_jwks_url,
            auth0_jwks_path,
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
