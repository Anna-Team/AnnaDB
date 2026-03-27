use serde::{Serialize, Deserialize};

use crate::constants::LT_OPERATOR;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, TySONMap};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LtOperator {
    values: Vec<(Primitive, Primitive)>,
}

impl BaseTySONItemInterface for LtOperator {
    fn get_prefix(&self) -> String {
        LT_OPERATOR.to_string()
    }
}

impl TySONMap for LtOperator {
    fn new(_: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self { values: vec![] })
    }

    fn insert(&mut self, k: Primitive, v: Item) -> Result<bool, DBError> {
        match v {
            Item::Primitive(o) => {
                self.values.push((k, o));
                Ok(true)
            }
            _ => Err(DBError::TypeMismatch("lt operator can contain only primitives".to_string())),
        }
    }

    fn get_items(&self) -> Vec<(Primitive, Item)> {
        let mut ve: Vec<(Primitive, Item)> = vec![];
        for (k, v) in &self.values {
            ve.push((k.clone(), Item::Primitive(v.clone())));
        }
        ve
    }

    fn to_item(self) -> Item {
        Item::Map(MapItem::LtOperator(self))
    }
}

impl LtOperator {
    pub fn get_values(&self) -> Vec<(&Primitive, &Primitive)> {
        let mut ve: Vec<(&Primitive, &Primitive)> = vec![];
        for (k, v) in &self.values {
            ve.push((k, v));
        }
        ve
    }
}
