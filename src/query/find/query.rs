use crate::constants::FIND_QUERY;
use crate::query::operations::QueryOperation;
use crate::{DBError, Item, TySONVector, VectorItem};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FindQuery {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for FindQuery {
    fn get_prefix(&self) -> String {
        FIND_QUERY.to_string()
    }
}

impl TySONVector for FindQuery {
    fn new(_: String) -> Result<Self, DBError> {
        Ok(Self { items: vec![] })
    }

    fn push(&mut self, item: Item) -> Result<bool, DBError> {
        self.items.push(item);
        Ok(true)
    }

    fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    fn to_item(self) -> Item {
        Item::Vector(VectorItem::FindQuery(self))
    }
}

impl FindQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![
            QueryOperation::FindOperation,
            QueryOperation::UpdateOperation,
            QueryOperation::DeleteOperation,
            QueryOperation::SortOperation,
            QueryOperation::LimitOperation,
            QueryOperation::OffsetOperation,
            QueryOperation::ProjectOperation,
        ]
    }
}
