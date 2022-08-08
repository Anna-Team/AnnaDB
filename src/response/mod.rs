pub mod data;
pub(crate) mod ids;
pub mod meta;
pub mod objects;

use crate::constants::{QUERY_RESPONSE, TRANSACTION_RESPONSE};
use crate::response::meta::Meta;
use crate::{DBError, Item};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QueryStatus {
    Ready,
    NotFetched,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QueryResponse {
    pub data: Item,
    pub meta: Meta,
    pub status: QueryStatus,
}

impl QueryResponse {
    pub fn new(data: Item, meta: Meta, status: QueryStatus) -> Self {
        Self { data, meta, status }
    }

    pub fn serialize(&self) -> String {
        let data_string = self.data.serialize();
        let meta_string = self.meta.serialize();
        format!(
            "{}{{s|data|:{},s|meta|:{},}}",
            QUERY_RESPONSE, data_string, meta_string
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OkTransactionResponse {
    pub responses: Vec<QueryResponse>,
}

impl OkTransactionResponse {
    pub fn new() -> Self {
        Self { responses: vec![] }
    }

    pub fn add_response(&mut self, response: QueryResponse) {
        self.responses.push(response)
    }

    pub fn serialize(&self) -> String {
        let mut query_responses: Vec<String> = vec![];
        for r in &self.responses {
            query_responses.push(r.serialize())
        }
        format!("{}:ok[{}]", TRANSACTION_RESPONSE, query_responses.join(","))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ErrorTransactionResponse {
    error: String,
}

impl From<DBError> for ErrorTransactionResponse {
    fn from(e: DBError) -> Self {
        Self { error: e.msg }
    }
}

impl ErrorTransactionResponse {
    pub fn serialize(&self) -> String {
        format!("{}:error|{}|", TRANSACTION_RESPONSE, self.error)
    }
}

pub enum TransactionResponse {
    Error(ErrorTransactionResponse),
    Ok(OkTransactionResponse),
}

impl TransactionResponse {
    pub fn serialize(&self) -> String {
        match self {
            TransactionResponse::Ok(v) => v.serialize(),
            TransactionResponse::Error(v) => v.serialize(),
        }
    }
}
