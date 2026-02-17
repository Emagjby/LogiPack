use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsureUserRequest {
    pub name: String,
    pub email: String,
}
