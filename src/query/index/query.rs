use crate::constants::INDEX_QUERY;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, TySONMap};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexQuery {
    pub(crate) values: Vec<(Primitive, Item)>,
}

impl BaseTySONItemInterface for IndexQuery {
    fn get_prefix(&self) -> String {
        INDEX_QUERY.to_string()
    }
}

impl TySONMap for IndexQuery {
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
        Item::Map(MapItem::IndexQuery(self))
    }
}

impl IndexQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![]
    }
}
