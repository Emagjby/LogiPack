use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDto {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClientResponse {
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListClientsResponse {
    pub clients: Vec<ClientDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClientResponse {
    pub client: ClientDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClientRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClientResponse {
    pub client_id: String,
}
