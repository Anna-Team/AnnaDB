use crate::data_types::item::Item;
use crate::data_types::primitives::Primitive;
use crate::tyson::item::BaseTySONItemInterface;
use crate::DBError;

pub trait TySONMap: BaseTySONItemInterface {
    fn new(prefix: String) -> Result<Self, DBError>
    where
        Self: Sized;

    fn insert(&mut self, k: Primitive, v: Item) -> Result<bool, DBError>;

    fn get_items(&self) -> Vec<(Primitive, Item)>;

    fn to_item(self) -> Item;

    fn serialize(&self) -> String {
        let mut contents: Vec<String> = vec![];
        for (k, v) in self.get_items() {
            let s = format!("{}:{}", k.serialize(), v.to_tyson());
            contents.push(s);
        }
        format!("{}{{{}}}", self.get_prefix(), contents.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tyson_map_serialize_empty() {
        use crate::data_types::map::storage::StorageMap;
        let m = StorageMap::new("".to_string()).unwrap();
        let s = TySONMap::serialize(&m);
        assert!(s.contains("m{"));
        assert!(s.contains("}"));
    }

    #[test]
    fn tyson_map_serialize_with_entries() {
        use crate::data_types::map::storage::StorageMap;
        let mut m = StorageMap::new("".to_string()).unwrap();
        let key = crate::Primitive::new("s".to_string(), "key".to_string()).unwrap();
        let val = crate::Item::Primitive(crate::Primitive::new("s".to_string(), "val".to_string()).unwrap());
        m.insert(key, val).unwrap();
        let s = TySONMap::serialize(&m);
        assert!(s.contains("key"));
        assert!(s.contains("val"));
    }
}
