#[macro_use]
extern crate lazy_static;
use tracing::info;

use arrow_flight::flight_service_server::FlightServiceServer;

use dotenv::dotenv;
use tonic::transport::Server;

mod handlers;
mod object_store;
mod service;
mod settings;
#[cfg(test)]
mod test_utils;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "0.0.0.0:50051".parse()?;
    let service = service::FlightFusionService::new_default();

    let svc = FlightServiceServer::new(service);
    info!("Listening on {:?} ({})", CONFIG.server.port, CONFIG.env);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
