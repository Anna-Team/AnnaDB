use serde::{Serialize, Deserialize};

use crate::constants::PROJECT_QUERY;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, MapItem, Primitive, TySONMap};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_query_new() {
        let pq = ProjectQuery::new("".to_string()).unwrap();
        assert!(pq.values.is_empty());
        assert_eq!(pq.get_prefix(), "project");
    }

    #[test]
    fn project_query_insert() {
        let mut pq = ProjectQuery::new("".to_string()).unwrap();
        let k = Primitive::new("s".to_string(), "name".to_string()).unwrap();
        let v = Item::Primitive(Primitive::new("s".to_string(), "value".to_string()).unwrap());
        pq.insert(k, v).unwrap();
        assert_eq!(pq.values.len(), 1);
    }

    #[test]
    fn project_query_get_items() {
        let mut pq = ProjectQuery::new("".to_string()).unwrap();
        let k = Primitive::new("s".to_string(), "key".to_string()).unwrap();
        let v = Item::Primitive(Primitive::new("n".to_string(), "1".to_string()).unwrap());
        pq.insert(k.clone(), v.clone()).unwrap();
        let items = pq.get_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], (k, v));
    }

    #[test]
    fn project_query_to_item() {
        let pq = ProjectQuery::new("".to_string()).unwrap();
        let item = pq.to_item();
        assert!(matches!(item, Item::Map(MapItem::ProjectQuery(_))));
    }

    #[test]
    fn project_query_next_available() {
        let pq = ProjectQuery::new("".to_string()).unwrap();
        assert!(pq.next_available().is_empty());
    }
}
