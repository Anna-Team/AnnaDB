use crate::constants::SET_OPERATOR;
use crate::{DBError, Item, MapItem, Primitive, TySONMap};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetOperator {
    pub(crate) values: Vec<(Primitive, Item)>,
}

impl BaseTySONItemInterface for SetOperator {
    fn get_prefix(&self) -> String {
        SET_OPERATOR.to_string()
    }
}

impl TySONMap for SetOperator {
    fn new(_: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self { values: vec![] })
    }

    fn insert(&mut self, k: Primitive, v: Item) -> Result<bool, DBError> {
        self.values.push((k, v));
        Ok(true)
    }

    fn get_items(&self) -> Vec<(Primitive, Item)> {
        let mut ve: Vec<(Primitive, Item)> = vec![];
        for (k, v) in &self.values {
            ve.push((k.clone(), v.clone()));
        }
        ve
    }

    fn to_item(self) -> Item {
        Item::Map(MapItem::SetOperator(self))
    }
}
