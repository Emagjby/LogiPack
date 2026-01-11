#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HUB_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("HUB_API_PORT")
            .ok()
            .and_then(|raw| raw.parse::<u16>().ok())
            .unwrap_or(3000);

        Self { host, port }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
