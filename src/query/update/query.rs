use serde::{Serialize, Deserialize};

use crate::constants::UPDATE_QUERY;
use crate::query::operations::QueryOperation;
use crate::{DBError, Item, TySONVector, VectorItem};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpdateQuery {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for UpdateQuery {
    fn get_prefix(&self) -> String {
        UPDATE_QUERY.to_string()
    }
}

impl TySONVector for UpdateQuery {
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
        Item::Vector(VectorItem::UpdateQuery(self))
    }
}

impl UpdateQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![]
    }

    // pub fn run(&self, storage: &Storage, collection: &Collection, mut insert_buf: &mut InsertBuffer, filter_buf: &FilterBuffer) -> Result<Response, DBError>{
    //     let docs = &storage.update(&collection, self, &mut insert_buf, &filter_buf)?;
    //     let mut data_types.md = VectorItem::new(STORAGE_VECTOR.to_string())?;
    //     for doc in docs {
    //         data_types.md.push(doc.clone())?;
    //     };
    //     let mut data: HashMap<Primitive, Item> = HashMap::new();
    //     data.insert(
    //         Primitive::new(STRING.to_string(), "docs".to_string())?,
    //         Item::Vector(data_types.md),
    //     );
    //     Ok(Response { data })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Primitive, Item};

    #[test]
    fn update_query_new() {
        let uq = UpdateQuery::new("".to_string()).unwrap();
        assert_eq!(uq.get_prefix(), "update");
    }

    #[test]
    fn update_query_push() {
        let mut uq = UpdateQuery::new("".to_string()).unwrap();
        let item = Item::Primitive(Primitive::new("null".to_string(), "".to_string()).unwrap());
        assert!(uq.push(item).is_ok());
        assert_eq!(uq.items.len(), 1);
    }

    #[test]
    fn update_query_next_available() {
        let uq = UpdateQuery::new("".to_string()).unwrap();
        assert!(uq.next_available().is_empty());
    }
}
