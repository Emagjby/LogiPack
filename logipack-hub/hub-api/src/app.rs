use crate::auth::middleware::auth0_jwt_middleware;
use crate::auth::middleware::Auth0Config;
use crate::config::AuthMode;
use crate::config::Config;
use crate::state::AppState;
use axum::{routing::get, Router};

use crate::routes;

pub fn router(cfg: Config, state: AppState) -> Router {
    let public_router = Router::new().route("/health", get(routes::health::get_health));

    let protected_router = Router::new().nest("/shipments", routes::shipments::router());

    let protected_router = match cfg.auth_mode {
        AuthMode::DevSecret => {
            let dev_secret = cfg.dev_secret.clone();
            protected_router.layer(axum::middleware::from_fn(move |req, next| {
                crate::dev_secret::dev_secret_middleware(req, next, dev_secret.clone())
            }))
        }
        AuthMode::Auth0 => {
            let auth_cfg = Auth0Config {
                issuer: cfg.auth0_issuer.clone().expect("AUTH0_ISSUER is required"),
                audience: cfg
                    .auth0_audience
                    .clone()
                    .expect("AUTH0_AUDIENCE is required"),
                jwks_url: cfg.auth0_jwks_url.clone(),
                local_jwks_path: cfg.auth0_jwks_path.clone(),
                local_jwks_json: None,
                jwks_cache_ttl: std::time::Duration::from_secs(60 * 10),
            };

            protected_router.layer(axum::middleware::from_fn(move |req, next| {
                auth0_jwt_middleware(req, next, auth_cfg.clone())
            }))
        }
    };

    public_router.merge(protected_router).with_state(state)
}
