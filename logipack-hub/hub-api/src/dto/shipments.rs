use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentListItem {
    pub id: String,
    pub client_id: String,
    pub current_status: String,
    pub current_office_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentDetail {
    pub id: String,
    pub client: ClientDto,
    pub current_status: String,
    pub current_office: Option<OfficeDto>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDto {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfficeDto {
    pub id: String,
    pub name: String,
    pub city: String,
    pub address: String,
}

impl From<core_data::entity::shipments::Model> for ShipmentListItem {
    fn from(value: core_data::entity::shipments::Model) -> Self {
        Self {
            id: value.id.to_string(),
            client_id: value.client_id.to_string(),
            current_status: value.current_status,
            current_office_id: value.current_office_id.map(|id| id.to_string()),
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

impl From<core_data::entity::shipments::Model> for ShipmentDetail {
    fn from(value: core_data::entity::shipments::Model) -> Self {
        Self {
            id: value.id.to_string(),
            client: ClientDto {
                id: value.client_id.to_string(),
                name: "".to_string(),
                email: None,
                phone: None,
            },
            current_status: value.current_status,
            current_office: None,
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}
