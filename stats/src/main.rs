mod api_handlers;
mod apierror;
mod businesserror;
mod stats;

use appconfig::appstate::AppState;
use axum::{Router, routing::get};
use error_stack::Report;
use error_stack::ResultExt;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use thiserror::Error;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

#[derive(Debug, Error)]
enum AppError {
    #[error("application error")]
    Error,

    #[error("axum error")]
    AxumError,

    #[error("network error")]
    NetworkError,
}

#[tokio::main]
async fn main() -> Result<(), Report<AppError>> {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        // .with_tonic()
        .build()
        .unwrap();

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .build();

    let tracer = provider.tracer("reverse_market");

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(ErrorLayer::default())
        .with(EnvFilter::from_default_env())
        //.with(fmt::layer().pretty())
        .with(telemetry_layer)
        .init();

    let config = appconfig::config::read().change_context(AppError::Error)?;
    let state = AppState::new(&config)
        .await
        .change_context(AppError::Error)?;

    let app = Router::new()
        .route("/stats/buy_price", get(api_handlers::buy_price_get_handler))
        .route(
            "/stats/sell_price",
            get(api_handlers::sell_price_get_handler),
        )
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .change_context(AppError::NetworkError)?;

    axum::serve(listener, app)
        .await
        .change_context(AppError::AxumError)?;

    Ok(())
}
