use crate::data_types::item::Item;
use crate::tyson::item::BaseTySONItemInterface;
use crate::DBError;

pub trait TySONVector: BaseTySONItemInterface {
    fn new(prefix: String) -> Result<Self, DBError>
    where
        Self: Sized;

    fn push(&mut self, item: Item) -> Result<bool, DBError>;

    fn get_items(&self) -> &Vec<Item>;

    fn serialize(&self) -> String {
        let prefix = self.get_prefix();
        let mut contents: Vec<String> = vec![];
        for i in self.get_items() {
            contents.push(i.to_tyson())
        }
        format!("{}[{}]", prefix, contents.join(","))
    }

    fn to_item(self) -> Item;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tyson_vector_serialize_empty() {
        use crate::data_types::vector::storage::StorageVector;
        let v = StorageVector::new("".to_string()).unwrap();
        let s = TySONVector::serialize(&v);
        assert_eq!(s, "v[]");
    }

    #[test]
    fn tyson_vector_serialize_with_items() {
        use crate::data_types::vector::storage::StorageVector;
        let mut v = StorageVector::new("".to_string()).unwrap();
        let item = crate::Item::Primitive(crate::Primitive::new("s".to_string(), "hi".to_string()).unwrap());
        v.push(item).unwrap();
        let s = TySONVector::serialize(&v);
        assert!(s.contains("hi"));
    }
}
