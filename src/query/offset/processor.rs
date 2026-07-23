use crate::constants::NULL;
use crate::query::offset::query::OffsetQuery;
use crate::response::meta::{FindMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::FilterBuffer;
use crate::{DBError, Item, Primitive};

pub fn offset(query: &OffsetQuery, buf: &mut FilterBuffer) -> Result<QueryResponse, DBError> {
    match query.get_value() {
        Item::Primitive(Primitive::NumberPrimitive(n)) => {
            let number = n.get_value() as usize;
            if number <= buf.ids.len() {
                buf.ids = buf.ids.as_slice()[number..].to_vec();
            } else {
                buf.ids = vec![];
            }
        }
        _ => return Err(DBError::TypeMismatch("offset expects a number".to_string())),
    }
    let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
    let meta = Meta::FindMeta(FindMeta::new(buf.ids.len()));
    Ok(QueryResponse::new(data, meta, QueryStatus::NotFetched))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::offset::query::OffsetQuery;
    use crate::tyson::modifier::TySONModifier;
    use crate::data_types::primitives::number::NumberPrimitive;
    use crate::tyson::primitive::TySONPrimitive;
    use crate::Link;

    #[test]
    fn offset_processor_skips() {
        let num = NumberPrimitive::new("".to_string(), "1".to_string()).unwrap();
        let num_item = Item::Primitive(Primitive::NumberPrimitive(num));
        let query = OffsetQuery::new("".to_string(), num_item).unwrap();
        let mut buf = FilterBuffer::new();
        buf.ids = vec![
            Link::create("test".to_string()),
            Link::create("test".to_string()),
            Link::create("test".to_string()),
        ];
        let _resp = offset(&query, &mut buf).unwrap();
        assert_eq!(buf.ids.len(), 2);
    }

    #[test]
    fn offset_processor_empties_when_beyond() {
        let num = NumberPrimitive::new("".to_string(), "10".to_string()).unwrap();
        let num_item = Item::Primitive(Primitive::NumberPrimitive(num));
        let query = OffsetQuery::new("".to_string(), num_item).unwrap();
        let mut buf = FilterBuffer::new();
        buf.ids = vec![Link::create("test".to_string())];
        let _resp = offset(&query, &mut buf).unwrap();
        assert!(buf.ids.is_empty());
    }
}
