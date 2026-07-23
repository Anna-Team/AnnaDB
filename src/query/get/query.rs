use serde::{Serialize, Deserialize};

use crate::constants::GET_QUERY;
use crate::query::operations::QueryOperation;
use crate::{DBError, Item, Link, Primitive, TySONVector, VectorItem};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
                return Err(DBError::TypeMismatch("get query can contain only links".to_string()));
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
                    return Err(DBError::TypeMismatch("get query can contain only links".to_string()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_query_prefix() {
        let gq = GetQuery::new("".to_string()).unwrap();
        assert_eq!(gq.get_prefix(), "get");
    }

    #[test]
    fn get_query_push_link() {
        let mut gq = GetQuery::new("".to_string()).unwrap();
        let link = Link::create("test".to_string());
        let item = Item::Primitive(Primitive::Link(link));
        assert!(gq.push(item).is_ok());
        assert_eq!(gq.items.len(), 1);
    }

    #[test]
    fn get_query_push_rejects_non_link() {
        let mut gq = GetQuery::new("".to_string()).unwrap();
        let item = Item::Primitive(Primitive::new("s".to_string(), "hello".to_string()).unwrap());
        assert!(gq.push(item).is_err());
    }

    #[test]
    fn get_query_get_ids() {
        let mut gq = GetQuery::new("".to_string()).unwrap();
        let link = Link::create("test".to_string());
        gq.push(Item::Primitive(Primitive::Link(link.clone()))).unwrap();
        let ids = gq.get_ids().unwrap();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0], &link);
    }

    #[test]
    fn get_query_next_available() {
        let gq = GetQuery::new("".to_string()).unwrap();
        assert!(!gq.next_available().is_empty());
    }
}
