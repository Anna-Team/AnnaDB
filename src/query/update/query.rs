use crate::constants::UPDATE_QUERY;
use crate::query::operations::QueryOperation;
use crate::{DBError, Item, TySONVector, VectorItem};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq)]
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
