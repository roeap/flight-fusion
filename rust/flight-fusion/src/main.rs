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
use tonic::transport::Server;

mod area_store;
mod error;
mod handlers;
mod service;
mod settings;
mod stream;
#[cfg(test)]
mod test_utils;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // install global collector configured based on RUST_LOG env var.
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("grpc-server")
        .install_batch(Tokio)?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(CONFIG.log.level.clone()))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(stdout_log)
        .try_init()?;

    let addr = format!("{}:{}", CONFIG.server.url, CONFIG.server.port).parse()?;
    let service = service::FlightFusionService::new_default(CONFIG.service.area_root.clone());

    let svc = FlightServiceServer::new(service);
    info!(
        "Listening on {}:{} ({})",
        CONFIG.server.url, CONFIG.server.port, CONFIG.env
    );

    Server::builder().add_service(svc).serve(addr).await?;

    global::shutdown_tracer_provider();

    Ok(())
}
