use reqwest_pipeline::prelude::*;
use std::convert::TryFrom;
use std::num::NonZeroU32;

// This type forbids zero as value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct MaxResults(NonZeroU32);

impl MaxResults {
    pub fn new(max_results: NonZeroU32) -> Self {
        Self(max_results)
    }
}

impl AppendToUrlQuery for MaxResults {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("limit", &format!("{}", self.0));
    }
}

impl From<NonZeroU32> for MaxResults {
    fn from(max_results: NonZeroU32) -> Self {
        Self::new(max_results)
    }
}

impl TryFrom<u32> for MaxResults {
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

#[derive(Debug, Clone)]
pub struct FieldsQuery(String);

impl FieldsQuery {
    pub fn new(fields: String) -> Self {
        Self(fields)
    }
}

impl AppendToUrlQuery for FieldsQuery {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("fields", &format!("{}", self.0));
    }
}

impl From<String> for FieldsQuery {
    fn from(fields: String) -> Self {
        Self::new(fields)
    }
}

#[derive(Debug, Clone)]
pub struct PagingBefore(String);

impl PagingBefore {
    pub fn new(fields: String) -> Self {
        Self(fields)
    }
}

impl AppendToUrlQuery for PagingBefore {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("before", &format!("{}", self.0));
    }
}

impl From<String> for PagingBefore {
    fn from(fields: String) -> Self {
        Self::new(fields)
    }
}

#[derive(Debug, Clone)]
pub struct PagingAfter(String);

impl PagingAfter {
    pub fn new(fields: String) -> Self {
        Self(fields)
    }
}

impl AppendToUrlQuery for PagingAfter {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("before", &format!("{}", self.0));
    }
}

impl From<String> for PagingAfter {
    fn from(fields: String) -> Self {
        Self::new(fields)
    }
}
