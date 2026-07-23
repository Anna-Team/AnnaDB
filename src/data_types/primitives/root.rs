use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::ROOT;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_primitive_prefix() {
        assert_eq!(RootPrimitive.get_prefix(), "root");
    }

    #[test]
    fn root_primitive_new() {
        let r = RootPrimitive::new("".to_string(), "".to_string()).unwrap();
        assert_eq!(r.get_string_value(), "");
    }
}
