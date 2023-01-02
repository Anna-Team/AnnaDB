use crate::constants::INTERNAL_COLLECTION_NAME;
use crate::query::update::query::UpdateQuery;
use crate::response::ids::ResponseIds;
use crate::response::meta::{Meta, UpdateMeta};
use crate::response::{QueryResponse, QueryStatus};
use crate::{DBError, Item, Link, MapItem, Primitive, Storage, TySONMap, TySONVector, VectorItem};
use std::collections::HashSet;

use crate::storage::buffer::{FilterBuffer, InsertBuffer};

use crate::storage::main::{FoundItem, FoundRootItem, FoundSubItem};

fn update_sub_item(
    storage: &Storage,
    found_item: FoundSubItem,
    insert_buf: &mut InsertBuffer,
    inserted_value: Item,
) -> Result<bool, DBError> {
    let inserted_id = storage.insert_item(
        INTERNAL_COLLECTION_NAME.to_string(),
        insert_buf,
        inserted_value,
    )?;
    match found_item.container_value {
        Item::Map(MapItem::StorageMap(updated_map)) => {
            let mut res_map = updated_map;
            res_map.replace_by_string(found_item.key, inserted_id)?;
            insert_buf.insert(found_item.container_id, res_map.to_item());
        }
        Item::Vector(VectorItem::StorageVector(updated_vec)) => {
            let mut res_vec = updated_vec;
            res_vec.replace_by_string(found_item.key, inserted_id)?;
            insert_buf.insert(found_item.container_id, res_vec.to_item());
        }
        _ => {}
    }
    Ok(true)
}

fn update_root_item(
    found_item: FoundRootItem,
    insert_buf: &mut InsertBuffer,
    inserted_value: Item,
) -> Result<bool, DBError> {
    insert_buf.insert(found_item.id, inserted_value);
    Ok(true)
}

fn update_item(
    storage: &Storage,
    found_item: FoundItem,
    insert_buf: &mut InsertBuffer,
    inserted_value: Item,
) -> Result<bool, DBError> {
    match found_item {
        FoundItem::FoundRootItem(i) => update_root_item(i, insert_buf, inserted_value),
        FoundItem::FoundSubItem(i) => update_sub_item(storage, i, insert_buf, inserted_value),
    }
}

fn manage_operator(
    op: &MapItem,
    storage: &Storage,
    insert_buf: &mut InsertBuffer,
    val: Item,
    found_item: FoundItem,
) -> Result<bool, DBError> {
    match op {
        MapItem::SetOperator(_) => {
            update_item(storage, found_item, insert_buf, val.clone())?;
        }
        MapItem::IncOperator(_) => match &found_item.get_value() {
            Some(Item::Primitive(Primitive::NumberPrimitive(found_val))) => match val {
                Item::Primitive(Primitive::NumberPrimitive(adding_val)) => {
                    let inserted_value =
                        Item::Primitive(Primitive::NumberPrimitive(found_val.add(&adding_val)));
                    update_item(storage, found_item, insert_buf, inserted_value)?;
                }
                _ => {
                    return Err(DBError::new("Inc operator supports numbers only"));
                }
            },
            _ => {}
        },
        _ => {
            return Err(DBError::new("Unsupported update operation"));
        }
    }

    Ok(true)
}

fn get_value(
    pr: &Primitive,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<Option<FoundItem>, DBError> {
    match pr {
        Primitive::PathToValue(path) => {
            let found_item = storage.get_value_by_path(path.clone(), id.clone(), insert_buf)?;
            match found_item {
                Some(mut i) => {
                    if let Some(item_from_buf) = insert_buf.items.get(&i.container_id) {
                        i.container_value = item_from_buf.clone();
                    }
                    Ok(Some(FoundItem::FoundSubItem(i)))
                }
                _ => Ok(None),
            }
        }
        Primitive::RootPrimitive(_) => {
            let item = storage.get_item_by_link(id, insert_buf, 0, None)?;
            Ok(Some(FoundItem::FoundRootItem(FoundRootItem {
                id: id.clone(),
                value: item,
            })))
        }
        _ => Ok(None),
    }
}

fn process(
    op: &MapItem,
    storage: &Storage,
    filter_buf: &FilterBuffer,
    insert_buf: &mut InsertBuffer,
) -> Result<HashSet<Link>, DBError> {
    // Process update
    let mut updated: bool;
    let mut result: HashSet<Link> = HashSet::new();
    for id in &filter_buf.ids {
        updated = false;
        for (k, v) in op.get_items() {
            let val = get_value(&k, id, storage, insert_buf)?;
            match val {
                Some(o) => {
                    updated = true;
                    manage_operator(op, storage, insert_buf, v, o)?;
                }
                _ => {}
            }
        }
        if updated {
            result.insert(id.clone());
        }
    }
    Ok(result)
}

pub(crate) fn update(
    storage: &Storage,
    query: &UpdateQuery,
    mut insert_buf: &mut InsertBuffer,
    filter_buf: &FilterBuffer,
) -> Result<QueryResponse, DBError> {
    let mut result: HashSet<Link> = HashSet::new();
    for item in query.get_items() {
        match item {
            Item::Map(op) => {
                result.extend(process(op, storage, &filter_buf, &mut insert_buf)?);
            }
            _ => return Err(DBError::new("Unexpected update operator")),
        }
    }
    let meta = Meta::UpdateMeta(UpdateMeta::new(result.len()));
    let data = Item::from(VectorItem::ResponseIds(ResponseIds::from(result)));
    Ok(QueryResponse::new(data, meta, QueryStatus::Ready))
}
