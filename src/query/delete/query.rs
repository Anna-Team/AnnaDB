use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::DELETE_QUERY;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::{DBError, Item, Primitive};

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct DeleteQuery;

impl BaseTySONItemInterface for DeleteQuery {
    fn get_prefix(&self) -> String {
        return DELETE_QUERY.to_string();
    }
}

impl TySONPrimitive for DeleteQuery {
    fn new(_: String, _: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn get_string_value(&self) -> String {
        "".to_string()
    }
}

impl DeleteQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![]
    }

    pub fn to_item(self) -> Item {
        Item::Primitive(Primitive::DeleteQuery(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_query_prefix() {
        let dq = DeleteQuery::new("".to_string(), "".to_string()).unwrap();
        assert_eq!(dq.get_prefix(), "delete");
    }

    #[test]
    fn delete_query_to_item() {
        let dq = DeleteQuery::new("".to_string(), "".to_string()).unwrap();
        let item = dq.to_item();
        assert!(matches!(item, Item::Primitive(Primitive::DeleteQuery(_))));
    }

    #[test]
    fn delete_query_next_available() {
        let dq = DeleteQuery::new("".to_string(), "".to_string()).unwrap();
        assert!(dq.next_available().is_empty());
    }

    #[test]
    fn delete_query_get_string_value() {
        let dq = DeleteQuery::new("".to_string(), "".to_string()).unwrap();
        assert_eq!(dq.get_string_value(), "");
    }
}
