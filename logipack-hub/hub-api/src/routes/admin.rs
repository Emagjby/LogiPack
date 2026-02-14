use crate::{
    routes::admin_ep::{clients, employees, offices},
    state::AppState,
};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/clients", clients::router())
        .nest("/employees", employees::router())
        .nest("/offices", offices::router())
}
