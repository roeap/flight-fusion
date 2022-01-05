#![cfg(feature = "integration")]
use flight_fusion_client::FlightFusionClient;

#[tokio::test]
async fn drop_table() {
    let client = FlightFusionClient::try_new().await.unwrap();
    let response = client.drop_table("table_name").await.unwrap();
    assert_eq!(response.name, "table_name")
}
