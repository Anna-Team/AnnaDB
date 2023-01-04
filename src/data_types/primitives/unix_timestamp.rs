use std::fmt::Debug;

use crate::constants::UTS;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, PartialOrd, Ord)]
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
