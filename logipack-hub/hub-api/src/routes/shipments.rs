use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    dto::shipments::{ShipmentDetail, ShipmentListItem},
    error::ApiError,
    state::AppState,
};

use core_application::shipments::{get as shipments_get, list as shipments_list};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_shipments))
        .route("/:id", get(get_shipment))
}

async fn list_shipments(
    State(state): State<AppState>,
) -> Result<Json<Vec<ShipmentListItem>>, ApiError> {
    let rows = shipments_list::list_shipments(&state.db).await?;
    let result = rows.into_iter().map(ShipmentListItem::from).collect();
    Ok(Json(result))
}

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
