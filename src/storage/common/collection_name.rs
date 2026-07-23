use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::COLLECTION_NAME;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct CollectionName {
    value: String,
}

impl BaseTySONItemInterface for CollectionName {
    fn get_prefix(&self) -> String {
        return COLLECTION_NAME.to_string();
    }
}

impl TySONPrimitive for CollectionName {
    fn new(_: String, value: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self { value })
    }

    fn get_string_value(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_name_prefix() {
        let cn = CollectionName::new("".to_string(), "test".to_string()).unwrap();
        assert_eq!(cn.get_prefix(), "collection");
        assert_eq!(cn.get_string_value(), "test");
    }
}
