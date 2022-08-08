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
            _ => Err(DBError::new("Unexpected queryset item type")),
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
                return Err(DBError::new(
                    "Query parsing error. Keys must be collections",
                ))
            }
        }
        Ok(true)
    }
}
