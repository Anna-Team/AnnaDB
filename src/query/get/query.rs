use crate::constants::GET_QUERY;
use crate::query::operations::QueryOperation;
use crate::{DBError, Item, Link, Primitive, TySONVector, VectorItem};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetQuery {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for GetQuery {
    fn get_prefix(&self) -> String {
        GET_QUERY.to_string()
    }
}

impl TySONVector for GetQuery {
    fn new(_: String) -> Result<Self, DBError> {
        Ok(Self { items: vec![] })
    }

    fn push(&mut self, item: Item) -> Result<bool, DBError> {
        match &item {
            Item::Primitive(Primitive::Link(_)) => self.items.push(item),
            _ => {
                return Err(DBError::new("Get query can contain only links"));
            }
        }
        Ok(true)
    }

    fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    fn to_item(self) -> Item {
        Item::Vector(VectorItem::GetQuery(self))
    }
}

impl GetQuery {
    pub(crate) fn get_ids(&self) -> Result<Vec<&Link>, DBError> {
        let mut ids: Vec<&Link> = vec![];
        for item in &self.items {
            match &item {
                Item::Primitive(Primitive::Link(o)) => ids.push(o),
                _ => {
                    return Err(DBError::new("Get query can contain only links"));
                }
            }
        }
        Ok(ids)
    }

    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![
            QueryOperation::UpdateOperation,
            QueryOperation::DeleteOperation,
            QueryOperation::LimitOperation,
            QueryOperation::OffsetOperation,
            QueryOperation::ProjectOperation,
        ]
    }
}
