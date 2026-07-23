use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::STRING;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct StringPrimitive {
    value: String,
}

impl BaseTySONItemInterface for StringPrimitive {
    fn get_prefix(&self) -> String {
        return STRING.to_string();
    }
}

impl TySONPrimitive for StringPrimitive {
    fn new(_: String, value: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self { value })
    }

    fn get_string_value(&self) -> String {
        self.value.to_string()
    }
}

impl From<&str> for StringPrimitive {
    fn from(v: &str) -> Self {
        Self {
            value: v.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_primitive_from_str() {
        let s = StringPrimitive::from("hello");
        assert_eq!(s.get_string_value(), "hello");
        assert_eq!(s.get_prefix(), "s");
    }
}
