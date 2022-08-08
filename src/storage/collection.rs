use crate::constants::INTERNAL_COLLECTION_NAME;
use std::collections::HashMap;
use std::fs;
use std::fs::{read_to_string, File};

use crate::data_types::item::Item;

use crate::data_types::primitives::link::Link;
use crate::data_types::primitives::Primitive;

use crate::DBError;

use crate::tyson::de::Desereilize;

#[derive(Debug)]
pub struct Collection {
    pub name: String,
    pub(crate) values: HashMap<Link, Item>,
}

impl Desereilize for Collection {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn new(name: String) -> Self {
        Self {
            name,
            values: HashMap::new(),
        }
    }

    fn push(&mut self, data: (Primitive, Item)) -> Result<bool, DBError> {
        match data.0 {
            Primitive::Link(o) => match data.1 {
                Item::Primitive(Primitive::DeletedPrimitive(_)) => {
                    self.values.remove(&o);
                }
                _ => {
                    self.values.insert(o, data.1);
                }
            },
            _ => return Err(DBError::new("Internal storage read error")),
        }

        Ok(true)
    }
}

impl Collection {
    pub(crate) fn new(name: String, wh_path: String) -> Result<Self, DBError> {
        if !name.starts_with("_") || name == INTERNAL_COLLECTION_NAME.to_string() {
            let file_path = format!("{}/{}.tyson", wh_path, name);
            let is_exists = std::path::Path::new(file_path.as_str()).exists();
            if is_exists {
                let data = read_to_string(file_path.as_str())?;
                Ok(Self::deserialize(name, data)?)
            } else {
                File::create(file_path.as_str())?;
                Ok(Self {
                    name,
                    values: HashMap::new(),
                })
            }
        } else {
            Err(DBError::new(
                format!("Invalid collection name: {}", name).as_str(),
            ))
        }
    }

    pub(crate) fn get_path(&self, wh_path: String) -> String {
        format!("{}/{}.tyson", wh_path, self.name)
    }

    pub(crate) fn get_file(&self, wh_path: String) -> Result<File, DBError> {
        let file_path = self.get_path(wh_path);
        Ok(fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path.as_str())?)
    }

    pub(crate) fn get_value(&self, id: &Link) -> Result<Item, DBError> {
        Ok(self
            .values
            .get(id)
            .ok_or(DBError::new(
                "Internal error: there is no such id in the collection",
            ))?
            .clone())
    }
}
