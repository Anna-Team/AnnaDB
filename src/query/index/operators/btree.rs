use std::fmt::Debug;

use crate::constants::BTREE;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub struct BTreePrimitive;

impl BaseTySONItemInterface for BTreePrimitive {
    fn get_prefix(&self) -> String {
        return BTREE.to_string();
    }
}

impl TySONPrimitive for BTreePrimitive {
    fn new(_: String, _: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn get_string_value(&self) -> String {
        "".to_string()
    }
}
