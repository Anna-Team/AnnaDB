use std::fmt::Debug;

use crate::tyson::map::TySONMap;
use crate::tyson::primitive::TySONPrimitive;
use crate::tyson::vector::TySONVector;

#[derive(Debug)]
pub enum TySONItem {
    Primitive(Box<dyn TySONPrimitive>),
    Vector(Box<dyn TySONVector>),
    Map(Box<dyn TySONMap>),
}

pub trait BaseTySONItemInterface: Debug {
    fn get_prefix(&self) -> String;
}

impl TySONItem {
    pub(crate) fn serialize(&self) -> String {
        match self {
            TySONItem::Map(o) => o.serialize(),
            TySONItem::Vector(o) => o.serialize(),
            TySONItem::Primitive(o) => o.serialize(),
        }
    }
}
