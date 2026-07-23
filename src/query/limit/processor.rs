use crate::constants::NULL;
use crate::query::limit::query::LimitQuery;
use crate::response::meta::{FindMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::FilterBuffer;
use crate::{DBError, Item, Primitive};

pub fn limit(query: &LimitQuery, buf: &mut FilterBuffer) -> Result<QueryResponse, DBError> {
    match query.get_value() {
        Item::Primitive(Primitive::NumberPrimitive(n)) => {
            let number = n.get_value() as usize;
            if number <= buf.ids.len() {
                buf.ids = buf.ids.as_slice()[..number].to_vec();
            }
        }
        _ => return Err(DBError::TypeMismatch("limit expects a number".to_string())),
    }
    let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
    let meta = Meta::FindMeta(FindMeta::new(buf.ids.len()));
    Ok(QueryResponse::new(data, meta, QueryStatus::NotFetched))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::limit::query::LimitQuery;
    use crate::tyson::modifier::TySONModifier;
    use crate::data_types::primitives::number::NumberPrimitive;
    use crate::tyson::primitive::TySONPrimitive;
    use crate::Link;

    #[test]
    fn limit_construction_rejects_non_number() {
        let limit_expr = Item::Primitive(
            Primitive::new("s".to_string(), "not_a_number".to_string()).unwrap(),
        );
        let query = LimitQuery::new("".to_string(), limit_expr);
        assert!(query.is_err(), "Limit with string value should fail at construction");
    }

    #[test]
    fn limit_processor_trims_buffer() {
        let num = NumberPrimitive::new("".to_string(), "2".to_string()).unwrap();
        let num_item = Item::Primitive(Primitive::NumberPrimitive(num));
        let query = LimitQuery::new("".to_string(), num_item).unwrap();
        let mut buf = FilterBuffer::new();
        buf.ids = vec![
            Link::create("test".to_string()),
            Link::create("test".to_string()),
            Link::create("test".to_string()),
        ];
        let resp = limit(&query, &mut buf).unwrap();
        assert_eq!(buf.ids.len(), 2);
    }

    #[test]
    fn limit_processor_noop_when_limit_larger() {
        let num = NumberPrimitive::new("".to_string(), "10".to_string()).unwrap();
        let num_item = Item::Primitive(Primitive::NumberPrimitive(num));
        let query = LimitQuery::new("".to_string(), num_item).unwrap();
        let mut buf = FilterBuffer::new();
        let links = vec![
            Link::create("test".to_string()),
            Link::create("test".to_string()),
        ];
        buf.ids = links.clone();
        let _resp = limit(&query, &mut buf).unwrap();
        assert_eq!(buf.ids.len(), 2);
    }
}
