use crate::{
    clients::{OpenMetadataClient, OpenMetadataOptions},
    generated::{CreateStorageServiceRequest, StorageServiceType},
};
use bytes::Bytes;
use reqwest_pipeline::{setters, Context};

type CreateStorageService = futures::future::BoxFuture<'static, crate::Result<()>>;

#[derive(Clone, Debug)]
pub struct ListServicesBuilder {
    client: OpenMetadataClient,
}

#[derive(Clone, Debug)]
pub struct CreateStorageServiceBuilder {
    client: OpenMetadataClient,
    context: Context,
    name: String,
    description: Option<String>,
    service_type: Option<StorageServiceType>,
}

// TODO add context type and context encoding headers...
impl CreateStorageServiceBuilder {
    pub(crate) fn new(client: OpenMetadataClient, name: String) -> Self {
        Self {
            client,
            context: Context::new(),
            name,
            description: None,
            service_type: None,
        }
    }

    setters! {
        description: String => Some(description),
        service_type: StorageServiceType => Some(service_type),
    }

    pub fn insert<E: Send + Sync + 'static>(&mut self, entity: E) -> &mut Self {
        self.context.insert(entity);
        self
    }

    pub fn into_future(self) -> CreateStorageService {
        let uri = self.client.api_routes().services().join("services/storageServices").unwrap();
        println!("{:?}", self.client.api_routes().services());
        println!("{:?}", self.client.api_routes().databases());
        Box::pin(async move {
            println!("{:?}", uri);
            let mut request = self
                .client
                .prepare_request(uri.as_str(), http::Method::POST);

            let body = CreateStorageServiceRequest {
                name: self.name.clone(),
                description: self.description.clone(),
                service_type: self.service_type.clone(),
            };

            request.set_body(Bytes::from(serde_json::to_string(&body)?).into());
            let _response = self
                .client
                .pipeline()
                .send(&mut self.context.clone(), &mut request)
                .await?;

            println!("{:?}", _response);

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_storage_service() {
        let client =
            OpenMetadataClient::new("http://localhost:8585", OpenMetadataOptions::default())
                .into_services_client();

        let result = client.create_storage_service("new_service2").into_future().await.unwrap();

        println!("{:?}", result)
    }
}
