use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use core_application::actor::ActorContext;

use crate::{
    dto::employee_offices::{AssignOfficeRequest, ListEmployeeOfficesResponse},
    error::ApiError,
    policy,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_employee_offices_handler))
        .route("/", post(assign_office_handler))
        .route("/:officeId", delete(remove_office_handler))
}

async fn assign_office_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Path(employee_id): Path<String>,
    Json(request): Json<AssignOfficeRequest>,
) -> Result<axum::http::StatusCode, ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let employee_uuid = employee_id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_employee_id", "Employee ID must be a valid UUID")
    })?;

    let office_uuid = request.office_id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_office_id", "Office ID must be a valid UUID")
    })?;

    let input = core_application::employee_offices::assign::AssignOffice {
        employee_id: employee_uuid,
        office_id: office_uuid,
    };

    core_application::employee_offices::assign::assign_office(&state.db, &actor, input)
        .await
        .map_err(|e| match e {
            core_application::employee_offices::assign::AssignOfficeError::Forbidden => {
                ApiError::forbidden("access_denied", "Access denied")
            }
            core_application::employee_offices::assign::AssignOfficeError::EmployeeNotFound => {
                ApiError::not_found("employee_not_found", "Employee not found")
            }
            core_application::employee_offices::assign::AssignOfficeError::OfficeNotFound => {
                ApiError::not_found("office_not_found", "Office not found")
            }
            core_application::employee_offices::assign::AssignOfficeError::AlreadyAssigned => {
                ApiError::conflict("assignment_exists", "Employee already assigned to office")
            }
            core_application::employee_offices::assign::AssignOfficeError::AssignError(err) => {
                ApiError::internal(err.to_string())
            }
        })?;

    Ok(axum::http::StatusCode::OK)
}

async fn remove_office_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Path((employee_id, office_id)): Path<(String, String)>,
) -> Result<axum::http::StatusCode, ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let employee_uuid = employee_id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_employee_id", "Employee ID must be a valid UUID")
    })?;

    let office_uuid = office_id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_office_id", "Office ID must be a valid UUID")
    })?;

    let input = core_application::employee_offices::remove::RemoveOffice {
        employee_id: employee_uuid,
        office_id: office_uuid,
    };

    core_application::employee_offices::remove::remove_office(&state.db, &actor, input)
        .await
        .map_err(|e| match e {
            core_application::employee_offices::remove::RemoveOfficeError::Forbidden => {
                ApiError::forbidden("access_denied", "Access denied")
            }
            core_application::employee_offices::remove::RemoveOfficeError::EmployeeNotFound => {
                ApiError::not_found("employee_not_found", "Employee not found")
            }
            core_application::employee_offices::remove::RemoveOfficeError::OfficeNotFound => {
                ApiError::not_found("office_not_found", "Office not found")
            }
            core_application::employee_offices::remove::RemoveOfficeError::RemoveError(err) => {
                ApiError::internal(err.to_string())
            }
        })?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn list_employee_offices_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Path(employee_id): Path<String>,
) -> Result<Json<ListEmployeeOfficesResponse>, ApiError> {
    policy::require_admin(&actor)
        .map_err(|_| ApiError::forbidden("access_denied", "Access denied"))?;

    let employee_uuid = employee_id.parse::<uuid::Uuid>().map_err(|_| {
        ApiError::bad_request("invalid_employee_id", "Employee ID must be a valid UUID")
    })?;

    let office_ids = core_application::employee_offices::list::list_employee_offices(
        &state.db,
        &actor,
        employee_uuid,
    )
    .await
    .map_err(|e| match e {
        core_application::employee_offices::list::ListEmployeeOfficesError::Forbidden => {
            ApiError::forbidden("access_denied", "Access denied")
        }
        core_application::employee_offices::list::ListEmployeeOfficesError::EmployeeNotFound => {
            ApiError::not_found("employee_not_found", "Employee not found")
        }
        core_application::employee_offices::list::ListEmployeeOfficesError::ListError(err) => {
            ApiError::internal(err.to_string())
        }
    })?;

    let response = ListEmployeeOfficesResponse {
        office_ids: office_ids.into_iter().map(|id| id.to_string()).collect(),
    };

    Ok(Json(response))
}
