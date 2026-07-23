use serde::{Serialize, Deserialize};

use crate::constants::{STORAGE_MAP, STRING};
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, StringPrimitive, TySONMap, TySONPrimitive};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StorageMap {
    pub(crate) values: HashMap<StringPrimitive, Item>,
}

impl BaseTySONItemInterface for StorageMap {
    fn get_prefix(&self) -> String {
        STORAGE_MAP.to_string()
    }
}

impl TySONMap for StorageMap {
    fn new(_: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self {
            values: HashMap::new(),
        })
    }

    fn insert(&mut self, k: Primitive, v: Item) -> Result<bool, DBError> {
        match k {
            Primitive::StringPrimitive(o) => {
                self.values.insert(o, v.clone());
            }
            _ => return Err(DBError::TypeMismatch("storage map allows only string keys".to_string())),
        }
        Ok(true)
    }

    fn get_items(&self) -> Vec<(Primitive, Item)> {
        let mut ve: Vec<(Primitive, Item)> = vec![];
        for (k, v) in &self.values {
            ve.push((Primitive::StringPrimitive(k.clone()), v.clone()));
        }
        ve
    }

    fn to_item(self) -> Item {
        Item::Map(MapItem::StorageMap(self))
    }
}

impl StorageMap {
    pub(crate) fn get_by_str(&self, k: &str) -> Result<Option<&Item>, DBError> {
        Ok(self
            .values
            .get(&StringPrimitive::new("".to_string(), k.to_string())?))
    }

    pub(crate) fn replace_by_string(&mut self, k: String, item: Item) -> Result<bool, DBError> {
        self.insert(Primitive::new(STRING.to_string(), k)?, item)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TySONPrimitive;

    #[test]
    fn storage_map_get_by_str() {
        let mut m = StorageMap::new("".to_string()).unwrap();
        let key = crate::StringPrimitive::new("".to_string(), "name".to_string()).unwrap();
        let val = crate::Item::Primitive(crate::Primitive::new("s".to_string(), "Ann".to_string()).unwrap());
        m.insert(crate::Primitive::StringPrimitive(key.clone()), val).unwrap();
        assert!(m.get_by_str("name").unwrap().is_some());
        assert!(m.get_by_str("missing").unwrap().is_none());
    }

    #[test]
    fn storage_map_replace() {
        let mut m = StorageMap::new("".to_string()).unwrap();
        let key = crate::StringPrimitive::new("".to_string(), "x".to_string()).unwrap();
        let val1 = crate::Item::Primitive(crate::Primitive::new("s".to_string(), "old".to_string()).unwrap());
        m.insert(crate::Primitive::StringPrimitive(key.clone()), val1).unwrap();
        let val2 = crate::Item::Primitive(crate::Primitive::new("s".to_string(), "new".to_string()).unwrap());
        m.replace_by_string("x".to_string(), val2).unwrap();
        let found = m.get_by_str("x").unwrap();
        assert!(found.is_some(), "replaced key should still exist");
    }
}
