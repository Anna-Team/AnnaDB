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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tyson_modifier_serialize() {
        use crate::query::find::operators::not::NotOperator;
        let val = crate::Item::Primitive(crate::Primitive::new("b".to_string(), "true".to_string()).unwrap());
        let not_op = NotOperator::new("".to_string(), val).unwrap();
        let s = TySONModifier::serialize(&not_op);
        assert!(s.contains("not"));
    }
}
