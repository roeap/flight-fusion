use super::{get_headers, PagedReturn};
use crate::{
    clients::OpenMetadataClient,
    generated::{
        CollectionDescriptor, CreateDatabaseServiceRequest, CreateStorageServiceRequest,
        DatabaseService, DatabaseServiceType, JdbcInfo, Schedule, StorageService,
        StorageServiceType,
    },
};
use bytes::Bytes;
use reqwest_pipeline::{setters, Context, Pageable};
use std::pin::Pin;

type CreateStorageService = futures::future::BoxFuture<'static, crate::Result<StorageService>>;
type CreateDatabaseService = futures::future::BoxFuture<'static, crate::Result<DatabaseService>>;
type ListServices = Pin<Box<Pageable<PagedReturn<CollectionDescriptor>>>>;

#[derive(Clone, Debug)]
pub struct ListServicesBuilder {
    client: OpenMetadataClient,
}

impl ListServicesBuilder {
    pub fn new(client: OpenMetadataClient) -> Self {
        Self { client }
    }

    pub fn into_stream(self) -> ListServices {
        let make_request = move |_: Option<String>| {
            let this = self.clone();
            let ctx = Context::new();

            async move {
                let uri = this.client.api_routes().services();
                let mut request = this.client.prepare_request(uri.as_str(), http::Method::GET);

                let response = match this
                    .client
                    .pipeline()
                    .send(&mut ctx.clone(), &mut request)
                    .await
                {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };

                PagedReturn::<CollectionDescriptor>::try_from(response).await
            }
        };

        Box::pin(Pageable::new(make_request))
    }
}

#[derive(Clone, Debug)]
pub struct CreateStorageServiceBuilder {
    client: OpenMetadataClient,
    context: Context,
    name: String,
    description: Option<String>,
    service_type: Option<StorageServiceType>,
}

impl CreateStorageServiceBuilder {
    pub(crate) fn new(client: OpenMetadataClient, name: String) -> Self {
        let mut context = Context::new();
        context.insert(get_headers());

        Self {
            client,
            context,
            name,
            service_type: None,
            description: None,
        }
    }

    setters! {
        description: String => Some(description),
        service_type: StorageServiceType => Some(service_type),
    }

    pub fn into_future(self) -> CreateStorageService {
        let uri = self
            .client
            .api_routes()
            .services()
            .join("services/storageServices")
            .unwrap();

        Box::pin(async move {
            let mut request = self
                .client
                .prepare_request(uri.as_str(), http::Method::POST);

            let body = CreateStorageServiceRequest {
                name: self.name.clone(),
                description: self.description.clone(),
                service_type: self.service_type.clone(),
            };

            request.set_body(Bytes::from(serde_json::to_string(&body)?).into());
            let response = self
                .client
                .pipeline()
                .send(&mut self.context.clone(), &mut request)
                .await?
                .into_body_string()
                .await;

            Ok(serde_json::from_str(&response)?)
        })
    }
}

#[derive(Clone, Debug)]
pub struct CreateDatabaseServiceBuilder {
    client: OpenMetadataClient,
    context: Context,
    name: String,
    service_type: DatabaseServiceType,
    jdbc: JdbcInfo,
    description: Option<String>,
    ingestion_schedule: Option<Schedule>,
}

impl CreateDatabaseServiceBuilder {
    pub(crate) fn new(
        client: OpenMetadataClient,
        name: String,
        service_type: DatabaseServiceType,
        jdbc: JdbcInfo,
    ) -> Self {
        let mut context = Context::new();
        context.insert(get_headers());

        Self {
            client,
            context,
            name,
            service_type,
            jdbc,
            description: None,
            ingestion_schedule: None,
        }
    }

    setters! {
        description: String => Some(description),
        ingestion_schedule: Schedule => Some(ingestion_schedule),
    }

    pub fn into_future(self) -> CreateDatabaseService {
        let uri = self
            .client
            .api_routes()
            .services()
            .join("services/databaseServices")
            .unwrap();

        Box::pin(async move {
            let mut request = self
                .client
                .prepare_request(uri.as_str(), http::Method::POST);

            let body = CreateDatabaseServiceRequest {
                name: self.name.clone(),
                description: self.description.clone(),
                service_type: self.service_type.clone(),
                jdbc: self.jdbc.clone(),
                ingestion_schedule: self.ingestion_schedule.clone(),
            };
            request.set_body(Bytes::from(serde_json::to_string(&body)?).into());

            let response = self
                .client
                .pipeline()
                .send(&mut self.context.clone(), &mut request)
                .await?
                .into_body_string()
                .await;

            Ok(serde_json::from_str(&response)?)
        })
    }
}
