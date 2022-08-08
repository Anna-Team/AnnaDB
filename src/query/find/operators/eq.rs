use crate::constants::EQ_OPERATOR;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, TySONMap};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EqOperator {
    values: Vec<(Primitive, Primitive)>,
}

impl BaseTySONItemInterface for EqOperator {
    fn get_prefix(&self) -> String {
        EQ_OPERATOR.to_string()
    }
}

impl TySONMap for EqOperator {
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
            _ => Err(DBError::new("Eq primitive can contain only primitives")),
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
        Item::Map(MapItem::EqOperator(self))
    }
}

impl EqOperator {
    pub fn get_values(&self) -> Vec<(&Primitive, &Primitive)> {
        let mut ve: Vec<(&Primitive, &Primitive)> = vec![];
        for (k, v) in &self.values {
            ve.push((k, v));
        }
        ve
    }
}
