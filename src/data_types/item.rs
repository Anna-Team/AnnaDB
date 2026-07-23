use serde::{Serialize, Deserialize};

use crate::data_types::map::MapItem;
use crate::data_types::modifier::ModifierItem;
use crate::data_types::primitives::link::Link;
use crate::data_types::primitives::Primitive;
use crate::data_types::vector::VectorItem;
use crate::tyson::map::TySONMap;
use crate::tyson::modifier::TySONModifier;
use crate::tyson::vector::TySONVector;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Primitive(Primitive),
    Map(MapItem),
    Vector(VectorItem),
    Modifier(ModifierItem),
}

impl Item {
    pub fn to_tyson(&self) -> String {
        match self {
            Self::Primitive(o) => Primitive::serialize(o),
            Self::Map(o) => TySONMap::serialize(o),
            Self::Vector(o) => TySONVector::serialize(o),
            Self::Modifier(o) => TySONModifier::serialize(o),
        }
    }

    pub(crate) fn to_link(&self) -> Result<Link, DBError> {
        match self {
            Self::Primitive(Primitive::Link(o)) => Ok(o.clone()),
            _ => Err(DBError::TypeMismatch("Link expected".to_string())),
        }
    }
}

impl From<Primitive> for Item {
    fn from(data: Primitive) -> Self {
        Item::Primitive(data)
    }
}

impl From<MapItem> for Item {
    fn from(data: MapItem) -> Self {
        Item::Map(data)
    }
}

impl From<VectorItem> for Item {
    fn from(data: VectorItem) -> Self {
        Item::Vector(data)
    }
}

impl From<ModifierItem> for Item {
    fn from(data: ModifierItem) -> Self {
        Item::Modifier(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::find::operators::not::NotOperator;
    use crate::tyson::modifier::TySONModifier;

    #[test]
    fn item_to_tyson_modifier() {
        let val = Item::Primitive(Primitive::new("b".to_string(), "true".to_string()).unwrap());
        let not_op = NotOperator::new("".to_string(), val).unwrap();
        let item = Item::Modifier(ModifierItem::NotOperator(not_op));
        let s = item.to_tyson();
        assert!(s.contains("not"));
    }

    #[test]
    fn item_to_link_error() {
        let item = Item::Primitive(Primitive::new("s".to_string(), "hello".to_string()).unwrap());
        assert!(item.to_link().is_err());
    }
}
