use crate::data_types::modifier::ModifierItem;
use crate::{DBError, Item, Link, MapItem, Primitive, Storage, TySONVector, VectorItem};

use crate::storage::buffer::InsertBuffer;

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
            match storage.get_value_by_path(o.clone(), id.clone(), insert_buf)? {
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
            let val = storage.get_value_by_link(id)?;
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
            }
        }
    }
}

fn check_bool(
    item: &Item,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<Res, DBError> {
    match item {
        Item::Primitive(Primitive::BoolPrimitive(o)) => {
            if o.val() {
                Ok(Res::True)
            } else {
                Ok(Res::False)
            }
        }
        _ => compare(item, id, storage, insert_buf),
    }
}

fn compare_scalar(
    values: Vec<(&Primitive, &Primitive)>,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
    match_true: fn(CompareResult) -> bool,
    short_circuit: Res,
    default: Res,
) -> Result<Res, DBError> {
    for (k, v) in values {
        let compare_res = compare_primitives(k, v, id, storage, insert_buf)?;
        if compare_res == CompareResult::CanNotCompare {
            return Ok(Res::None);
        }
        if match_true(compare_res) {
            return Ok(short_circuit);
        }
    }
    Ok(default)
}

fn compare_logical(
    op: &Item,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<Res, DBError> {
    match op {
        Item::Vector(VectorItem::AndOperator(o)) => {
            for i in o.get_items() {
                match check_bool(i, id, storage, insert_buf)? {
                    Res::False => return Ok(Res::False),
                    Res::None => return Ok(Res::None),
                    _ => {}
                }
            }
            Ok(Res::True)
        }
        Item::Vector(VectorItem::OrOperator(o)) => {
            for i in o.get_items() {
                match check_bool(i, id, storage, insert_buf)? {
                    Res::True => return Ok(Res::True),
                    Res::None => return Ok(Res::None),
                    _ => {}
                }
            }
            Ok(Res::False)
        }
        Item::Modifier(ModifierItem::NotOperator(o)) => {
            match check_bool(o.get_value(), id, storage, insert_buf)? {
                Res::True => Ok(Res::False),
                Res::False => Ok(Res::True),
                _ => Ok(Res::None),
            }
        }
        _ => Err(DBError::UnsupportedOperation("compare operator".to_string())),
    }
}

pub fn compare(
    op: &Item,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<Res, DBError> {
    match op {
        Item::Map(MapItem::EqOperator(o)) => compare_scalar(
            o.get_values(), id, storage, insert_buf,
            |r| r != CompareResult::Equal, Res::False, Res::True,
        ),
        Item::Map(MapItem::NeqOperator(o)) => compare_scalar(
            o.get_values(), id, storage, insert_buf,
            |r| r == CompareResult::Equal, Res::False, Res::True,
        ),
        Item::Map(MapItem::GtOperator(o)) => compare_scalar(
            o.get_values(), id, storage, insert_buf,
            |r| r != CompareResult::Greater, Res::False, Res::True,
        ),
        Item::Map(MapItem::GteOperator(o)) => compare_scalar(
            o.get_values(), id, storage, insert_buf,
            |r| r != CompareResult::Greater && r != CompareResult::Equal, Res::False, Res::True,
        ),
        Item::Map(MapItem::LtOperator(o)) => compare_scalar(
            o.get_values(), id, storage, insert_buf,
            |r| r != CompareResult::Less, Res::False, Res::True,
        ),
        Item::Map(MapItem::LteOperator(o)) => compare_scalar(
            o.get_values(), id, storage, insert_buf,
            |r| r != CompareResult::Less && r != CompareResult::Equal, Res::False, Res::True,
        ),
        _ => compare_logical(op, id, storage, insert_buf),
    }
}
