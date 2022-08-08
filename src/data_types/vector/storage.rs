use crate::constants::STORAGE_VECTOR;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StorageVector {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for StorageVector {
    fn get_prefix(&self) -> String {
        STORAGE_VECTOR.to_string()
    }
}

impl TySONVector for StorageVector {
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
        Item::Vector(VectorItem::StorageVector(self))
    }
}

impl StorageVector {
    pub(crate) fn get_by_str(&self, k: &str) -> Result<Option<&Item>, DBError> {
        match k.parse::<usize>() {
            Ok(num) => Ok(self.items.get(num)),
            Err(_) => Ok(None),
        }
    }

    pub(crate) fn replace_by_string(&mut self, k: String, item: Item) -> Result<bool, DBError> {
        let index = k.as_str().parse::<usize>()?;
        if self.items.len() > index {
            self.items[index] = item;
            Ok(true)
        } else {
            Err(DBError::new("Vector index does not exist"))
        }
    }
}
