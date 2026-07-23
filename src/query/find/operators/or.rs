use serde::{Serialize, Deserialize};

use crate::constants::OR_OPERATOR;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
            Item::Map(MapItem::KnnOperator(_)) => {
                self.items.push(item);
            }
            _ => return Err(DBError::UnsupportedOperation("item for OR operator".to_string())),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::find::operators::eq::EqOperator;
    use crate::TySONMap;

    #[test]
    fn or_operator_new() {
        let op = OrOperator::new("".to_string()).unwrap();
        assert_eq!(op.get_prefix(), "or");
    }

    #[test]
    fn or_operator_push_eq() {
        let mut op = OrOperator::new("".to_string()).unwrap();
        let eq = EqOperator::new("".to_string()).unwrap();
        let item = eq.to_item();
        assert!(op.push(item).is_ok());
    }

    #[test]
    fn or_operator_push_bool() {
        let mut op = OrOperator::new("".to_string()).unwrap();
        let item = Item::Primitive(Primitive::new("b".to_string(), "true".to_string()).unwrap());
        assert!(op.push(item).is_ok());
    }

    #[test]
    fn or_operator_rejects_invalid() {
        let mut op = OrOperator::new("".to_string()).unwrap();
        let item = Item::Primitive(Primitive::new("s".to_string(), "bad".to_string()).unwrap());
        assert!(op.push(item).is_err());
    }

    #[test]
    fn or_operator_to_item() {
        let op = OrOperator::new("".to_string()).unwrap();
        let item = op.to_item();
        assert!(matches!(item, Item::Vector(VectorItem::OrOperator(_))));
    }
}
