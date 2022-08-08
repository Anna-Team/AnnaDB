use crate::constants::RESPONSE_IDS;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, Link, Primitive, TySONVector, VectorItem};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
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
            _ => return Err(DBError::new("Only Link primitives can be ids")),
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
