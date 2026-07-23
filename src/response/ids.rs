use serde::{Serialize, Deserialize};

use crate::constants::RESPONSE_IDS;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, Link, Primitive, TySONVector, VectorItem};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseIds {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for ResponseIds {
    fn get_prefix(&self) -> String {
        RESPONSE_IDS.to_string()
    }
}

impl TySONVector for ResponseIds {
    fn new(_: String) -> Result<Self, DBError> {
        Ok(Self { items: vec![] })
    }

    fn push(&mut self, item: Item) -> Result<bool, DBError> {
        match item {
            Item::Primitive(Primitive::Link(_)) => {
                self.items.push(item);
            }
            _ => return Err(DBError::TypeMismatch("only Link primitives can be ids".to_string())),
        }
        Ok(true)
    }

    fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    fn to_item(self) -> Item {
        Item::Vector(VectorItem::ResponseIds(self))
    }
}

impl From<HashSet<Link>> for ResponseIds {
    fn from(h: HashSet<Link>) -> Self {
        let mut items: Vec<Item> = vec![];
        for l in h {
            items.push(Item::from(Primitive::Link(l)));
        }
        Self { items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_ids_new() {
        let ids = ResponseIds::new("".to_string()).unwrap();
        assert_eq!(ids.get_prefix(), "ids");
    }

    #[test]
    fn response_ids_from_hashset() {
        use std::collections::HashSet;
        let links = HashSet::new();
        let ids = ResponseIds::from(links);
        assert!(ids.get_items().is_empty());
    }

    #[test]
    fn response_ids_push_non_link_returns_err() {
        let mut ids = ResponseIds::new("".to_string()).unwrap();
        let non_link = Item::Primitive(Primitive::new("s".to_string(), "hello".to_string()).unwrap());
        assert!(ids.push(non_link).is_err());
    }

    #[test]
    fn response_ids_push_link_succeeds() {
        let mut ids = ResponseIds::new("".to_string()).unwrap();
        let link = Item::Primitive(Primitive::new("c".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap());
        assert!(ids.push(link).is_ok());
        assert_eq!(ids.get_items().len(), 1);
    }

    #[test]
    fn response_ids_to_item() {
        let ids = ResponseIds::new("".to_string()).unwrap();
        let item = ids.to_item();
        assert!(matches!(item, Item::Vector(VectorItem::ResponseIds(_))));
    }
}
