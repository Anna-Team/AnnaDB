use crate::query::queryset::QuerySet;
use crate::{DBError, Desereilize, Item, Primitive, TySONPrimitive, VectorItem};

#[derive(Debug)]
pub struct TransactionStep {
    pub(crate) collection_name: String,
    pub(crate) query_set: QuerySet,
}

impl TransactionStep {
    fn new(collection_name: String, data: Item) -> Result<Self, DBError> {
        match data {
            Item::Vector(VectorItem::QueriesVector(o)) => Ok(Self {
                collection_name,
                query_set: o,
            }),
            Item::Vector(VectorItem::InsertQuery(q)) => Ok(Self {
                collection_name,
                query_set: QuerySet::from(q),
            }),
            Item::Vector(VectorItem::FindQuery(q)) => Ok(Self {
                collection_name,
                query_set: QuerySet::from(q),
            }),
            Item::Vector(VectorItem::GetQuery(q)) => Ok(Self {
                collection_name,
                query_set: QuerySet::from(q),
            }),
            Item::Vector(VectorItem::UpdateQuery(q)) => Ok(Self {
                collection_name,
                query_set: QuerySet::from(q),
            }),
            Item::Primitive(Primitive::DeleteQuery(q)) => Ok(Self {
                collection_name,
                query_set: QuerySet::from(q),
            }),
            _ => Err(DBError::UnexpectedType("queryset item".to_string())),
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub(crate) steps: Vec<TransactionStep>,
}

impl Desereilize for Transaction {
    fn get_name(&self) -> String {
        "NONE".to_string()
    }

    fn new(_: String) -> Self {
        Self { steps: vec![] }
    }

    fn push(&mut self, data: (Primitive, Item)) -> Result<bool, DBError> {
        match data.0 {
            Primitive::CollectionName(o) => {
                let step: TransactionStep = TransactionStep::new(o.get_string_value(), data.1)?;
                self.steps.push(step);
            }
            _ => {
                return Err(DBError::Validation(
                    "query keys must be collections".to_string(),
                ))
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::insert::query::InsertQuery;
    use crate::query::delete::query::DeleteQuery;
    use crate::storage::common::collection_name::CollectionName;
    use crate::tyson::vector::TySONVector;
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn transaction_new() {
        let txn = Transaction::new("".to_string());
        assert!(txn.steps.is_empty());
        assert_eq!(txn.get_name(), "NONE");
    }

    #[test]
    fn transaction_push_collection() {
        let mut txn = Transaction::new("".to_string());
        let cn = CollectionName::new("".to_string(), "test".to_string()).unwrap();
        let iq = InsertQuery::new("".to_string()).unwrap();
        let item = iq.to_item();
        assert!(txn.push((Primitive::CollectionName(cn), item)).unwrap());
        assert_eq!(txn.steps.len(), 1);
    }

    #[test]
    fn transaction_push_rejects_non_collection() {
        let mut txn = Transaction::new("".to_string());
        let item = Item::Primitive(Primitive::new("null".to_string(), "".to_string()).unwrap());
        assert!(txn.push((Primitive::new("s".to_string(), "bad".to_string()).unwrap(), item)).is_err());
    }

    #[test]
    fn transaction_step_insert_query() {
        let cn = "docs".to_string();
        let iq = InsertQuery::new("".to_string()).unwrap();
        let step = TransactionStep::new(cn, iq.to_item()).unwrap();
        assert_eq!(step.collection_name, "docs");
    }

    #[test]
    fn transaction_step_get_query() {
        use crate::query::get::query::GetQuery;
        let gq = GetQuery::new("".to_string()).unwrap();
        let step = TransactionStep::new("docs".to_string(), gq.to_item()).unwrap();
        assert_eq!(step.collection_name, "docs");
    }

    #[test]
    fn transaction_step_find_query() {
        use crate::query::find::query::FindQuery;
        let fq = FindQuery::new("".to_string()).unwrap();
        let step = TransactionStep::new("docs".to_string(), fq.to_item()).unwrap();
        assert_eq!(step.collection_name, "docs");
    }

    #[test]
    fn transaction_step_update_query() {
        use crate::query::update::query::UpdateQuery;
        let uq = UpdateQuery::new("".to_string()).unwrap();
        let step = TransactionStep::new("docs".to_string(), uq.to_item()).unwrap();
        assert_eq!(step.collection_name, "docs");
    }

    #[test]
    fn transaction_step_delete_query() {
        let dq = DeleteQuery::new("".to_string(), "".to_string()).unwrap();
        let step = TransactionStep::new("docs".to_string(), dq.to_item()).unwrap();
        assert_eq!(step.collection_name, "docs");
    }

    #[test]
    fn transaction_step_queries_vector() {
        use crate::query::queryset::QuerySet;
        let qs = QuerySet::new("".to_string()).unwrap();
        let step = TransactionStep::new("docs".to_string(), qs.to_item()).unwrap();
        assert_eq!(step.collection_name, "docs");
    }

    #[test]
    fn transaction_step_rejects_invalid() {
        let item = Item::Primitive(Primitive::new("null".to_string(), "".to_string()).unwrap());
        assert!(TransactionStep::new("docs".to_string(), item).is_err());
    }
}
