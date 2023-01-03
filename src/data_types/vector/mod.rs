use std::fmt::Debug;

use crate::query::find::query::FindQuery;
use crate::query::insert::query::InsertQuery;
use crate::query::queryset::QuerySet;

use crate::constants::{
    AND_OPERATOR, FIND_QUERY, GET_QUERY, INSERT_QUERY, OR_OPERATOR, QUERY_SET, RESPONSE_IDS,
    SORT_QUERY, STORAGE_VECTOR, UPDATE_QUERY,
};
use crate::data_types::item::Item;
use crate::data_types::vector::storage::StorageVector;
use crate::query::find::operators::and::AndOperator;
use crate::query::find::operators::or::OrOperator;
use crate::query::get::query::GetQuery;
use crate::query::sort::query::SortQuery;
use crate::query::update::query::UpdateQuery;
use crate::response::ids::ResponseIds;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::vector::TySONVector;
use crate::DBError;

pub mod storage;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VectorItem {
    StorageVector(StorageVector),

    // queries
    QueriesVector(QuerySet),
    InsertQuery(InsertQuery),
    FindQuery(FindQuery),
    GetQuery(GetQuery),
    UpdateQuery(UpdateQuery),
    SortQuery(SortQuery),

    // find operators
    AndOperator(AndOperator),
    OrOperator(OrOperator),

    // response
    ResponseIds(ResponseIds),
}

impl BaseTySONItemInterface for VectorItem {
    fn get_prefix(&self) -> String {
        match self {
            VectorItem::StorageVector(_) => STORAGE_VECTOR.to_string(),

            // QUERIES
            VectorItem::QueriesVector(_) => QUERY_SET.to_string(),
            VectorItem::InsertQuery(_) => INSERT_QUERY.to_string(),
            VectorItem::FindQuery(_) => FIND_QUERY.to_string(),
            VectorItem::GetQuery(_) => GET_QUERY.to_string(),
            VectorItem::UpdateQuery(_) => UPDATE_QUERY.to_string(),
            VectorItem::SortQuery(_) => SORT_QUERY.to_string(),

            // FIND OPERATORS
            VectorItem::AndOperator(_) => AND_OPERATOR.to_string(),
            VectorItem::OrOperator(_) => OR_OPERATOR.to_string(),

            // RESPONSE IDS
            VectorItem::ResponseIds(_) => RESPONSE_IDS.to_string(),
        }
    }
}

impl TySONVector for VectorItem {
    fn new(prefix: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        match prefix.as_str() {
            STORAGE_VECTOR => Ok(VectorItem::StorageVector(StorageVector::new(
                "".to_string(),
            )?)),

            // QUERIES
            QUERY_SET => Ok(VectorItem::QueriesVector(QuerySet::new("".to_string())?)),
            INSERT_QUERY => Ok(VectorItem::InsertQuery(InsertQuery::new("".to_string())?)),
            FIND_QUERY => Ok(VectorItem::FindQuery(FindQuery::new("".to_string())?)),
            GET_QUERY => Ok(VectorItem::GetQuery(GetQuery::new("".to_string())?)),
            UPDATE_QUERY => Ok(VectorItem::UpdateQuery(UpdateQuery::new("".to_string())?)),
            SORT_QUERY => Ok(VectorItem::SortQuery(SortQuery::new("".to_string())?)),

            // FIND OPERATORS
            AND_OPERATOR => Ok(VectorItem::AndOperator(AndOperator::new("".to_string())?)),
            OR_OPERATOR => Ok(VectorItem::OrOperator(OrOperator::new("".to_string())?)),

            // RESPONSE
            RESPONSE_IDS => Ok(VectorItem::ResponseIds(ResponseIds::new("".to_string())?)),

            // OTHER
            _ => Err(DBError::new("Unexpected vector type")),
        }
    }

    fn push(&mut self, item: Item) -> Result<bool, DBError> {
        match self {
            VectorItem::StorageVector(o) => o.push(item),

            // QUERIES
            VectorItem::QueriesVector(o) => o.push(item),
            VectorItem::InsertQuery(o) => o.push(item),
            VectorItem::FindQuery(o) => o.push(item),
            VectorItem::GetQuery(o) => o.push(item),
            VectorItem::UpdateQuery(o) => o.push(item),
            VectorItem::SortQuery(o) => o.push(item),

            // FIND OPERATORS
            VectorItem::AndOperator(o) => o.push(item),
            VectorItem::OrOperator(o) => o.push(item),

            // RESPONSE
            VectorItem::ResponseIds(o) => o.push(item),
        }
    }

    fn get_items(&self) -> &Vec<Item> {
        match self {
            VectorItem::StorageVector(o) => o.get_items(),

            // QUERIES
            VectorItem::QueriesVector(o) => o.get_items(),
            VectorItem::InsertQuery(o) => o.get_items(),
            VectorItem::FindQuery(o) => o.get_items(),
            VectorItem::GetQuery(o) => o.get_items(),
            VectorItem::UpdateQuery(o) => o.get_items(),
            VectorItem::SortQuery(o) => o.get_items(),

            // FIND OPERATORS
            VectorItem::AndOperator(o) => o.get_items(),
            VectorItem::OrOperator(o) => o.get_items(),

            // RESPONSE
            VectorItem::ResponseIds(o) => o.get_items(),
        }
    }

    fn to_item(self) -> Item {
        Item::Vector(self)
    }
}
