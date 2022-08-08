use crate::tyson::de::Desereilize;
use crate::tyson::se::Serialize;

pub trait TySONJournal: Desereilize + Serialize {}
