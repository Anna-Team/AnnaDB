use crate::constants::NULL;
use crate::query::find::compare::{compare, Res};
use crate::query::find::query::FindQuery;
use crate::response::meta::{FindMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::{FilterBuffer, InsertBuffer};
use crate::{DBError, Item, Link, Primitive, Storage};
use std::collections::HashSet;

fn get_ids_list(
    storage: &Storage,
    collection_name: String,
    insert_buf: &InsertBuffer,
) -> Vec<Link> {
    let pot_collection = storage.get_collection(collection_name.clone());
    if pot_collection.is_some() && !insert_buf.dropped_collections.contains(&collection_name) {
        let collection = pot_collection.unwrap(); // TODO ugly
        let mut res = Vec::from_iter(collection.values.keys().cloned());
        for link in insert_buf.items.keys() {
            if link.collection_name == collection.name && !collection.values.contains_key(link) {
                res.push(link.clone());
            }
        }
        res
    } else {
        let mut res: Vec<Link> = vec![];
        for link in insert_buf.items.keys() {
            if link.collection_name == collection_name {
                res.push(link.clone());
            }
        }
        res
    }
}

pub fn find(
    storage: &Storage,
    collection_name: String,
    query: &FindQuery,
    buf: &mut FilterBuffer,
    insert_buf: &InsertBuffer,
    is_first: bool,
) -> Result<QueryResponse, DBError> {
    let mut found_ids: HashSet<Link> = HashSet::new();
    let mut started: bool = false;
    if !is_first {
        found_ids = buf.ids.iter().cloned().collect();
        started = true;
    }
    if query.items.len() == 0 && !started {
        found_ids = get_ids_list(storage, collection_name.clone(), insert_buf)
            .iter()
            .cloned()
            .collect();
    }
    for op in &query.items {
        if started {
            let iter = found_ids.clone();
            found_ids = compare(op, &iter, storage, insert_buf)?;
        } else {
            started = true;

            let iter = get_ids_list(storage, collection_name.clone(), insert_buf)
                .iter()
                .cloned()
                .collect();
            found_ids = compare(op, &iter, storage, insert_buf)?;
        }
    }
    buf.update(found_ids.iter().cloned().collect());
    let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
    let meta = Meta::FindMeta(FindMeta::new(buf.ids.len()));
    Ok(QueryResponse::new(data, meta, QueryStatus::NotFetched))
}
