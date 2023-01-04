use crate::data_types::modifier::ModifierItem;
use crate::{DBError, Item, Link, MapItem, Primitive, Storage, TySONVector, VectorItem};
use std::collections::HashSet;

use crate::storage::buffer::InsertBuffer;

#[derive(PartialEq, Debug)]
pub enum CompareSign {
    Eq,
    NEq,
    GT,
    GTE,
    LT,
    LTE,
}

#[derive(PartialEq, Debug)]
pub enum CompareResult {
    Equal,
    Greater,
    Less,
    CanNotCompare,
}

#[derive(PartialEq)]
pub enum Res {
    True,
    False,
    None,
}

fn prepare_item(
    item: &Primitive,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<Option<Primitive>, DBError> {
    match item {
        Primitive::PathToValue(o) => {
            match storage.find_sub_item_by_path(o.clone(), id.clone(), insert_buf)? {
                Some(i) => {
                    match &i.value {
                        Some(Item::Primitive(val)) => {
                            Ok(prepare_item(val, id, storage, insert_buf)?)
                        }
                        // _ => { Err(DBError::new("Can not compare primitive and container")) }
                        _ => Ok(None),
                    }
                }
                None => Ok(None),
            }
        }
        Primitive::RootPrimitive(_) => {
            let val = storage.get_value_by_link(id, None)?;
            match val {
                Item::Primitive(prim_val) => Ok(prepare_item(&prim_val, id, storage, insert_buf)?),
                // _ => { Err(DBError::new("Can not compare primitive and container")) }
                _ => Ok(None),
            }
        }
        _ => Ok(Some(item.clone())),
    }
}

fn compare_primitives(
    left: &Primitive,
    right: &Primitive,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<CompareResult, DBError> {
    let new_l = prepare_item(left, id, storage, insert_buf)?;
    let new_r = prepare_item(right, id, storage, insert_buf)?;
    match (new_l, new_r) {
        (_, None) => Ok(CompareResult::CanNotCompare),
        (None, _) => Ok(CompareResult::CanNotCompare),
        (Some(l), Some(r)) => {
            return if l.get_prefix() != r.get_prefix() {
                Ok(CompareResult::CanNotCompare)
            } else if l == r {
                Ok(CompareResult::Equal)
            } else if l > r {
                Ok(CompareResult::Greater)
            } else if l < r {
                Ok(CompareResult::Less)
            } else {
                Ok(CompareResult::CanNotCompare)
            };
        }
    }
}

fn compare_sets_using_full_scan(
    links: &HashSet<Link>,
    left: &Primitive,
    right: &Primitive,
    sigh: CompareSign,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<HashSet<Link>, DBError> {
    let mut res: HashSet<Link> = HashSet::new();
    for link in links {
        let compare_res = compare_primitives(left, right, link, storage, insert_buf)?;
        if sigh == CompareSign::Eq {
            if compare_res == CompareResult::Equal {
                res.insert(link.clone());
            }
        } else if sigh == CompareSign::NEq {
            if compare_res != CompareResult::Equal {
                res.insert(link.clone());
            }
        } else if sigh == CompareSign::GT {
            if compare_res == CompareResult::Greater {
                res.insert(link.clone());
            }
        } else if sigh == CompareSign::GTE {
            if compare_res == CompareResult::Greater || compare_res == CompareResult::Equal {
                res.insert(link.clone());
            }
        } else if sigh == CompareSign::LT {
            if compare_res == CompareResult::Less {
                res.insert(link.clone());
            }
        } else if sigh == CompareSign::LTE {
            if compare_res == CompareResult::Less || compare_res == CompareResult::Equal {
                res.insert(link.clone());
            }
        }
    }
    Ok(res)
}

fn compare_sets_using_indexes(
    links: &HashSet<Link>,
    left: &Primitive,
    right: &Primitive,
    sigh: CompareSign,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<HashSet<Link>, DBError> {
    let mut res: HashSet<Link> = HashSet::new();
    match (left, right) {
        (Primitive::PathToValue(l), Primitive::PathToValue(r)) => {
            res = compare_sets_using_full_scan(links, left, right, sigh, storage, insert_buf)?;
        }
        (Primitive::PathToValue(l), Primitive::RootPrimitive(r)) => {
            res = compare_sets_using_full_scan(links, left, right, sigh, storage, insert_buf)?;
        }
        (Primitive::RootPrimitive(l), Primitive::PathToValue(r)) => {
            res = compare_sets_using_full_scan(links, left, right, sigh, storage, insert_buf)?;
        }
        (Primitive::RootPrimitive(l), Primitive::RootPrimitive(r)) => {
            res = compare_sets_using_full_scan(links, left, right, sigh, storage, insert_buf)?;
        }
        (Primitive::RootPrimitive(l), r) => {
            res = compare_sets_using_full_scan(links, left, right, sigh, storage, insert_buf)?;
        }
        (Primitive::PathToValue(l), r) => {
            res = compare_sets_using_full_scan(links, left, right, sigh, storage, insert_buf)?;
        }
        (l, r) => {
            res = compare_sets_using_full_scan(links, left, right, sigh, storage, insert_buf)?;
        }
    }
    Ok(res)
}

fn check_bool(
    op: &Item,
    links: &HashSet<Link>,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<HashSet<Link>, DBError> {
    match op {
        Item::Primitive(Primitive::BoolPrimitive(o)) => {
            if o.val() {
                return Ok(links.clone());
            } else {
                return Ok(HashSet::new());
            }
        }
        _ => compare(op, links, storage, insert_buf),
    }
}

pub fn compare(
    op: &Item,
    links: &HashSet<Link>,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<HashSet<Link>, DBError> {
    println!("{:?}", op);
    match op {
        Item::Map(MapItem::EqOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for (left, right) in o.get_values() {
                let buf = compare_sets_using_indexes(
                    links,
                    left,
                    right,
                    CompareSign::Eq,
                    storage,
                    insert_buf,
                )?;
                if started {
                    res = res.intersection(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Map(MapItem::NeqOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for (left, right) in o.get_values() {
                let buf = compare_sets_using_indexes(
                    links,
                    left,
                    right,
                    CompareSign::NEq,
                    storage,
                    insert_buf,
                )?;
                if started {
                    res = res.intersection(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Map(MapItem::GtOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for (left, right) in o.get_values() {
                let buf = compare_sets_using_indexes(
                    links,
                    left,
                    right,
                    CompareSign::GT,
                    storage,
                    insert_buf,
                )?;
                if started {
                    res = res.intersection(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Map(MapItem::GteOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for (left, right) in o.get_values() {
                let buf = compare_sets_using_indexes(
                    links,
                    left,
                    right,
                    CompareSign::GTE,
                    storage,
                    insert_buf,
                )?;
                if started {
                    res = res.intersection(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Map(MapItem::LtOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for (left, right) in o.get_values() {
                let buf = compare_sets_using_indexes(
                    links,
                    left,
                    right,
                    CompareSign::LT,
                    storage,
                    insert_buf,
                )?;
                if started {
                    res = res.intersection(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Map(MapItem::LteOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for (left, right) in o.get_values() {
                let buf = compare_sets_using_indexes(
                    links,
                    left,
                    right,
                    CompareSign::LTE,
                    storage,
                    insert_buf,
                )?;
                if started {
                    res = res.intersection(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Vector(VectorItem::AndOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for i in o.get_items() {
                let buf = check_bool(i, links, storage, insert_buf)?;
                if started {
                    res = res.intersection(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Vector(VectorItem::OrOperator(o)) => {
            let mut started: bool = false;
            let mut res: HashSet<Link> = HashSet::new();
            for i in o.get_items() {
                let buf = check_bool(i, links, storage, insert_buf)?;
                if started {
                    res = res.union(&buf).cloned().collect();
                } else {
                    started = true;
                    res = buf;
                }
            }
            Ok(res)
        }
        Item::Modifier(ModifierItem::NotOperator(o)) => {
            let buf = check_bool(o.get_value(), links, storage, insert_buf)?;
            println!("{:?}", buf);
            let res = links.difference(&buf).cloned().collect();
            println!("{:?}", res);
            Ok(res)
        }
        _ => Err(DBError::new("Unsupported compare operator")),
    }
}
