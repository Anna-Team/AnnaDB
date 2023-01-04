use std::fmt::Debug;

use crate::constants::KEEP;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct KeepPrimitive;

impl BaseTySONItemInterface for KeepPrimitive {
    fn get_prefix(&self) -> String {
        return KEEP.to_string();
    }
}

impl TySONPrimitive for KeepPrimitive {
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
