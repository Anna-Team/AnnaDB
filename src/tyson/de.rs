use crate::data_types::primitives::Primitive;
use pest::iterators::Pair;
use pest::Parser;

use crate::data_types::item::Item;
use crate::data_types::map::MapItem;
use crate::data_types::modifier::ModifierItem;
use crate::data_types::vector::VectorItem;
use crate::errors::DBError;
use crate::tyson::map::TySONMap;
use crate::tyson::modifier::TySONModifier;
use crate::tyson::vector::TySONVector;

#[derive(Parser)]
#[grammar = "tyson/grammar.pest"]
struct TySONParser;

pub trait Desereilize {
    fn get_name(&self) -> String;

    fn deserialize_primitive(&self, pair: Pair<Rule>) -> Result<Primitive, DBError> {
        let mut data: String = String::new();
        let mut prefix: String = String::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::prefix => {
                    prefix = pair.as_str().to_string();
                }
                _ => {
                    data = pair.as_str().to_string();
                }
            }
        }
        Ok(Self::new_primitive(&self, prefix, data)?)
    }

    fn deserialize_modifier(&self, pair: Pair<Rule>) -> Result<ModifierItem, DBError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules
            .next()
            .ok_or(DBError::unexpected_parsing())?
            .as_str()
            .to_string();
        let pair = inner_rules.next().ok_or(DBError::unexpected_parsing())?;
        Ok(Self::new_modifier(
            prefix,
            Self::route_deserialization(&self, pair)?,
        )?)
    }

    fn deserialize_vector(&self, pair: Pair<Rule>) -> Result<VectorItem, DBError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules
            .next()
            .ok_or(DBError::unexpected_parsing())?
            .as_str()
            .to_string();
        let mut vector = Self::new_vector(prefix)?;
        for pair in inner_rules {
            vector.push(Self::route_deserialization(&self, pair)?)?;
        }
        Ok(vector)
    }

    fn deserialize_map(&self, pair: Pair<Rule>) -> Result<MapItem, DBError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules
            .next()
            .ok_or(DBError::unexpected_parsing())?
            .as_str()
            .to_string();
        let mut map = Self::new_map(prefix)?;
        for pair in inner_rules {
            let mut inner_rules = pair.into_inner();
            let left = Self::deserialize_primitive(
                &self,
                inner_rules.next().ok_or(DBError::unexpected_parsing())?,
            )?;
            map.insert(
                left,
                Self::route_deserialization(
                    &self,
                    inner_rules.next().ok_or(DBError::unexpected_parsing())?,
                )?,
            )?;
        }
        Ok(map)
    }

    fn route_deserialization(&self, pair: Pair<Rule>) -> Result<Item, DBError> {
        return match pair.as_rule() {
            Rule::map => {
                let res = Self::deserialize_map(&self, pair)?;
                Ok(Item::Map(res))
            }
            Rule::vector => {
                let res = Self::deserialize_vector(&self, pair)?;
                Ok(Item::Vector(res))
            }
            Rule::modifier => {
                let res = Self::deserialize_modifier(&self, pair)?;
                Ok(Item::from(res))
            }
            Rule::primitive => {
                let res = Self::deserialize_primitive(&self, pair)?;
                Ok(Item::from(res))
            }
            _ => Err(DBError::new("Deserialization error")),
        };
    }

    fn deserialize(name: String, data: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        let pair = TySONParser::parse(Rule::journal, data.as_str())?
            .next()
            .ok_or(DBError::unexpected_parsing())?;

        let mut result = Self::new(name);

        match pair.as_rule() {
            Rule::journal => {
                for pair in pair.into_inner() {
                    let mut inner_rules = pair.into_inner();
                    match inner_rules.next() {
                        Some(v) => {
                            let key = result.deserialize_primitive(v)?;
                            result.push((
                                key,
                                result.route_deserialization(
                                    inner_rules.next().ok_or(DBError::unexpected_parsing())?,
                                )?,
                            ))?;
                        }
                        _ => {}
                    }
                }
                Ok(result)
            }
            _ => Err(DBError::new("Deserialization error")),
        }
    }

    fn new(name: String) -> Self;

    fn push(&mut self, data: (Primitive, Item)) -> Result<bool, DBError>;

    fn new_modifier(prefix: String, data: Item) -> Result<ModifierItem, DBError> {
        Ok(ModifierItem::new(prefix, data)?)
    }

    fn new_vector(prefix: String) -> Result<VectorItem, DBError> {
        Ok(VectorItem::new(prefix.to_string())?)
    }

    fn new_map(prefix: String) -> Result<MapItem, DBError> {
        Ok(MapItem::new(prefix.to_string())?)
    }

    fn new_primitive(&self, prefix: String, data: String) -> Result<Primitive, DBError> {
        Ok(Primitive::new(prefix, data)?)
    }
}
