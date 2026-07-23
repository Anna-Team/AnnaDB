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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tyson::item::TySONItem;
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn tyson_primitive_serialize() {
        let s = crate::StringPrimitive::new("s".to_string(), "hello".to_string()).unwrap();
        let serialized = TySONPrimitive::serialize(&s);
        assert_eq!(serialized, "s|hello|");
    }

    #[test]
    fn tyson_primitive_get_string_value() {
        let s = crate::StringPrimitive::new("s".to_string(), "hello".to_string()).unwrap();
        assert_eq!(s.get_string_value(), "hello");
    }

    #[test]
    fn base_tyson_item_interface_prefix() {
        use crate::tyson::item::BaseTySONItemInterface;
        let s = crate::StringPrimitive::new("s".to_string(), "hello".to_string()).unwrap();
        assert_eq!(s.get_prefix(), "s");
    }
}
