use crate::constants::NULL;
use crate::query::get::query::GetQuery;
use crate::response::meta::{GetMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::FilterBuffer;
use crate::{DBError, Item, Link, Primitive, Storage};

pub fn get(
    storage: &Storage,
    collection_name: &str,
    query: &GetQuery,
    buf: &mut FilterBuffer,
) -> Result<QueryResponse, DBError> {
    let mut count = 0_usize;
    if let Some(collection) = storage.get_collection(collection_name) {
        let mut found_ids: Vec<Link> = vec![];
        for id in query.get_ids()? {
            if collection.values.contains_key(id) {
                found_ids.push(id.clone());
            }
        }
        count = found_ids.len();
        buf.update(found_ids);
    };
    Ok(QueryResponse::new(
        Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?),
        Meta::GetMeta(GetMeta::new(count)),
        QueryStatus::NotFetched,
    ))
}
