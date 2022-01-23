mod area_store;
mod catalog;
pub mod error;
mod handlers;
mod service;
mod settings;
mod stream;
#[cfg(test)]
mod test_utils;

use arrow_flight::flight_service_server::FlightServiceServer;
use dotenv::dotenv;
use lazy_static::lazy_static;
use observability_deps::{
    opentelemetry::{global, runtime::Tokio, sdk::propagation::TraceContextPropagator},
    opentelemetry_jaeger,
    tracing::info,
    tracing_opentelemetry,
    tracing_subscriber::{self, layer::SubscriberExt, prelude::*},
};
use service::FlightFusionService;
use tonic::transport::Server;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

pub const MB_TO_BYTES: i64 = 1_048_576;

pub async fn start_server() -> crate::error::Result<()> {
    dotenv().ok();
    // install global collector configured based on RUST_LOG env var.
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("grpc-server")
        .install_batch(Tokio)
        .unwrap();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(CONFIG.log.level.clone()))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(stdout_log)
        .try_init()
        .unwrap();

    let addr = format!("{}:{}", CONFIG.server.url, CONFIG.server.port)
        .parse()
        .unwrap();

    let service = match CONFIG.service.storage.as_str() {
        "file" => Ok(FlightFusionService::new_default(
            CONFIG.service.local.area_root.clone(),
        )),
        "azure" => Ok(FlightFusionService::new_azure(
            &CONFIG.service.azure.account,
            &CONFIG.service.azure.key,
            &CONFIG.service.azure.file_system,
        )),
        _ => Err("unrecognized storage service"),
    }
    .unwrap();

    let svc = FlightServiceServer::new(service);
    info!(
        "Listening on {}:{} ({})",
        CONFIG.server.url, CONFIG.server.port, CONFIG.env
    );

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();

    global::shutdown_tracer_provider();

    Ok(())
}
