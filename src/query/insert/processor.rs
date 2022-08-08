use crate::constants::RESPONSE_IDS;
use crate::response::ids::ResponseIds;
use crate::response::meta::{InsertMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::InsertBuffer;
use crate::{DBError, Item, Storage, TySONVector, VectorItem};

pub fn insert(
    storage: &Storage,
    collection_name: String,
    items: &Vec<Item>,
    mut buf: &mut InsertBuffer,
) -> Result<QueryResponse, DBError> {
    let mut links: ResponseIds = ResponseIds::new(RESPONSE_IDS.to_string())?;
    for item in items {
        links.push(storage.insert_item(collection_name.clone(), &mut buf, item.clone())?)?;
    }
    let meta = InsertMeta::new(links.items.len());
    Ok(QueryResponse::new(
        Item::Vector(VectorItem::ResponseIds(links)),
        Meta::InsertMeta(meta),
        QueryStatus::Ready,
    ))
}
