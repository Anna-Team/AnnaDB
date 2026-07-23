use crate::data_types::primitives::Primitive;
use crate::Item;

pub trait Serialize {
    fn items(&self) -> Vec<(&Primitive, &Item)>;

    fn serialize(&self) -> String {
        let mut contents: Vec<String> = vec![];

        for (k, v) in self.items() {
            contents.push(format!("{}:{}", k.serialize(), v.to_tyson()));
        }
        format!("{}", contents.join(";"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_types::primitives::Primitive;

    struct TestJournal {
        entries: Vec<(Primitive, Item)>,
    }

    impl Serialize for TestJournal {
        fn items(&self) -> Vec<(&Primitive, &Item)> {
            self.entries.iter().map(|(k, v)| (k, v)).collect()
        }
    }

    #[test]
    fn serialize_empty() {
        let j = TestJournal { entries: vec![] };
        assert_eq!(j.serialize(), "");
    }

    #[test]
    fn serialize_single_entry() {
        let k = Primitive::new("s".to_string(), "key".to_string()).unwrap();
        let v = Item::Primitive(Primitive::new("s".to_string(), "value".to_string()).unwrap());
        let j = TestJournal { entries: vec![(k, v)] };
        let s = j.serialize();
        assert!(s.contains("key"));
        assert!(s.contains("value"));
        assert!(s.contains(":"));
    }

    #[test]
    fn serialize_multiple_entries() {
        let k1 = Primitive::new("s".to_string(), "a".to_string()).unwrap();
        let v1 = Item::Primitive(Primitive::new("n".to_string(), "1".to_string()).unwrap());
        let k2 = Primitive::new("s".to_string(), "b".to_string()).unwrap();
        let v2 = Item::Primitive(Primitive::new("n".to_string(), "2".to_string()).unwrap());
        let j = TestJournal { entries: vec![(k1, v1), (k2, v2)] };
        let s = j.serialize();
        assert!(s.contains(";"));
    }
}
