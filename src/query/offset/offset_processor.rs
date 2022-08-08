use crate::constants::NULL;
use crate::query::offset::offset_query::OffsetQuery;
use crate::response::meta::{FindMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::FilterBuffer;
use crate::{DBError, Item, Primitive};

pub fn offset(query: &OffsetQuery, mut buf: &mut FilterBuffer) -> Result<QueryResponse, DBError> {
    match query.get_value() {
        Item::Primitive(Primitive::NumberPrimitive(n)) => {
            let number = n.get_value() as usize;
            if number <= buf.ids.len() {
                buf.ids = buf.ids.as_slice()[number..].to_vec();
            } else {
                buf.ids = vec![];
            }
        }
        _ => {}
    }
    let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
    let meta = Meta::FindMeta(FindMeta::new(buf.ids.len()));
    Ok(QueryResponse::new(data, meta, QueryStatus::NotFetched))
}
