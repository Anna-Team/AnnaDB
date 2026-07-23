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
        let data_string = self.data.to_tyson();
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
        Self {
            error: e.to_string(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::meta::FindMeta;

    #[test]
    fn query_response_new() {
        let data = Item::Primitive(crate::Primitive::new("null".to_string(), "".to_string()).unwrap());
        let meta = Meta::FindMeta(FindMeta::new(5));
        let resp = QueryResponse::new(data, meta, QueryStatus::Ready);
        assert!(resp.serialize().contains("response"));
    }

    #[test]
    fn query_status_not_fetched() {
        let data = Item::Primitive(crate::Primitive::new("null".to_string(), "".to_string()).unwrap());
        let meta = Meta::FindMeta(FindMeta::new(0));
        let resp = QueryResponse::new(data, meta, QueryStatus::NotFetched);
        assert!(resp.serialize().contains("response"));
    }

    #[test]
    fn ok_transaction_response() {
        let mut ok = OkTransactionResponse::new();
        assert!(ok.responses.is_empty());
        let data = Item::Primitive(crate::Primitive::new("null".to_string(), "".to_string()).unwrap());
        let meta = Meta::FindMeta(FindMeta::new(1));
        ok.add_response(QueryResponse::new(data, meta, QueryStatus::Ready));
        assert!(!ok.responses.is_empty());
        assert!(ok.serialize().contains("ok"));
    }

    #[test]
    fn error_transaction_response() {
        let err = ErrorTransactionResponse::from(crate::DBError::ItemNotFound);
        assert!(err.serialize().contains("error"));
    }

    #[test]
    fn transaction_response_ok() {
        let mut ok = OkTransactionResponse::new();
        let resp = TransactionResponse::Ok(ok);
        assert!(resp.serialize().contains("ok"));
    }
}
