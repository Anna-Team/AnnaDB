use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::UTS;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, PartialOrd, Serialize, Deserialize)]
pub struct UTSPrimitive {
    value: i64,
}

impl BaseTySONItemInterface for UTSPrimitive {
    fn get_prefix(&self) -> String {
        return UTS.to_string();
    }
}

impl TySONPrimitive for UTSPrimitive {
    fn new(_: String, value: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        let int_value: i64 = value.as_str().parse::<i64>()?;
        Ok(Self { value: int_value })
    }

    fn get_string_value(&self) -> String {
        self.value.to_string()
    }
}

impl PartialEq<Self> for UTSPrimitive {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() == 0 as i64
    }
}

impl Eq for UTSPrimitive {}

impl UTSPrimitive {
    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn add(&self, other: &UTSPrimitive) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl From<usize> for UTSPrimitive {
    fn from(n: usize) -> Self {
        Self { value: n as i64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uts_primitive_creation() {
        let u = UTSPrimitive::new("".to_string(), "1234567890".to_string()).unwrap();
        assert_eq!(u.get_value(), 1234567890);
        assert_eq!(u.get_prefix(), "uts");
    }

    #[test]
    fn uts_primitive_add() {
        let a = UTSPrimitive::new("".to_string(), "100".to_string()).unwrap();
        let b = UTSPrimitive::new("".to_string(), "50".to_string()).unwrap();
        let c = a.add(&b);
        assert_eq!(c.get_value(), 150);
    }

    #[test]
    fn uts_primitive_equality() {
        let a = UTSPrimitive::new("".to_string(), "42".to_string()).unwrap();
        let b = UTSPrimitive::new("".to_string(), "42".to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn uts_primitive_from_usize() {
        let u = UTSPrimitive::from(42usize);
        assert_eq!(u.get_value(), 42);
    }

    #[test]
    fn uts_primitive_get_string_value() {
        let u = UTSPrimitive::new("".to_string(), "1234567890".to_string()).unwrap();
        assert_eq!(u.get_string_value(), "1234567890");
    }
}
