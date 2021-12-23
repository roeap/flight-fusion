pub mod collections;
pub mod databases;

pub use databases::*;
pub use collections::*;

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_get_version() {
        let client = OpenMetadataClient::new(
            "http://localhost:8585",
            OpenMetadataOptions::new_with_transaction_name("get_version".to_string()),
        );
        let version = client.get_version().into_future().await.unwrap();
        println!("{:?}", version)
    }

    #[tokio::test]
    async fn test_list_collections() {
        let client = OpenMetadataClient::new(
            "http://localhost:8585",
            OpenMetadataOptions::new_with_transaction_name("list_collections".to_string()),
        );

        let collections = client.list_collections().into_stream()
            .next()
            .await
            .unwrap()
            .unwrap();

        println!("{:?}", collections);
    }
}
