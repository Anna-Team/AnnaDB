use std::fmt::Debug;

use crate::constants::COLLECTION_NAME;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd)]
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
