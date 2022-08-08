use crate::constants::QUERY_SET;
use crate::query::delete::query::DeleteQuery;
use crate::query::find::query::FindQuery;
use crate::query::get::query::GetQuery;
use crate::query::insert::query::InsertQuery;
use crate::query::update::query::UpdateQuery;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq)]
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
