use crate::constants::NULL;
use crate::query::get::query::GetQuery;
use crate::response::meta::{GetMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::FilterBuffer;
use crate::{DBError, Item, Link, Primitive, Storage};

pub fn get(
    storage: &Storage,
    collection_name: String,
    query: &GetQuery,
    buf: &mut FilterBuffer,
) -> Result<QueryResponse, DBError> {
    let mut count = 0 as usize;
    match storage.get_collection(collection_name) {
        Some(collection) => {
            let mut found_ids: Vec<Link> = vec![];
            for id in query.get_ids()? {
                match collection.values.get(id) {
                    Some(_) => {
                        found_ids.push(id.clone());
                    }
                    None => {}
                }
            }
            count = found_ids.len();
            buf.update(found_ids);
        }
        None => {}
    };
    Ok(QueryResponse::new(
        Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?),
        Meta::GetMeta(GetMeta::new(count)),
        QueryStatus::NotFetched,
    ))
}
