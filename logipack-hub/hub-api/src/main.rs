use hub_api::{app, config::Config, migrate::migrate, state::AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    let cfg = Config::from_env();
    let db_url = std::env::var("LOGIPACK_DATABASE_URL").expect("LOGIPACK_DATABASE_URL must be set");
    let db = sea_orm::Database::connect(&db_url)
        .await
        .expect("connect hub-api database");
    migrate(&db).await;
    let state = AppState {
        db,
        auth_mode: cfg.auth_mode,
    };

    let listener = tokio::net::TcpListener::bind(cfg.bind_addr())
        .await
        .expect("bind hub-api listener");

    tracing::info!(addr = %listener.local_addr().unwrap(), "hub-api listening");

    axum::serve(listener, app::router(cfg, state))
        .await
        .expect("serve hub-api");
}
