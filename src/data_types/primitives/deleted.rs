use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::DELETED;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deleted_primitive_prefix() {
        assert_eq!(DeletedPrimitive.get_prefix(), "deleted");
    }

    #[test]
    fn deleted_primitive_new() {
        let d = DeletedPrimitive::new("".to_string(), "".to_string()).unwrap();
        assert_eq!(d.get_string_value(), "");
    }
}
