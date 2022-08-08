use std::fmt::Debug;

use crate::constants::NUMBER;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, PartialOrd)]
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
