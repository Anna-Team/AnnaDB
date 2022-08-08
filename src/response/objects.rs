use crate::constants::RESPONSE_OBJECTS;
use crate::{DBError, Item, Link, MapItem, Primitive, TySONMap};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseObjects {
    pub values: Vec<(Link, Item)>,
}

impl BaseTySONItemInterface for ResponseObjects {
    fn get_prefix(&self) -> String {
        RESPONSE_OBJECTS.to_string()
    }
}

impl TySONMap for ResponseObjects {
    fn new(_: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self { values: vec![] })
    }

    fn insert(&mut self, k: Primitive, v: Item) -> Result<bool, DBError> {
        match k {
            Primitive::Link(o) => {
                self.values.push((o, v));
            }
            _ => return Err(DBError::new("Only Link primitives can be ids")),
        }
        Ok(true)
    }

    fn get_items(&self) -> Vec<(Primitive, Item)> {
        let mut ve: Vec<(Primitive, Item)> = vec![];
        for (k, v) in &self.values {
            ve.push((Primitive::Link(k.clone()), v.clone()));
        }
        ve
    }

    fn to_item(self) -> Item {
        Item::Map(MapItem::ResponseObjects(self))
    }
}
