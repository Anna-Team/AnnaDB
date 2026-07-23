use serde::{Deserialize, Serialize};

use crate::constants::KNN_OPERATOR;
use crate::data_types::primitives::embedding::EmbeddingPrimitive;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::map::TySONMap;
use crate::tyson::primitive::TySONPrimitive;
use crate::{DBError, Item, MapItem, Primitive};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct KnnOperator {
    pub field: String,
    pub k: usize,
    pub query_embedding: EmbeddingPrimitive,
    pub metric: String,
    values: Vec<(Primitive, Primitive)>,
}

impl KnnOperator {
    pub fn get_field(&self) -> &str {
        &self.field
    }

    pub fn get_k(&self) -> usize {
        self.k
    }

    pub fn get_query_embedding(&self) -> &EmbeddingPrimitive {
        &self.query_embedding
    }

    pub fn get_metric(&self) -> &str {
        &self.metric
    }
}

impl BaseTySONItemInterface for KnnOperator {
    fn get_prefix(&self) -> String {
        KNN_OPERATOR.to_string()
    }
}

impl TySONMap for KnnOperator {
    fn new(_: String) -> Result<Self, DBError> {
        Ok(Self {
            field: String::new(),
            k: 10,
            query_embedding: EmbeddingPrimitive::new(1, vec![0.0]),
            metric: "cosine".to_string(),
            values: vec![],
        })
    }

    fn insert(&mut self, k: Primitive, v: Item) -> Result<bool, DBError> {
        // Extract primitive value before matching on key
        let primitive_val = match v {
            Item::Primitive(ref p) => p.clone(),
            _ => return Err(DBError::TypeMismatch("knn values must be primitives".to_string())),
        };

        match &k {
            Primitive::PathToValue(p) => {
                self.field = p.value.clone();
            }
            Primitive::StringPrimitive(ref s) => match s.get_string_value().as_str() {
                "k" => {
                    if let Primitive::NumberPrimitive(ref n) = primitive_val {
                        self.k = n.get_value() as usize;
                    } else {
                        return Err(DBError::TypeMismatch("knn k must be a number".to_string()));
                    }
                }
                "of" => {
                    if let Primitive::EmbeddingPrimitive(ref e) = primitive_val {
                        self.query_embedding = e.clone();
                    } else {
                        return Err(DBError::TypeMismatch(
                            "knn of must be an embedding".to_string(),
                        ));
                    }
                }
                "using" => {
                    if let Primitive::StringPrimitive(ref val) = primitive_val {
                        self.metric = val.get_string_value();
                    } else {
                        return Err(DBError::TypeMismatch(
                            "knn using must be a string".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(DBError::TypeMismatch(format!(
                        "unknown knn field: {}",
                        s.get_string_value()
                    )));
                }
            },
            _ => {
                return Err(DBError::TypeMismatch(
                    "knn keys must be a path or string".to_string(),
                ));
            }
        }
        self.values.push((k, primitive_val));
        Ok(true)
    }

    fn get_items(&self) -> Vec<(Primitive, Item)> {
        self.values
            .iter()
            .map(|(k, v)| (k.clone(), Item::Primitive(v.clone())))
            .collect()
    }

    fn to_item(self) -> Item {
        Item::Map(MapItem::KnnOperator(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn knn_operator_new() {
        let op = KnnOperator::new("".to_string()).unwrap();
        assert_eq!(op.get_prefix(), "knn");
        assert_eq!(op.get_k(), 10);
    }

    #[test]
    fn knn_operator_insert_field() {
        let mut op = KnnOperator::new("".to_string()).unwrap();
        let k = Primitive::PathToValue(crate::PathToValue::new("".to_string(), "embedding".to_string()).unwrap());
        let v = Item::Primitive(Primitive::new("null".to_string(), "".to_string()).unwrap());
        op.insert(k, v).unwrap();
        assert_eq!(op.get_field(), "embedding");
    }

    #[test]
    fn knn_operator_insert_k() {
        let mut op = KnnOperator::new("".to_string()).unwrap();
        let k = Primitive::new("s".to_string(), "k".to_string()).unwrap();
        let v = Item::Primitive(Primitive::new("n".to_string(), "5".to_string()).unwrap());
        op.insert(k, v).unwrap();
        assert_eq!(op.get_k(), 5);
    }

    #[test]
    fn knn_operator_insert_using() {
        let mut op = KnnOperator::new("".to_string()).unwrap();
        let k = Primitive::new("s".to_string(), "using".to_string()).unwrap();
        let v = Item::Primitive(Primitive::new("s".to_string(), "euclidean".to_string()).unwrap());
        op.insert(k, v).unwrap();
        assert_eq!(op.get_metric(), "euclidean");
    }

    #[test]
    fn knn_operator_to_item() {
        let op = KnnOperator::new("".to_string()).unwrap();
        let item = op.to_item();
        assert!(matches!(item, Item::Map(MapItem::KnnOperator(_))));
    }
}
