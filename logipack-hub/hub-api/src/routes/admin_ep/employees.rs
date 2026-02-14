use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use core_application::actor::ActorContext;

use crate::{
    dto::employees::{
        CreateEmployeeRequest, CreateEmployeeResponse, EmployeeDto, GetEmployeeResponse,
        ListEmployeesResponse, UpdateEmployeeRequest, UpdateEmployeeResponse,
    },
    error::ApiError,
    policy,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_employees_handler))
        .route("/:id", get(get_employee_handler))
        .route("/", post(create_employee_handler))
        .route("/:id", put(update_employee_handler))
        .route("/:id", delete(delete_employee_handler))
}

async fn list_employees_handler(
    State(state): State<AppState>,
    actor: ActorContext,
) -> Result<Json<ListEmployeesResponse>, ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let out = core_application::employees::list::list_employees(&state.db, &actor)
        .await
        .map_err(|e| match e {
            core_application::employees::list::ListEmployeesError::Forbidden => {
                ApiError::forbidden("access_denied", "Access denied")
            }
            core_application::employees::list::ListEmployeesError::EmployeeError(err) => {
                ApiError::internal(err.to_string())
            }
        })?;

    let dtos: Vec<EmployeeDto> = out
        .into_iter()
        .map(|employee| EmployeeDto {
            id: employee.id.to_string(),
            user_id: employee.user_id.to_string(),
            full_name: employee.full_name,
        })
        .collect();

    let result = ListEmployeesResponse { employees: dtos };

    Ok(Json(result))
}

async fn get_employee_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Path(id): Path<String>,
) -> Result<Json<GetEmployeeResponse>, ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let employee_uuid = id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_employee_id", "Employee ID must be a valid UUID")
    })?;

    let out = core_application::employees::get::get_employee(&state.db, &actor, employee_uuid)
        .await
        .map_err(|e| match e {
            core_application::employees::get::GetEmployeeError::NotFound => {
                ApiError::not_found("employee_not_found", "Employee not found")
            }
            core_application::employees::get::GetEmployeeError::Forbidden => {
                ApiError::forbidden("access_denied", "Access denied")
            }
            core_application::employees::get::GetEmployeeError::EmployeeError(err) => {
                ApiError::internal(err.to_string())
            }
        })?;

    let employee =
        out.ok_or_else(|| ApiError::not_found("employee_not_found", "Employee not found"))?;
    let result = GetEmployeeResponse {
        employee: EmployeeDto {
            id: employee.id.to_string(),
            user_id: employee.user_id.to_string(),
            full_name: employee.full_name,
        },
    };

    Ok(Json(result))
}

async fn create_employee_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Json(request): Json<CreateEmployeeRequest>,
) -> Result<(axum::http::StatusCode, Json<CreateEmployeeResponse>), ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let user_id = request
        .user_id
        .parse::<uuid::Uuid>()
        .map_err(|_| ApiError::bad_request("invalid_user_id", "User ID must be a valid UUID"))?;

    let input = core_application::employees::create::CreateEmployee {
        user_id,
        full_name: request.full_name,
    };

    let employee_id =
        core_application::employees::create::create_employee(&state.db, &actor, input)
            .await
            .map_err(|e| match e {
                core_application::employees::create::CreateEmployeeError::Forbidden => {
                    ApiError::forbidden("access_denied", "Access denied")
                }
                core_application::employees::create::CreateEmployeeError::Validation(err) => {
                    ApiError::bad_request("invalid_employee", err.to_string())
                }
                core_application::employees::create::CreateEmployeeError::EmployeeCreationError(
                    err,
                ) => match err {
                    core_data::repository::employees_repo::EmployeeError::EmployeeDbError(
                        db_err,
                    ) => db_err.into(),
                    core_data::repository::employees_repo::EmployeeError::RecordNotFound => {
                        ApiError::internal(err.to_string())
                    }
                },
            })?;

    let result = CreateEmployeeResponse {
        employee_id: employee_id.to_string(),
    };

    Ok((axum::http::StatusCode::CREATED, Json(result)))
}

async fn update_employee_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateEmployeeRequest>,
) -> Result<Json<UpdateEmployeeResponse>, ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let employee_uuid = id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_employee_id", "Employee ID must be a valid UUID")
    })?;

    let input = core_application::employees::update::UpdateEmployee {
        id: employee_uuid,
        full_name: request.full_name,
    };

    let out = core_application::employees::update::update_employee(&state.db, &actor, input)
        .await
        .map_err(|e| match e {
            core_application::employees::update::UpdateEmployeeError::NotFound => {
                ApiError::not_found("employee_not_found", "Employee not found")
            }
            core_application::employees::update::UpdateEmployeeError::Forbidden => {
                ApiError::forbidden("access_denied", "Access denied")
            }
            core_application::employees::update::UpdateEmployeeError::Validation(err) => {
                ApiError::bad_request("invalid_employee", err.to_string())
            }
            core_application::employees::update::UpdateEmployeeError::EmployeeError(err) => {
                ApiError::internal(err.to_string())
            }
        })?;

    let result = UpdateEmployeeResponse {
        employee_id: out.to_string(),
    };

    Ok(Json(result))
}

async fn delete_employee_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Path(id): Path<String>,
) -> Result<axum::http::StatusCode, ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let employee_uuid = id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_employee_id", "Employee ID must be a valid UUID")
    })?;

    core_application::employees::delete::delete_employee(&state.db, &actor, employee_uuid)
        .await
        .map_err(|e| match e {
            core_application::employees::delete::DeleteEmployeeError::NotFound => {
                ApiError::not_found("employee_not_found", "Employee not found")
            }
            core_application::employees::delete::DeleteEmployeeError::Forbidden => {
                ApiError::forbidden("access_denied", "Access denied")
            }
            core_application::employees::delete::DeleteEmployeeError::EmployeeError(err) => {
                ApiError::internal(err.to_string())
            }
        })?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}
