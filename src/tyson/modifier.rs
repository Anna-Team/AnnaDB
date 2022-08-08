use crate::tyson::item::BaseTySONItemInterface;
use crate::{DBError, Item};

pub trait TySONModifier: BaseTySONItemInterface {
    fn new(prefix: String, value: Item) -> Result<Self, DBError>
    where
        Self: Sized;

    fn get_serialized_value(&self) -> String;

    fn serialize(&self) -> String {
        let prefix = self.get_prefix();
        format!("{}({})", prefix, self.get_serialized_value())
    }
}
