use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::NUMBER;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, PartialOrd, Serialize, Deserialize)]
pub struct NumberPrimitive {
    value: f64,
}

impl BaseTySONItemInterface for NumberPrimitive {
    fn get_prefix(&self) -> String {
        return NUMBER.to_string();
    }
}

impl TySONPrimitive for NumberPrimitive {
    fn new(_: String, value: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        let f_value: f64 = value.as_str().parse::<f64>()?;
        Ok(Self { value: f_value })
    }

    fn get_string_value(&self) -> String {
        self.value.to_string()
    }
}

impl PartialEq<Self> for NumberPrimitive {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() == f64::from(0) // TODO fix this!!
    }
}

impl Eq for NumberPrimitive {}

impl NumberPrimitive {
    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn add(&self, other: &NumberPrimitive) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl From<usize> for NumberPrimitive {
    fn from(n: usize) -> Self {
        Self { value: n as f64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn number_primitive_get_value() {
        let n = NumberPrimitive::new("".to_string(), "42.5".to_string()).unwrap();
        assert_eq!(n.get_value(), 42.5);
    }

    #[test]
    fn number_primitive_add() {
        let a = NumberPrimitive::new("".to_string(), "10".to_string()).unwrap();
        let b = NumberPrimitive::new("".to_string(), "5".to_string()).unwrap();
        let c = a.add(&b);
        assert_eq!(c.get_value(), 15.0);
    }

    #[test]
    fn number_primitive_equality() {
        let a = NumberPrimitive::new("".to_string(), "42".to_string()).unwrap();
        let b = NumberPrimitive::new("".to_string(), "42".to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn number_primitive_ordering() {
        let a = NumberPrimitive::new("".to_string(), "10".to_string()).unwrap();
        let b = NumberPrimitive::new("".to_string(), "20".to_string()).unwrap();
        assert!(a < b);
    }

    #[test]
    fn number_primitive_from_usize() {
        let n = NumberPrimitive::from(42usize);
        assert_eq!(n.get_value(), 42.0);
    }
}
