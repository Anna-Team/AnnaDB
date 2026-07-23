use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::KEEP;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_primitive_prefix() {
        assert_eq!(KeepPrimitive.get_prefix(), "keep");
    }

    #[test]
    fn keep_primitive_new() {
        let k = KeepPrimitive::new("".to_string(), "".to_string()).unwrap();
        assert_eq!(k.get_string_value(), "");
    }
}
