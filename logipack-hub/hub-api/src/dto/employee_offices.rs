use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignOfficeRequest {
    pub office_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListEmployeeOfficesResponse {
    pub office_ids: Vec<String>,
}
