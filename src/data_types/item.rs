use crate::data_types::map::MapItem;
use crate::data_types::modifier::ModifierItem;
use crate::data_types::primitives::link::Link;
use crate::data_types::primitives::Primitive;
use crate::data_types::vector::VectorItem;
use crate::tyson::map::TySONMap;
use crate::tyson::modifier::TySONModifier;
use crate::tyson::vector::TySONVector;
use crate::DBError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    Primitive(Primitive),
    Map(MapItem),
    Vector(VectorItem),
    Modifier(ModifierItem),
}

impl Item {
    pub fn serialize(&self) -> String {
        match self {
            Self::Primitive(o) => o.serialize(),
            Self::Map(o) => o.serialize(),
            Self::Vector(o) => o.serialize(),
            Self::Modifier(o) => o.serialize(),
        }
    }

    pub(crate) fn to_link(&self) -> Result<Link, DBError> {
        match self {
            Self::Primitive(Primitive::Link(o)) => Ok(o.clone()),
            _ => Err(DBError::new("Unexpected type. Link was expected")),
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
