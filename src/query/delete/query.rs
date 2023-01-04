use std::fmt::Debug;

use crate::constants::DELETE_QUERY;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::{DBError, Item, Primitive};

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
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
