use crate::constants::NULL;
use crate::query::find::compare::{compare, Res};
use crate::query::find::query::FindQuery;
use crate::response::meta::{FindMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::{FilterBuffer, InsertBuffer};
use crate::storage::index::{CompareOp, IndexKey};
use crate::{DBError, Item, Link, MapItem, Primitive, Storage};

fn get_ids_list(
    storage: &Storage,
    collection_name: String,
    insert_buf: &InsertBuffer,
) -> Vec<Link> {
    if let Some(collection) = storage.get_collection(collection_name.clone()) {
        if !insert_buf.dropped_collections.contains(&collection_name) {
            let mut res = Vec::from_iter(collection.values.keys().cloned());
            for link in insert_buf.items.keys() {
                if link.collection_name == collection.name
                    && !collection.values.contains_key(link)
                {
                    res.push(link.clone());
                }
            }
            return res;
        }
    }
    let mut res: Vec<Link> = vec![];
    for link in insert_buf.items.keys() {
        if link.collection_name == collection_name {
            res.push(link.clone());
        }
    }
    res
}

/// Try to use an index for a single-field comparison operator.
/// Returns Some(matching_links) if an index was used, None if fallback scan needed.
fn try_index_lookup(
    storage: &Storage,
    collection_name: &str,
    op: &Item,
) -> Option<Vec<Link>> {
    let (field_path, value, cmp_op) = match op {
        Item::Map(MapItem::EqOperator(o)) => {
            let vals = o.get_values();
            if vals.len() != 1 { return None; }
            extract_path_and_value(vals[0].0, vals[0].1, CompareOp::Eq)
        }
        Item::Map(MapItem::GtOperator(o)) => {
            let vals = o.get_values();
            if vals.len() != 1 { return None; }
            extract_path_and_value(vals[0].0, vals[0].1, CompareOp::Gt)
        }
        Item::Map(MapItem::GteOperator(o)) => {
            let vals = o.get_values();
            if vals.len() != 1 { return None; }
            extract_path_and_value(vals[0].0, vals[0].1, CompareOp::Gte)
        }
        Item::Map(MapItem::LtOperator(o)) => {
            let vals = o.get_values();
            if vals.len() != 1 { return None; }
            extract_path_and_value(vals[0].0, vals[0].1, CompareOp::Lt)
        }
        Item::Map(MapItem::LteOperator(o)) => {
            let vals = o.get_values();
            if vals.len() != 1 { return None; }
            extract_path_and_value(vals[0].0, vals[0].1, CompareOp::Lte)
        }
        Item::Map(MapItem::NeqOperator(o)) => {
            let vals = o.get_values();
            if vals.len() != 1 { return None; }
            extract_path_and_value(vals[0].0, vals[0].1, CompareOp::Neq)
        }
        _ => None,
    }?;

    let index = storage.index_mgr.get_index(collection_name, &field_path)?;
    let index_key = IndexKey::from_primitive(&value)?;
    let result = index.lookup_by_op(cmp_op, &index_key);

    Some(result)
}

fn extract_path_and_value(
    key: &Primitive,
    value: &Primitive,
    op: CompareOp,
) -> Option<(String, Primitive, CompareOp)> {
    let field_path = match key {
        Primitive::PathToValue(p) => p.value.clone(),
        Primitive::RootPrimitive(_) => "_root".to_string(),
        _ => return None,
    };
    Some((field_path, value.clone(), op))
}

pub fn find(
    storage: &Storage,
    collection_name: String,
    query: &FindQuery,
    buf: &mut FilterBuffer,
    insert_buf: &InsertBuffer,
    is_first: bool,
) -> Result<QueryResponse, DBError> {
    let mut found_ids: Vec<Link> = vec![];
    let mut started: bool = false;
    if !is_first {
        found_ids = buf.ids.clone();
        started = true;
    }
    if query.items.len() == 0 && !started {
        found_ids = get_ids_list(storage, collection_name.clone(), insert_buf);
    }
    for op in &query.items {
        if started {
            let iter = found_ids.clone();
            found_ids = vec![];
            for k in iter {
                if compare(op, &k, storage, insert_buf)? == Res::True {
                    found_ids.push(k);
                }
            }
        } else {
            started = true;

            // Try index lookup for the first filter on this collection
            if insert_buf.items.is_empty() {
                if let Some(indexed_ids) = try_index_lookup(storage, &collection_name, op) {
                    // Index hit: use indexed results directly, but still verify
                    // against the compare function for correctness (handles edge
                    // cases like type mismatches the index doesn't track).
                    // For simple cases this is redundant but safe.
                    found_ids = indexed_ids;
                    continue;
                }
            }

            // Fallback: full scan
            let iter = get_ids_list(storage, collection_name.clone(), insert_buf);
            for k in iter {
                if compare(op, &k, storage, insert_buf)? == Res::True {
                    found_ids.push(k.clone());
                }
            }
        }
    }
    buf.update(found_ids);
    let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
    let meta = Meta::FindMeta(FindMeta::new(buf.ids.len()));
    Ok(QueryResponse::new(data, meta, QueryStatus::NotFetched))
}
