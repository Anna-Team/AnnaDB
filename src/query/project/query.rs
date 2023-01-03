use crate::constants::PROJECT_QUERY;
use crate::data_types::map::storage::StorageMap;
use crate::query::operations::QueryOperation;
use crate::query::project::processor::resolve;
use crate::storage::projection::Projection;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, TySONMap, TySONVector, VectorItem};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectQuery {
    pub(crate) values: Vec<(Primitive, Item)>,
}

impl BaseTySONItemInterface for ProjectQuery {
    fn get_prefix(&self) -> String {
        PROJECT_QUERY.to_string()
    }
}

impl TySONMap for ProjectQuery {
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
        Item::Map(MapItem::ProjectQuery(self))
    }
}

impl ProjectQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![]
    }
}
