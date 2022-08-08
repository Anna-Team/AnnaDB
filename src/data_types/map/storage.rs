use crate::constants::{STORAGE_MAP, STRING};
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, StringPrimitive, TySONMap, TySONPrimitive};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
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
            _ => return Err(DBError::new("Storage map allows only string keys")),
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
