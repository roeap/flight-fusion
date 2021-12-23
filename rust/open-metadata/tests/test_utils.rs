use open_metadata::clients::{OpenMetadataClient, OpenMetadataOptions};

#[cfg(not(feature = "mock_transport_framework"))]
pub fn initialize() -> OpenMetadataClient {
    OpenMetadataClient::new("http://localhost:8585", OpenMetadataOptions::default())
}

#[cfg(feature = "mock_transport_framework")]
pub fn initialize<T: Into<String>>(transaction_name: T) -> OpenMetadataClient {
    OpenMetadataClient::new_with_transaction("http://localhost:8585", transaction_name.into())
}
