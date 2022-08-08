use crate::constants::OR_OPERATOR;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrOperator {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for OrOperator {
    fn get_prefix(&self) -> String {
        OR_OPERATOR.to_string()
    }
}

impl TySONVector for OrOperator {
    fn new(_: String) -> Result<Self, DBError> {
        Ok(Self { items: vec![] })
    }

    fn push(&mut self, item: Item) -> Result<bool, DBError> {
        match item {
            Item::Primitive(Primitive::BoolPrimitive(_)) => {
                self.items.push(item);
            }
            Item::Map(MapItem::EqOperator(_)) => {
                self.items.push(item);
            }
            Item::Map(MapItem::NeqOperator(_)) => {
                self.items.push(item);
            }
            Item::Map(MapItem::GtOperator(_)) => {
                self.items.push(item);
            }
            Item::Map(MapItem::GteOperator(_)) => {
                self.items.push(item);
            }
            Item::Map(MapItem::LtOperator(_)) => {
                self.items.push(item);
            }
            Item::Map(MapItem::LteOperator(_)) => {
                self.items.push(item);
            }
            _ => return Err(DBError::new("Unsupported item for OR operator")),
        };
        Ok(true)
    }

    fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    fn to_item(self) -> Item {
        Item::Vector(VectorItem::OrOperator(self))
    }
}
