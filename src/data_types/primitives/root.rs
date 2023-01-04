use std::fmt::Debug;

use crate::constants::ROOT;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct RootPrimitive;

impl BaseTySONItemInterface for RootPrimitive {
    fn get_prefix(&self) -> String {
        return ROOT.to_string();
    }
}

impl TySONPrimitive for RootPrimitive {
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
