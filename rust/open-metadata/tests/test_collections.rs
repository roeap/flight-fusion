#![cfg(feature = "mock_transport_framework")]
use futures::StreamExt;
use std::{error::Error, assert};
use test_utils::initialize;

mod test_utils;

type BoxedError = Box<dyn Error + Send + Sync>;

#[tokio::test]
async fn test_list_collections() -> Result<(), BoxedError> {
    let client = initialize("list_collections");

    let collections = client
        .list_collections()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();

    assert!(collections.data.len() > 0);

    Ok(())
}
