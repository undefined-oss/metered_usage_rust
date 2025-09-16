pub mod entities;
pub mod server;

use crate::server::{ServerState, events_handler};
use metered_usage::infrastructure::clickhouse_client::ClickhouseClient;
use std::env;
use std::sync::Arc;
use tokio::signal;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=TRACE,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing Clickhouse client");
    let client = ClickhouseClient::new();

    let server_state = Arc::new(ServerState {
        clickhouse_client_creator: client,
    });

    let api_url = get_server_url();

    let listener = tokio::net::TcpListener::bind(&api_url).await.unwrap();

    debug!("Listening server in the url {}", api_url);

    axum::serve(listener, events_handler(server_state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

fn get_server_url() -> String {
    let host = match env::var("HOST") {
        Ok(host) => host,
        Err(_) => "127.0.0.1".into(),
    };

    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8000".into(),
    };

    return format!("{host}:{port}");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
