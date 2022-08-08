use std::fmt::Debug;

use crate::constants::DELETED;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub struct DeletedPrimitive;

impl BaseTySONItemInterface for DeletedPrimitive {
    fn get_prefix(&self) -> String {
        return DELETED.to_string();
    }
}

impl TySONPrimitive for DeletedPrimitive {
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
