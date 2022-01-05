// #![cfg(feature = "integration")]
use flight_fusion_client::{
    arrow::datatypes::{DataType, Field, Schema},
    FlightFusionClient,
};
use flight_fusion_ipc::{flight_do_put_request, FlightDoPutRequest, PutMemoryTableRequest};
use std::sync::Arc;

mod setup;

#[tokio::test]
async fn do_put_memory_table() {
    let schema = Arc::new(Schema::new(vec![
        Field::new("col1", DataType::Float64, true),
        Field::new("col2", DataType::Float64, true),
        Field::new("col3", DataType::Float64, true),
    ]));
    let row_count = 4;
    let batch = setup::generate_random_batch(row_count, schema.clone());

    let ticket = FlightDoPutRequest {
        operation: Some(flight_do_put_request::Operation::Memory(
            PutMemoryTableRequest {
                name: "test_table".into(),
            },
        )),
    };
    let client = FlightFusionClient::try_new().await.unwrap();
    let response = client.register_batches(vec![batch], ticket).await.unwrap();

    println!("{:?}", response)
}
