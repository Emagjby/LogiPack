use crate::{routes::admin_ep::clients, state::AppState};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().nest("/clients", clients::router())
}
