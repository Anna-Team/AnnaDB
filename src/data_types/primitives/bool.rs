use std::fmt::Debug;

use crate::constants::BOOL;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct BoolPrimitive {
    value: bool,
}

impl BaseTySONItemInterface for BoolPrimitive {
    fn get_prefix(&self) -> String {
        return BOOL.to_string();
    }
}

impl TySONPrimitive for BoolPrimitive {
    fn new(_: String, value: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        match value.as_str() {
            "true" => Ok(Self { value: true }),
            "false" => Ok(Self { value: false }),
            _ => Err(DBError::new("Bool parsing error")),
        }
    }

    fn get_string_value(&self) -> String {
        self.value.to_string()
    }
}

impl BoolPrimitive {
    pub fn create_true() -> Self {
        Self { value: true }
    }

    pub fn create_false() -> Self {
        Self { value: false }
    }

    pub fn val(&self) -> bool {
        self.value
    }
}
