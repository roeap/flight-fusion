use arrow_flight::flight_service_server::FlightServiceServer;
use dotenv::dotenv;
use lazy_static::lazy_static;
use observability_deps::opentelemetry::{
    global, runtime::Tokio, sdk::propagation::TraceContextPropagator,
};
use observability_deps::opentelemetry_jaeger;
use observability_deps::tracing::info;
use observability_deps::tracing_opentelemetry;
use observability_deps::tracing_subscriber;
use observability_deps::tracing_subscriber::prelude::*;
use tonic::transport::Server;
use tracing_subscriber::layer::SubscriberExt;

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
    // tracing_subscriber::fmt::init();
    // let subscriber = tracing_subscriber::fmt
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("grpc-server")
        .install_batch(Tokio)?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(stdout_log)
        .try_init()?;

    let addr = "0.0.0.0:50051".parse()?;
    let area_root = "/home/robstar/github/flight-fusion/.tmp";
    let service = service::FlightFusionService::new_default(area_root);

    let svc = FlightServiceServer::new(service);
    info!("Listening on {:?} ({})", CONFIG.server.port, CONFIG.env);

    Server::builder().add_service(svc).serve(addr).await?;

    global::shutdown_tracer_provider();

    Ok(())
}
