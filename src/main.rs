use crate::routes::build_routes;
use crate::state::VismutState;

mod database;
mod routes;
mod state;
mod config;
mod responses;
mod requests;

#[tokio::main]
async fn main() -> () {
    tracing_subscriber::fmt().compact().init();

    let state = VismutState::new().await;
    let listener = tokio::net::TcpListener::bind(state.get_env().get_host()).await.unwrap();
    tracing::info!("Vismut Web Editor is listening on {}", state.get_env().get_host());

    let app = build_routes().with_state(state);
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::unix::signal(unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("received SIGTERM, graceful shutdown...");
        },
        _ = terminate => {
            tracing::info!("received SIGTERM, graceful shutdown...");
        },
    }
}
