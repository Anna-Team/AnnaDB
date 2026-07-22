use crate::constants::NULL;
use crate::data_types::map::storage::StorageMap;
use crate::data_types::vector::storage::StorageVector;
use crate::storage::buffer::InsertBuffer;
use crate::{
    DBError, Item, Link, PathToValue, Primitive, Storage, StringPrimitive, TySONMap,
    TySONPrimitive, TySONVector,
};

pub fn resolve(
    rules: Item,
    field: Option<&StringPrimitive>,
    link: &Link,
    storage: &Storage,
    insert_buf: &InsertBuffer,
) -> Result<Item, DBError> {
    let default = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
    match rules {
        Item::Primitive(Primitive::KeepPrimitive(_)) => {
            if let Some(f) = field {
                let path = PathToValue::new("".to_string(), f.get_string_value())?;
                let res = storage.get_value_by_path(path, link.clone(), insert_buf)?;
                match res {
                    Some(o) => {
                        let item_to_fetch = o.value.unwrap_or(default);
                        Ok(storage.fetch(&item_to_fetch, insert_buf, 0)?)
                    }
                    None => Ok(default),
                }
            } else {
                Ok(default)
            }
        }
        Item::Primitive(Primitive::PathToValue(path)) => {
            let res = storage.get_value_by_path(path.clone(), link.clone(), insert_buf)?;
            match res {
                Some(o) => {
                    let item_to_fetch = o.value.unwrap_or(default);
                    Ok(storage.fetch(&item_to_fetch, insert_buf, 0)?)
                }
                None => Ok(default),
            }
        }
        Item::Primitive(Primitive::NumberPrimitive(_)) => Ok(rules.clone()),
        Item::Primitive(Primitive::StringPrimitive(_)) => Ok(rules.clone()),
        Item::Primitive(Primitive::UTSPrimitive(_)) => Ok(rules.clone()),
        Item::Primitive(Primitive::BoolPrimitive(_)) => Ok(rules.clone()),
        Item::Primitive(Primitive::NullPrimitive(_)) => Ok(rules.clone()),
        Item::Vector(v) => {
            let mut new_vec = StorageVector::new("".to_string())?;
            for (i, v) in v.get_items().iter().enumerate() {
                let new_field = if let Some(f) = field {
                    format!("{}.{}", f.get_string_value(), i)
                } else {
                    i.to_string()
                };

                new_vec.push(resolve(
                    v.clone(),
                    Some(&StringPrimitive::new("".to_string(), new_field)?),
                    link,
                    storage,
                    insert_buf,
                )?)?;
            }
            Ok(new_vec.to_item())
        }
        Item::Map(m) => {
            let mut new_map = StorageMap::new("".to_string())?;
            for (k, v) in m.get_items() {
                match &k {
                    Primitive::StringPrimitive(s) => {
                        let new_field = if let Some(f) = field {
                            format!("{}.{}", f.get_string_value(), s.get_string_value())
                        } else {
                            s.get_string_value()
                        };

                        new_map.insert(
                            k.clone(),
                            resolve(
                                v,
                                Some(&StringPrimitive::new("".to_string(), new_field)?),
                                link,
                                storage,
                                insert_buf,
                            )?,
                        )?;
                    }
                    _ => return Err(DBError::TypeMismatch("projection keys must be strings".to_string())),
                }
            }
            Ok(new_map.to_item())
        }
        _ => Err(DBError::UnsupportedOperation("projection rule".to_string())),
    }
}
