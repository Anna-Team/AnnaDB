use crate::constants::{DELETE_META, FIND_META, GET_META, INSERT_META, UPDATE_META};
use crate::data_types::primitives::number::NumberPrimitive;
use crate::TySONPrimitive;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InsertMeta {
    pub count: NumberPrimitive,
}

impl InsertMeta {
    pub fn new(count: usize) -> Self {
        Self {
            count: NumberPrimitive::from(count),
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}{{s|count|:{}}}", INSERT_META, self.count.serialize())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GetMeta {
    pub count: NumberPrimitive,
}

impl GetMeta {
    pub fn new(count: usize) -> Self {
        Self {
            count: NumberPrimitive::from(count),
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}{{s|count|:{}}}", GET_META, self.count.serialize())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeleteMeta {
    pub count: NumberPrimitive,
}

impl DeleteMeta {
    pub fn new(count: usize) -> Self {
        Self {
            count: NumberPrimitive::from(count),
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}{{s|count|:{}}}", DELETE_META, self.count.serialize())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateMeta {
    pub count: NumberPrimitive,
}

impl UpdateMeta {
    pub fn new(count: usize) -> Self {
        Self {
            count: NumberPrimitive::from(count),
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}{{s|count|:{}}}", UPDATE_META, self.count.serialize())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FindMeta {
    pub count: NumberPrimitive,
}

impl FindMeta {
    pub fn new(count: usize) -> Self {
        Self {
            count: NumberPrimitive::from(count),
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}{{s|count|:{}}}", FIND_META, self.count.serialize())
    }
}

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
