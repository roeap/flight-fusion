use open_metadata::prelude::*;
use std::result::Result;

/// This example demonstrates how to use the DataFrame API against in-memory data.
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let client = OpenMetadataClient::new("http://localhost:8585", OpenMetadataOptions::default());

    let mut databases = client
        .into_databases_collection_client()
        .list_databases()
        .limit(QueryLimit::try_from(1).unwrap())
        .into_stream();

    while let Some(Ok(chunk)) = databases.next().await {
        println!("Chunk --> {:?}", chunk);
    }

    let mut tables = client
        .into_tables_collection_client()
        .list_tables()
        .into_stream();

    while let Some(Ok(chunk)) = tables.next().await {
        println!("Chunk --> {:?}", chunk.data.len());
    }

    Ok(())
}
