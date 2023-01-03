use crate::constants::NULL;
use crate::data_types::map::storage::StorageMap;
use crate::data_types::vector::storage::StorageVector;
use crate::storage::buffer::InsertBuffer;
use crate::{
    DBError, Item, Link, MapItem, PathToValue, Primitive, Storage, StringPrimitive, TySONMap,
    TySONPrimitive, TySONVector, VectorItem,
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
            let path = PathToValue::new("".to_string(), field.unwrap().get_string_value())?;
            let res = storage.get_value_by_path(path, link.clone(), insert_buf)?;
            match res {
                Some(o) => {
                    let item_to_fetch = o.value.unwrap_or(default);
                    Ok(storage.fetch(&item_to_fetch, insert_buf, 0)?)
                }
                None => Ok(default),
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
                let mut new_field = i.to_string();
                if field.is_some() {
                    new_field = format!("{}.{}", field.unwrap().get_string_value(), new_field);
                }

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
                        let mut new_field = s.get_string_value();
                        if field.is_some() {
                            new_field =
                                format!("{}.{}", field.unwrap().get_string_value(), new_field);
                        }

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
                    _ => return Err(DBError::new("Projection keys must be strings")),
                }
            }
            Ok(new_map.to_item())
        }
        _ => Err(DBError::new("Projection rule is not supported")),
    }
}
