use reqwest_pipeline::{prelude::*, query_prop};
use std::convert::TryFrom;
use std::num::NonZeroU32;

query_prop! {
    service: String => QueryService,
    database: String => QueryDatabase,
    fields: String => QueryFields,
    after: String => QueryAfter,
    before: String => QueryBefore,
}

// This type forbids zero as value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct QueryLimit(NonZeroU32);

impl QueryLimit {
    pub fn new(max_results: NonZeroU32) -> Self {
        Self(max_results)
    }
}

impl AppendToUrlQuery for QueryLimit {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("limit", &format!("{}", self.0));
    }
}

impl From<NonZeroU32> for QueryLimit {
    fn from(max_results: NonZeroU32) -> Self {
        Self::new(max_results)
    }
}

impl TryFrom<u32> for QueryLimit {
    type Error = String;

    fn try_from(max_results: u32) -> Result<Self, Self::Error> {
        match NonZeroU32::new(max_results) {
            Some(max_results) => Ok(max_results.into()),
            None => Err(format!(
                "number {} is not a valid NonZeroU32 value",
                max_results
            )),
        }
    }
}
