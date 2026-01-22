use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    dto::shipments::{
        ChangeStatusRequest, CreateShipmentRequest, CreateShipmentResponse, ShipmentDetail,
        ShipmentListItem, TimelineItem,
    },
    error::ApiError,
    state::AppState,
};

use core_application::{
    actor::ActorContext,
    shipments::{
        change_status::{ChangeStatus, change_status},
        create::{CreateShipment, create_shipment},
        get as shipments_get, list as shipments_list,
        timeline::read_timeline,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_shipments))
        .route("/:id", get(get_shipment))
        .route("/", post(create_shipment_handler))
        .route("/:id/status", post(change_status_handler))
        .route("/:id/timeline", get(get_timeline_handler))
}

/// List all shipments
async fn list_shipments(
    State(state): State<AppState>,
) -> Result<Json<Vec<ShipmentListItem>>, ApiError> {
    let rows = shipments_list::list_shipments(&state.db).await?;
    let result = rows.into_iter().map(ShipmentListItem::from).collect();
    Ok(Json(result))
}

/// Get shipment by id
async fn get_shipment(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ShipmentDetail>, ApiError> {
    let shipment_id = id
        .parse()
        .map_err(|_e| ApiError::bad_request("invalid_shipment_id", "Invalid shipment id"))?;
    let row = shipments_get::get_shipment(&state.db, shipment_id).await?;
    let result = ShipmentDetail::from(row);
    Ok(Json(result))
}

async fn create_shipment_handler(
    State(state): State<AppState>,
    actor: ActorContext,
    Json(req): Json<CreateShipmentRequest>,
) -> Result<Json<CreateShipmentResponse>, ApiError> {
    let id = create_shipment(
        &state.db,
        &actor,
        CreateShipment {
            client_id: req.client_id,
            current_office_id: req.current_office_id,
            notes: req.notes,
        },
    )
    .await?;

    Ok(Json(CreateShipmentResponse { shipment_id: id }))
}

async fn change_status_handler(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    actor: ActorContext,
    Json(req): Json<ChangeStatusRequest>,
) -> Result<(), ApiError> {
    change_status(
        &state.db,
        &actor,
        ChangeStatus {
            shipment_id: id,
            to_status: req.to_status,
            to_office_id: req.to_office_id,
            notes: req.notes,
        },
    )
    .await?;

    Ok(())
}

async fn get_timeline_handler(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TimelineItem>>, ApiError> {
    let rows = read_timeline(&state.db, id).await?;
    let result = rows.into_iter().map(TimelineItem::from).collect();

    Ok(Json(result))
}
