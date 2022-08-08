use std::fmt::Debug;

use crate::constants::PATH_TO_VALUE;
use crate::data_types::primitives::root::RootPrimitive;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd)]
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
