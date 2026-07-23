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
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn tyson_modifier_serialize() {
        use crate::query::find::operators::not::NotOperator;
        let val = crate::Item::Primitive(crate::Primitive::new("b".to_string(), "true".to_string()).unwrap());
        let not_op = NotOperator::new("".to_string(), val).unwrap();
        let s = TySONModifier::serialize(&not_op);
        assert!(s.contains("not"));
    }

    #[test]
    fn asc_operator_serialize() {
        use crate::query::sort::query::AscOperator;
        use crate::Primitive;
        use crate::Item;
        use crate::data_types::primitives::path::PathToValue;

        let p = PathToValue::new("".to_string(), "name".to_string()).unwrap();
        let asc = AscOperator::new("".to_string(), Item::Primitive(Primitive::PathToValue(p))).unwrap();
        let s = TySONModifier::serialize(&asc);
        assert!(s.contains("asc"));
    }

    #[test]
    fn desc_operator_serialize() {
        use crate::query::sort::query::DescOperator;
        use crate::Primitive;
        use crate::Item;
        use crate::data_types::primitives::path::PathToValue;

        let p = PathToValue::new("".to_string(), "age".to_string()).unwrap();
        let desc = DescOperator::new("".to_string(), Item::Primitive(Primitive::PathToValue(p))).unwrap();
        let s = TySONModifier::serialize(&desc);
        assert!(s.contains("desc"));
    }
}
