use crate::{
    clients::{OpenMetadataClient, PagedReturn},
    generated::{CreateDatabaseRequest, Database, EntityReference},
    request_options::{QueryAfter, QueryBefore, QueryFields, QueryLimit, QueryService},
};
use bytes::Bytes;
use reqwest_pipeline::{setters, AppendToUrlQuery, Context, Pageable};

/// A future of a create database response
type CreateDatabase = futures::future::BoxFuture<'static, crate::Result<()>>;
type ListDatabases = Pageable<PagedReturn<Database>>;

#[derive(Clone, Debug)]
pub struct ListDatabasesBuilder {
    client: OpenMetadataClient,
    fields: Option<QueryFields>,
    service: Option<QueryService>,
    limit: Option<QueryLimit>,
    before: Option<QueryBefore>,
    after: Option<QueryAfter>,
    context: Option<Context>,
}

impl ListDatabasesBuilder {
    pub fn new(client: OpenMetadataClient) -> Self {
        Self {
            client: client,
            fields: None,
            service: None,
            limit: None,
            before: None,
            after: None,
            context: None,
        }
    }

    setters! {
        fields: QueryFields => Some(fields),
        service: QueryService => Some(service),
        limit: QueryLimit => Some(limit),
        before: QueryBefore => Some(before),
        after: QueryAfter => Some(after),
        context: Context => Some(context),
    }

    pub fn into_stream<'a>(self) -> ListDatabases {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone().unwrap_or_default();

            async move {
                let mut uri = this.client.api_routes().databases().clone();
                this.fields.append_to_url_query(&mut uri);
                this.service.append_to_url_query(&mut uri);
                this.limit.append_to_url_query(&mut uri);
                this.before.append_to_url_query(&mut uri);
                this.after.append_to_url_query(&mut uri);

                let mut request = this.client.prepare_request(uri.as_str(), http::Method::GET);

                // if let Some(c) = continuation {
                //     match http::HeaderValue::from_str(c.as_str()) {
                //         Ok(h) => request.headers_mut().append(headers::CONTINUATION, h),
                //         Err(e) => return Err(azure_core::Error::Other(Box::new(e))),
                //     };
                // }

                let response = match this
                    .client
                    .pipeline()
                    .send(&mut ctx.clone(), &mut request)
                    .await
                {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };

                PagedReturn::<Database>::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder {
    client: OpenMetadataClient,
    database_name: String,
    service: EntityReference,
    description: Option<String>,
    owner: Option<EntityReference>,
    context: Context,
}

impl CreateDatabaseBuilder {
    pub(crate) fn new(
        client: OpenMetadataClient,
        database_name: String,
        service: EntityReference,
    ) -> Self {
        Self {
            client,
            database_name,
            service,
            description: None,
            owner: None,
            context: Context::new(),
        }
    }

    setters! {
        description: String => Some(description),
        owner: EntityReference => Some(owner),
        context: Context => context,
    }

    pub fn insert<E: Send + Sync + 'static>(&mut self, entity: E) -> &mut Self {
        self.context.insert(entity);
        self
    }

    pub fn into_future(mut self) -> CreateDatabase {
        Box::pin(async move {
            let mut request = self.client.prepare_request("dbs", http::Method::POST);

            let body = CreateDatabaseRequest {
                name: self.database_name.clone(),
                description: self.description.clone(),
                owner: self.owner.clone(),
                service: self.service.clone(),
            };

            request.set_body(Bytes::from(serde_json::to_string(&body)?).into());
            let response = self
                .client
                .pipeline()
                .send(&mut self.context, &mut request)
                .await?;

            Ok(())
        })
    }
}
