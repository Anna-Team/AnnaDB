use crate::constants::NULL;
use crate::query::find::compare::{compare, Res};
use crate::query::find::operators::and::AndOperator;
use crate::query::find::query::FindQuery;
use crate::response::meta::{FindMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::{FilterBuffer, InsertBuffer};
use crate::storage::index::{CompareOp, IndexKey};
use crate::storage::vector::hnsw::HnswMetric;
use crate::tyson::vector::TySONVector;
use crate::{DBError, Item, Link, MapItem, Primitive, Storage, VectorItem};

fn try_knn_lookup(
    storage: &Storage,
    collection_name: &str,
    op: &Item,
) -> Option<Vec<Link>> {
    match op {
        Item::Map(MapItem::KnnOperator(knn)) => {
            let field_path = knn.get_field();
            if field_path.is_empty() {
                return None;
            }
            let vec_index = storage.index_mgr.get_vector_index(collection_name, field_path)?;
            let embedding = knn.get_query_embedding();
            let metric = match knn.get_metric() {
                "euclidean" => HnswMetric::Euclidean,
                "dot" => HnswMetric::DotProduct,
                _ => HnswMetric::Cosine,
            };
            if vec_index.metric != metric {
                return None;
            }
            let k = knn.get_k();
            let results = vec_index.search(embedding.values(), k);
            Some(results)
        }
        _ => None,
    }
}

/// If `op` is an AND/OR containing a knn sub-expression plus other filters,
/// run the vector search first and return (candidates, remaining_filter).
fn try_hybrid_candidates(
    storage: &Storage,
    collection_name: &str,
    op: &Item,
) -> Option<(Vec<Link>, Option<Item>)> {
    // Standalone knn
    if let Some(links) = try_knn_lookup(storage, collection_name, op) {
        return Some((links, None));
    }

    // AND[knn, eq{...}] — extract knn, get candidates, return remaining filter
    match op {
        Item::Vector(VectorItem::AndOperator(and_op)) => {
            let items = and_op.get_items();
            let mut knn_idx: Option<usize> = None;
            for (i, sub) in items.iter().enumerate() {
                if matches!(sub, Item::Map(MapItem::KnnOperator(_))) {
                    knn_idx = Some(i);
                    break;
                }
            }
            if let Some(idx) = knn_idx {
                let knn_item = &items[idx];
                let links = try_knn_lookup(storage, collection_name, knn_item)?;
                let remaining: Vec<&Item> = items
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != idx)
                    .map(|(_, item)| item)
                    .collect();
                let filter = match remaining.len() {
                    0 => None,
                    1 => Some(remaining[0].clone()),
                    _ => {
                        let mut a = AndOperator::new("".to_string()).ok()?;
                        for item in remaining {
                            a.push(item.clone()).ok()?;
                        }
                        Some(a.to_item())
                    }
                };
                return Some((links, filter));
            }
            None
        }
        _ => None,
    }
}

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

            // Try hybrid (knn + structural) or standalone knn lookup
            if insert_buf.items.is_empty() {
                if let Some((hybrid_ids, remaining_filter)) =
                    try_hybrid_candidates(storage, &collection_name, op)
                {
                    if let Some(filter_op) = remaining_filter {
                        // knn narrowed candidates, now apply structural filter
                        let mut filtered = Vec::new();
                        for k in hybrid_ids {
                            if compare(&filter_op, &k, storage, insert_buf)? == Res::True {
                                filtered.push(k);
                            }
                        }
                        found_ids = filtered;
                    } else {
                        found_ids = hybrid_ids;
                    }
                    continue;
                }
            }

            // Try index lookup for the first filter on this collection
            if insert_buf.items.is_empty() {
                if let Some(indexed_ids) = try_index_lookup(storage, &collection_name, op) {
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
