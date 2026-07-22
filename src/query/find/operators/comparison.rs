macro_rules! comparison_operator {
    ($name:ident, $const_name:ident, $err_label:expr) => {
        #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            values: Vec<($crate::Primitive, $crate::Primitive)>,
        }

        impl $crate::tyson::item::BaseTySONItemInterface for $name {
            fn get_prefix(&self) -> String {
                $const_name.to_string()
            }
        }

        impl $crate::tyson::map::TySONMap for $name {
            fn new(_: String) -> Result<Self, $crate::DBError>
            where
                Self: Sized,
            {
                Ok(Self { values: vec![] })
            }

            fn insert(&mut self, k: $crate::Primitive, v: $crate::Item) -> Result<bool, $crate::DBError> {
                match v {
                    $crate::Item::Primitive(o) => {
                        self.values.push((k, o));
                        Ok(true)
                    }
                    _ => Err($crate::DBError::TypeMismatch(
                        format!("{} operator can contain only primitives", $err_label),
                    )),
                }
            }

            fn get_items(&self) -> Vec<($crate::Primitive, $crate::Item)> {
                self.values
                    .iter()
                    .map(|(k, v)| (k.clone(), $crate::Item::Primitive(v.clone())))
                    .collect()
            }

            fn to_item(self) -> $crate::Item {
                $crate::Item::Map($crate::MapItem::$name(self))
            }
        }

        impl $name {
            pub fn get_values(&self) -> Vec<(&$crate::Primitive, &$crate::Primitive)> {
                self.values.iter().map(|(k, v)| (k, v)).collect()
            }
        }
    };
}
