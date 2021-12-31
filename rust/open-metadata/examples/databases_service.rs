use open_metadata::{
    generated::{DatabaseServiceType, JdbcInfo, StorageServiceType},
    prelude::*,
};
use std::result::Result;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let client = OpenMetadataClient::new("http://localhost:8585", OpenMetadataOptions::default());

    let mut tables = client.into_services_client().list_services().into_stream();
    while let Some(Ok(chunk)) = tables.next().await {
        println!("Chunk --> {:?}", chunk.data);
    }

    let generated = client
        .into_services_client()
        .create_database_service(
            "db_service2",
            DatabaseServiceType::Druid,
            JdbcInfo {
                connection_url: "asd:pwd@host:port".to_string(),
                driver_class: "jdbc".to_string(),
            },
        )
        .into_future()
        .await
        .unwrap();

    // let generated = client
    //     .into_services_client()
    //     .create_storage_service("storage_service2")
    //     .service_type(StorageServiceType::Abfs)
    //     .into_future()
    //     .await
    //     .unwrap();

    println!("{:?}", generated);

    Ok(())
}
