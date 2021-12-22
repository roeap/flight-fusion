use crate::{
    clients::{OpenMetadataClient, PagedReturn},
    models::Database,
    request_options::{QueryFields, QueryAfter, QueryBefore, QueryLimit, QueryService},
};
use reqwest_pipeline::{setters, AppendToUrlQuery, Context, Pageable};

#[derive(Clone, Debug)]
pub struct ListDatabases {
    open_meta_client: OpenMetadataClient,
    fields: Option<QueryFields>,
    service: Option<QueryService>,
    limit: Option<QueryLimit>,
    before: Option<QueryBefore>,
    after: Option<QueryAfter>,
    context: Option<Context>,
}

impl ListDatabases {
    pub fn new(client: OpenMetadataClient) -> Self {
        Self {
            open_meta_client: client,
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

    pub fn into_stream<'a>(self) -> Pageable<PagedReturn<Database>> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone().unwrap_or_default();

            async move {
                let mut uri = this.open_meta_client.api_routes().databases().clone();
                this.fields.append_to_url_query(&mut uri);
                this.service.append_to_url_query(&mut uri);
                this.limit.append_to_url_query(&mut uri);
                this.before.append_to_url_query(&mut uri);
                this.after.append_to_url_query(&mut uri);

                let mut request = this
                    .open_meta_client
                    .prepare_request(uri.as_str(), http::Method::GET);

                // if let Some(c) = continuation {
                //     match http::HeaderValue::from_str(c.as_str()) {
                //         Ok(h) => request.headers_mut().append(headers::CONTINUATION, h),
                //         Err(e) => return Err(azure_core::Error::Other(Box::new(e))),
                //     };
                // }

                let response = match this
                    .open_meta_client
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
