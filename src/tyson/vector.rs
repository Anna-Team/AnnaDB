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
            contents.push(i.serialize())
        }
        format!("{}[{}]", prefix, contents.join(","))
    }

    fn to_item(self) -> Item;
}
