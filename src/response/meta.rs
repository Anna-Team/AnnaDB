use crate::constants::{DELETE_META, FIND_META, GET_META, INSERT_META, UPDATE_META};
use crate::data_types::primitives::number::NumberPrimitive;
use crate::tyson::primitive::TySONPrimitive;

macro_rules! meta_type {
    ($name:ident, $const_name:ident) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $name {
            pub count: NumberPrimitive,
        }

        impl $name {
            pub fn new(count: usize) -> Self {
                Self {
                    count: NumberPrimitive::from(count),
                }
            }

            pub fn serialize(&self) -> String {
                format!(
                    "{}{{s|count|:{}}}",
                    $const_name,
                    TySONPrimitive::serialize(&self.count)
                )
            }
        }
    };
}

meta_type!(InsertMeta, INSERT_META);
meta_type!(GetMeta, GET_META);
meta_type!(DeleteMeta, DELETE_META);
meta_type!(UpdateMeta, UPDATE_META);
meta_type!(FindMeta, FIND_META);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Meta {
    InsertMeta(InsertMeta),
    GetMeta(GetMeta),
    DeleteMeta(DeleteMeta),
    UpdateMeta(UpdateMeta),
    FindMeta(FindMeta),
}

impl Meta {
    pub fn serialize(&self) -> String {
        match self {
            Meta::InsertMeta(v) => v.serialize(),
            Meta::GetMeta(v) => v.serialize(),
            Meta::DeleteMeta(v) => v.serialize(),
            Meta::UpdateMeta(v) => v.serialize(),
            Meta::FindMeta(v) => v.serialize(),
        }
    }
}
