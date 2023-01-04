use float_ord::FloatOrd;
use std::fmt::Debug;

use crate::constants::NUMBER;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct NumberPrimitive {
    value: FloatOrd<f64>,
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
        Ok(Self {
            value: FloatOrd(f_value),
        })
    }

    fn get_string_value(&self) -> String {
        self.value.0.to_string()
    }
}

impl PartialEq<Self> for NumberPrimitive {
    fn eq(&self, other: &Self) -> bool {
        (self.value.0 - other.value.0).abs() == f64::from(0) // TODO fix this!!
    }
}

impl Eq for NumberPrimitive {}

impl NumberPrimitive {
    pub fn get_value(&self) -> f64 {
        self.value.0
    }

    pub fn add(&self, other: &NumberPrimitive) -> Self {
        Self {
            value: FloatOrd(self.value.0 + other.value.0),
        }
    }
}

impl From<usize> for NumberPrimitive {
    fn from(n: usize) -> Self {
        Self {
            value: FloatOrd(n as f64),
        }
    }
}
