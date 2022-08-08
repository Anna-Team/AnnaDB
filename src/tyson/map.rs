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
            let s = format!("{}:{}", k.serialize(), v.serialize());
            contents.push(s);
        }
        format!("{}{{{}}}", self.get_prefix(), contents.join(","))
    }
}
