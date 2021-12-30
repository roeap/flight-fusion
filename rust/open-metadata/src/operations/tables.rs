use super::PagedReturn;
use crate::{
    clients::OpenMetadataClient,
    generated::Table,
    request_options::{QueryAfter, QueryBefore, QueryDatabase, QueryFields, QueryLimit},
};
use reqwest_pipeline::{setters, AppendToUrlQuery, Context, Pageable};
use std::pin::Pin;

type ListTables = Pin<Box<Pageable<PagedReturn<Table>>>>;

#[derive(Clone, Debug)]
pub struct ListTablesBuilder {
    client: OpenMetadataClient,
    fields: Option<QueryFields>,
    database: Option<QueryDatabase>,
    limit: Option<QueryLimit>,
    before: Option<QueryBefore>,
    after: Option<QueryAfter>,
    context: Option<Context>,
}

impl ListTablesBuilder {
    pub fn new(client: OpenMetadataClient) -> Self {
        Self {
            client: client,
            fields: None,
            database: None,
            limit: None,
            before: None,
            after: None,
            context: None,
        }
    }

    setters! {
        fields: QueryFields => Some(fields),
        database: QueryDatabase => Some(database),
        limit: QueryLimit => Some(limit),
        before: QueryBefore => Some(before),
        after: QueryAfter => Some(after),
        context: Context => Some(context),
    }

    pub fn into_stream<'a>(self) -> ListTables {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone().unwrap_or_default();

            async move {
                let mut uri = this.client.api_routes().tables().clone();
                this.fields.append_to_url_query(&mut uri);
                this.database.append_to_url_query(&mut uri);
                this.limit.append_to_url_query(&mut uri);
                this.before.append_to_url_query(&mut uri);

                if let Some(c) = continuation {
                    let param = QueryAfter::new(c);
                    param.append_to_url_query(&mut uri);
                } else {
                    this.after.append_to_url_query(&mut uri);
                }

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

                PagedReturn::<Table>::try_from(response).await
            }
        };

        Box::pin(Pageable::new(make_request))
    }
}
