use serde::{Serialize, Deserialize};

use crate::constants::QUERY_SET;
use crate::query::delete::query::DeleteQuery;
use crate::query::find::query::FindQuery;
use crate::query::get::query::GetQuery;
use crate::query::insert::query::InsertQuery;
use crate::query::update::query::UpdateQuery;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QuerySet {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for QuerySet {
    fn get_prefix(&self) -> String {
        QUERY_SET.to_string()
    }
}

impl TySONVector for QuerySet {
    fn new(_: String) -> Result<Self, DBError> {
        Ok(Self { items: vec![] })
    }

    fn push(&mut self, item: Item) -> Result<bool, DBError> {
        self.items.push(item);
        Ok(true)
    }

    fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    fn to_item(self) -> Item {
        Item::Vector(VectorItem::QueriesVector(self))
    }
}

impl From<InsertQuery> for QuerySet {
    fn from(q: InsertQuery) -> Self {
        Self {
            items: vec![q.to_item()],
        }
    }
}

impl From<GetQuery> for QuerySet {
    fn from(q: GetQuery) -> Self {
        Self {
            items: vec![q.to_item()],
        }
    }
}

impl From<FindQuery> for QuerySet {
    fn from(q: FindQuery) -> Self {
        Self {
            items: vec![q.to_item()],
        }
    }
}

impl From<UpdateQuery> for QuerySet {
    fn from(q: UpdateQuery) -> Self {
        Self {
            items: vec![q.to_item()],
        }
    }
}

impl From<DeleteQuery> for QuerySet {
    fn from(q: DeleteQuery) -> Self {
        Self {
            items: vec![q.to_item()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::delete::query::DeleteQuery;
    use crate::query::get::query::GetQuery;
    use crate::query::update::query::UpdateQuery;
    use crate::tyson::primitive::TySONPrimitive;
    use crate::tyson::vector::TySONVector;
    use crate::Primitive;
    use crate::Item;

    #[test]
    fn query_set_from_insert() {
        let iq = InsertQuery::new("".to_string()).unwrap();
        let qs = QuerySet::from(iq);
        assert_eq!(qs.items.len(), 1);
    }

    #[test]
    fn query_set_from_find() {
        let fq = FindQuery::new("".to_string()).unwrap();
        let qs = QuerySet::from(fq);
        assert_eq!(qs.items.len(), 1);
    }

    #[test]
    fn query_set_from_get() {
        let gq = GetQuery::new("".to_string()).unwrap();
        let qs = QuerySet::from(gq);
        assert_eq!(qs.items.len(), 1);
    }

    #[test]
    fn query_set_from_update() {
        let uq = UpdateQuery::new("".to_string()).unwrap();
        let qs = QuerySet::from(uq);
        assert_eq!(qs.items.len(), 1);
    }

    #[test]
    fn query_set_from_delete() {
        let dq = DeleteQuery::new("".to_string(), "".to_string()).unwrap();
        let qs = QuerySet::from(dq);
        assert_eq!(qs.items.len(), 1);
    }

    #[test]
    fn query_set_new() {
        let qs = QuerySet::new("".to_string()).unwrap();
        assert_eq!(qs.get_prefix(), "q");
    }

    #[test]
    fn query_set_push() {
        let mut qs = QuerySet::new("".to_string()).unwrap();
        let item = Item::Primitive(Primitive::new("null".to_string(), "".to_string()).unwrap());
        assert!(qs.push(item).is_ok());
        assert_eq!(qs.items.len(), 1);
    }
}
