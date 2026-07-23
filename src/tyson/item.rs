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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tyson_item_serialize_primitive() {
        use crate::data_types::primitives::string::StringPrimitive;
        use crate::tyson::primitive::TySONPrimitive;
        let s = StringPrimitive::new("s".to_string(), "hello".to_string()).unwrap();
        let item = TySONItem::Primitive(Box::new(s));
        let serialized = item.serialize();
        assert!(serialized.contains("hello"));
    }
}
