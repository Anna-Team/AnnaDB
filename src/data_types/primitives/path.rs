use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::PATH_TO_VALUE;
use crate::data_types::primitives::root::RootPrimitive;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct PathToValue {
    pub value: String,
}

impl BaseTySONItemInterface for PathToValue {
    fn get_prefix(&self) -> String {
        return PATH_TO_VALUE.to_string();
    }
}

impl TySONPrimitive for PathToValue {
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

impl PathToValue {
    // pub(crate) fn to_path(&self) -> Vec<String>{
    //     self.value.split(".").collect()
    // }
}

pub enum Path {
    PathToValue(PathToValue),
    Root(RootPrimitive),
}

impl From<PathToValue> for Path {
    fn from(p: PathToValue) -> Self {
        Path::PathToValue(p)
    }
}

impl From<RootPrimitive> for Path {
    fn from(p: RootPrimitive) -> Self {
        Path::Root(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_to_value_creation() {
        let p = PathToValue::new("".to_string(), "foo.bar".to_string()).unwrap();
        assert_eq!(p.get_string_value(), "foo.bar");
        assert_eq!(p.get_prefix(), "value");
    }

    #[test]
    fn path_from_path_to_value() {
        let ptv = PathToValue::new("".to_string(), "a.b".to_string()).unwrap();
        let path = Path::from(ptv);
        match path {
            Path::PathToValue(p) => assert_eq!(p.get_string_value(), "a.b"),
            _ => panic!("expected PathToValue"),
        }
    }

    #[test]
    fn path_from_root() {
        let root = RootPrimitive::new("".to_string(), "".to_string()).unwrap();
        let path = Path::from(root);
        match path {
            Path::Root(_) => {},
            _ => panic!("expected Root"),
        }
    }
}
