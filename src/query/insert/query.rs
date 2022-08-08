use crate::constants::INSERT_QUERY;
use crate::query::operations::QueryOperation;
use crate::{DBError, Item, TySONVector, VectorItem};

use crate::tyson::item::BaseTySONItemInterface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsertQuery {
    pub(crate) items: Vec<Item>,
}

impl BaseTySONItemInterface for InsertQuery {
    fn get_prefix(&self) -> String {
        INSERT_QUERY.to_string()
    }
}

impl TySONVector for InsertQuery {
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
        Item::Vector(VectorItem::InsertQuery(self))
    }
}

impl InsertQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![]
    }

    // pub fn run(&self, storage: &Storage, collection: &Collection, mut insert_buf: &mut InsertBuffer) -> Result<Response, DBError> {
    //     let links = storage.insert(collection, &self.data_types.md, &mut insert_buf)?;
    //     let mut data_types.md = VectorItem::new(STORAGE_VECTOR.to_string())?;
    //     for link in links {
    //         data_types.md.push(Item::Primitive(Primitive::Link(link)))?;
    //     };
    //     let mut data: HashMap<Primitive, Item> = HashMap::new();
    //     data.insert(
    //         Primitive::StringPrimitive(StringPrimitive::new("".to_string(), "ids".to_string())?),
    //         Item::Vector(data_types.md),
    //     );
    //     Ok(Response { data })
    // }
}
