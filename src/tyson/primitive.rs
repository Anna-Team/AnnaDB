use crate::tyson::item::BaseTySONItemInterface;
use crate::DBError;

pub trait TySONPrimitive: BaseTySONItemInterface {
    fn new(prefix: String, value: String) -> Result<Self, DBError>
    where
        Self: Sized;

    fn get_string_value(&self) -> String;

    fn serialize(&self) -> String {
        let prefix = self.get_prefix();
        let value = self.get_string_value(); // TODO escape
        if value == "".to_string() {
            format!("{}", prefix)
        } else if prefix == "".to_string() {
            format!("|{}|", value)
        } else {
            format!("{}|{}|", prefix, value)
        }
    }
}
