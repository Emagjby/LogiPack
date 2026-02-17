use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDto {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmployeeRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmployeeResponse {
    pub employee_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListEmployeesResponse {
    pub employees: Vec<EmployeeDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEmployeeResponse {
    pub employee: EmployeeDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmployeeRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmployeeResponse {
    pub employee_id: String,
}
