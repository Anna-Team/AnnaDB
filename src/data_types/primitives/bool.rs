use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::BOOL;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
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
            _ => Err(DBError::BoolParse),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_primitive_create_true() {
        assert!(BoolPrimitive::create_true().val());
    }

    #[test]
    fn bool_primitive_create_false() {
        assert!(!BoolPrimitive::create_false().val());
    }

    #[test]
    fn bool_primitive_from_str_true() {
        let b = BoolPrimitive::new("".to_string(), "true".to_string()).unwrap();
        assert!(b.val());
    }

    #[test]
    fn bool_primitive_from_str_false() {
        let b = BoolPrimitive::new("".to_string(), "false".to_string()).unwrap();
        assert!(!b.val());
    }

    #[test]
    fn bool_primitive_invalid_value() {
        assert!(BoolPrimitive::new("".to_string(), "invalid".to_string()).is_err());
    }

    #[test]
    fn bool_primitive_get_prefix() {
        let b = BoolPrimitive::create_true();
        assert_eq!(b.get_prefix(), "b");
    }
}
