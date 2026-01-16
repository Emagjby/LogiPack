use hub_api::{app, config::Config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    let cfg = Config::from_env();

    let listener = tokio::net::TcpListener::bind(cfg.bind_addr())
        .await
        .expect("bind hub-api listener");

    tracing::info!(addr = %listener.local_addr().unwrap(), "hub-api listening");

    axum::serve(listener, app::router(cfg))
        .await
        .expect("serve hub-api");
}
