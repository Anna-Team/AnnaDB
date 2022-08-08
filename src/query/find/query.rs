use crate::constants::FIND_QUERY;
use crate::query::operations::QueryOperation;
use crate::{DBError, Item, TySONVector, VectorItem};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FindQuery {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for FindQuery {
    fn get_prefix(&self) -> String {
        FIND_QUERY.to_string()
    }
}

impl TySONVector for FindQuery {
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
        Item::Vector(VectorItem::FindQuery(self))
    }
}

impl FindQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![
            QueryOperation::FindOperation,
            QueryOperation::UpdateOperation,
            QueryOperation::DeleteOperation,
            QueryOperation::SortOperation,
            QueryOperation::LimitOperation,
            QueryOperation::OffsetOperation,
        ]
    }

    // pub fn run(&self, storage: &Storage, collection: &Collection, mut filter_buf: &mut FilterBuffer) -> Result<Response, DBError> {
    //     let docs: VectorItem = storage.find(&collection, &self, &mut filter_buf)?;
    //     let mut data_types.md = VectorItem::new(STORAGE_VECTOR.to_string())?;
    //     for doc in docs {
    //         data_types.md.push(doc)?;
    //     };
    //     let mut data: HashMap<Primitive, Item> = HashMap::new();
    //     data.insert(
    //         Primitive::new(STRING.to_string(), "docs".to_string())?,
    //         Item::Vector(data_types.md),
    //     );
    //     Ok(Response { data })
    // }
}
