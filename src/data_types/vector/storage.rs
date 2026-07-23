use serde::{Serialize, Deserialize};

use crate::constants::STORAGE_VECTOR;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
            Err(DBError::Validation("vector index does not exist".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_vector_push_and_get() {
        let mut v = StorageVector::new("".to_string()).unwrap();
        let item = crate::Item::Primitive(crate::Primitive::new("s".to_string(), "a".to_string()).unwrap());
        v.push(item.clone()).unwrap();
        assert_eq!(v.get_items().len(), 1);
    }

    #[test]
    fn storage_vector_replace() {
        let mut v = StorageVector::new("".to_string()).unwrap();
        let item = crate::Item::Primitive(crate::Primitive::new("s".to_string(), "a".to_string()).unwrap());
        v.push(item).unwrap();
        let link = crate::Link::create("test".to_string());
        assert!(v.replace_by_string("0".to_string(), crate::Item::Primitive(crate::Primitive::Link(link))).is_ok());
    }

    #[test]
    fn storage_vector_get_by_str() {
        let mut v = StorageVector::new("".to_string()).unwrap();
        let item = crate::Item::Primitive(crate::Primitive::new("n".to_string(), "42".to_string()).unwrap());
        v.push(item.clone()).unwrap();
        assert!(v.get_by_str("0").unwrap().is_some());
        assert!(v.get_by_str("1").unwrap().is_none());
        assert!(v.get_by_str("not_a_number").unwrap().is_none());
    }

    #[test]
    fn storage_vector_replace_out_of_bounds() {
        let mut v = StorageVector::new("".to_string()).unwrap();
        let item = crate::Item::Primitive(crate::Primitive::new("null".to_string(), "".to_string()).unwrap());
        assert!(v.replace_by_string("99".to_string(), item).is_err());
    }
}
