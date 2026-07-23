use serde::{Serialize, Deserialize};

use crate::constants::SET_OPERATOR;
use crate::{DBError, Item, MapItem, Primitive, TySONMap};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_operator_new() {
        let s = SetOperator::new("".to_string()).unwrap();
        assert_eq!(s.get_prefix(), "set");
    }

    #[test]
    fn set_operator_insert_and_get_items() {
        let mut s = SetOperator::new("".to_string()).unwrap();
        let k = Primitive::new("s".to_string(), "name".to_string()).unwrap();
        let v = Item::Primitive(Primitive::new("s".to_string(), "hello".to_string()).unwrap());
        s.insert(k.clone(), v.clone()).unwrap();
        let items = s.get_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], (k, v));
    }

    #[test]
    fn set_operator_to_item() {
        let s = SetOperator::new("".to_string()).unwrap();
        let item = s.to_item();
        assert!(matches!(item, Item::Map(MapItem::SetOperator(_))));
    }
}
