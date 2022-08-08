use crate::data_types::primitives::Primitive;
use crate::Item;

pub trait Serialize {
    fn items(&self) -> Vec<(&Primitive, &Item)>;

    fn serialize(&self) -> String {
        let mut contents: Vec<String> = vec![];

        for (k, v) in self.items() {
            contents.push(format!("{}:{}", k.serialize(), v.serialize()));
        }
        format!("{}", contents.join(";"))
    }
}
