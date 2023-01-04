use std::fmt::Debug;

use crate::constants::NULL;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct NullPrimitive;

impl BaseTySONItemInterface for NullPrimitive {
    fn get_prefix(&self) -> String {
        return NULL.to_string();
    }
}

impl TySONPrimitive for NullPrimitive {
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
