use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::NULL;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_primitive_prefix() {
        assert_eq!(NullPrimitive.get_prefix(), "null");
    }

    #[test]
    fn null_primitive_new() {
        let n = NullPrimitive::new("".to_string(), "".to_string()).unwrap();
        assert_eq!(n.get_string_value(), "");
    }
}
