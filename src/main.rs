use anyhow::{Context as _, Result};
use axum::{routing::get, Router};
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::FULL)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with_target(true)
        .init();

    let router = Router::new()
        .route("/", get(|| async { "Hello, GCP App Engine!" }))
        .layer(TraceLayer::new_for_http());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .with_context(|| format!("failed to bind to {address}"))?;

    tracing::info!("Server is running on {address} 🚀");

    axum::serve(listener, router.into_make_service())
        .await
        .context("axum::serve() failed")
}
