use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub role: String,
}
