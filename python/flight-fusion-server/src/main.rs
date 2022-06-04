use artifact_store::{
    gen::mlflow_artifacts_service_server::MlflowArtifactsServiceServer, MlflowArtifacts,
};
use clap::Parser;
use dotenv::dotenv;
use flight_fusion::{FlightFusionService, FlightServiceServer, Server};
use observability_deps::{
    opentelemetry::{
        global, sdk::export::trace::stdout, sdk::propagation::TraceContextPropagator,
        sdk::trace::config, sdk::Resource, KeyValue,
    },
    tracing::info,
    tracing_opentelemetry,
    tracing_subscriber::{self, layer::SubscriberExt, prelude::*},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to a config file
    #[clap(short, long)]
    config_file: Option<String>,

    /// Host where the server should run
    #[clap(short, long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    /// Port where the server should run
    #[clap(short, long, parse(try_from_str), default_value_t = 50051)]
    port: usize,

    /// Host where the server should run
    #[clap(long, default_value_t = String::from("warn"))]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();

    // install global collector configured based on RUST_LOG env var.
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    global::set_text_map_propagator(TraceContextPropagator::new());

    let resource = Resource::new(vec![KeyValue::new("service.name", "flight-fusion")]);
    let config = config().with_resource(resource);
    // let tracer = opentelemetry_jaeger::new_pipeline()
    //     .with_service_name("grpc-server")
    //     .install_batch(Tokio)?;
    let tracer = stdout::new_pipeline()
        .with_pretty_print(true)
        .with_trace_config(config)
        .install_simple();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(args.log_level.clone()))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(stdout_log)
        .try_init()?;

    let addr = format!("{}:{}", args.host, args.port).parse()?;
    let service = FlightFusionService::new_default("./.fusion").unwrap();
    let service_artifacts = MlflowArtifacts::new_default("./.mlflow/mlruns").unwrap();

    let svc = FlightServiceServer::new(service);
    let artifact_svc = MlflowArtifactsServiceServer::new(service_artifacts);
    info!("Listening on {}:{}", args.host, args.port);

    Server::builder()
        // .add_service(health_service)
        .add_service(svc)
        .add_service(artifact_svc)
        .serve(addr)
        .await?;

    global::shutdown_tracer_provider();

    Ok(())
}
