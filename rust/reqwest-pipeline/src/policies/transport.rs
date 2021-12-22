use crate::policies::{Policy, PolicyResult};
use crate::{Context, Request, Response, TransportOptions};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportPolicy {
    #[allow(unused)]
    pub(crate) transport_options: TransportOptions,
}

impl TransportPolicy {
    pub fn new(transport_options: TransportOptions) -> Self {
        Self { transport_options }
    }
}

#[async_trait::async_trait]
impl Policy for TransportPolicy {
    async fn send(
        &self,
        _ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        let response = { self.transport_options.http_client.execute_request(request) };

        Ok(response.await?)
    }
}
