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
                return Ok(Res::True);
            } else {
                return Ok(Res::False);
            }
        }
        _ => compare(item, id, storage, insert_buf),
    }
}

pub fn compare(
    op: &Item,
    id: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<Res, DBError> {
    match op {
        Item::Map(MapItem::EqOperator(o)) => {
            for (k, v) in o.get_values() {
                let compare_res = compare_primitives(k, v, id, storage, insert_buf)?;
                // if compare_res == CompareResult::CanNotCompare {
                //     return Ok(Res::None);
                // } else
                if compare_res != CompareResult::Equal {
                    return Ok(Res::False);
                }
            }
            Ok(Res::True)
        }
        Item::Map(MapItem::NeqOperator(o)) => {
            for (k, v) in o.get_values() {
                let compare_res = compare_primitives(k, v, id, storage, insert_buf)?;
                // if compare_res == CompareResult::CanNotCompare {
                //     return Ok(Res::None);
                // } else
                if compare_res == CompareResult::Equal {
                    return Ok(Res::False);
                }
            }
            Ok(Res::True)
        }
        Item::Map(MapItem::GtOperator(o)) => {
            for (k, v) in o.get_values() {
                let compare_res = compare_primitives(k, v, id, storage, insert_buf)?;
                // if compare_res == CompareResult::CanNotCompare {
                //     return Ok(Res::None);
                // } else
                if compare_res != CompareResult::Greater {
                    return Ok(Res::False);
                }
            }
            Ok(Res::True)
        }
        Item::Map(MapItem::GteOperator(o)) => {
            for (k, v) in o.get_values() {
                let compare_res = compare_primitives(k, v, id, storage, insert_buf)?;
                // if compare_res == CompareResult::CanNotCompare {
                //     return Ok(Res::None);
                // } else
                if compare_res != CompareResult::Greater && compare_res != CompareResult::Equal {
                    return Ok(Res::False);
                }
            }
            Ok(Res::True)
        }
        Item::Map(MapItem::LtOperator(o)) => {
            for (k, v) in o.get_values() {
                let compare_res = compare_primitives(k, v, id, storage, insert_buf)?;
                // if compare_res == CompareResult::CanNotCompare {
                //     return Ok(Res::None);
                // } else
                if compare_res != CompareResult::Less {
                    return Ok(Res::False);
                }
            }
            Ok(Res::True)
        }
        Item::Map(MapItem::LteOperator(o)) => {
            for (k, v) in o.get_values() {
                let compare_res = compare_primitives(k, v, id, storage, insert_buf)?;
                // if compare_res == CompareResult::CanNotCompare {
                //     return Ok(Res::None);
                // } else
                if compare_res != CompareResult::Less && compare_res != CompareResult::Equal {
                    return Ok(Res::False);
                }
            }
            Ok(Res::True)
        }
        Item::Vector(VectorItem::AndOperator(o)) => {
            for i in o.get_items() {
                let bool_res = check_bool(i, id, storage, insert_buf)?;
                if bool_res == Res::False {
                    return Ok(Res::False);
                } else if bool_res == Res::None {
                    return Ok(Res::None);
                }
            }
            Ok(Res::True)
        }
        Item::Vector(VectorItem::OrOperator(o)) => {
            for i in o.get_items() {
                let bool_res = check_bool(i, id, storage, insert_buf)?;
                if bool_res == Res::True {
                    return Ok(Res::True);
                } else if bool_res == Res::None {
                    return Ok(Res::None);
                }
            }
            Ok(Res::False)
        }
        Item::Modifier(ModifierItem::NotOperator(o)) => {
            let bool_res = check_bool(o.get_value(), id, storage, insert_buf)?;
            if bool_res == Res::True {
                return Ok(Res::False);
            } else if bool_res == Res::False {
                return Ok(Res::True);
            } else {
                return Ok(Res::None);
            }
        }
        _ => Err(DBError::new("Unsupported compare operator")),
    }
}
