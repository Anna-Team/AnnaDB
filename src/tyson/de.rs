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
            _ => Err(DBError::Deserialization("unknown rule in route_deserialization".to_string())),
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
                    // Skip EOI token that may appear in journal inner tokens
                    if pair.as_rule() == Rule::EOI {
                        continue;
                    }
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
                        _ => return Err(DBError::unexpected_parsing()),
                    }
                }
                Ok(result)
            }
            _ => Err(DBError::Deserialization("expected journal rule".to_string())),
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

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    // ─── P2.1: Primitive Deserialization ────────────────────────────

    #[test]
    fn parse_string_primitive() {
        let p = TySONParser::parse(Rule::primitive, "s|hello|").unwrap();
        let outer = p.into_iter().next().unwrap();
        let mut inner = outer.into_inner();
        let prefix = inner.next().unwrap();
        assert_eq!(prefix.as_str(), "s");
        let value = inner.next().unwrap();
        assert_eq!(value.as_str(), "hello");
    }

    #[test]
    fn parse_number_primitive() {
        let p = TySONParser::parse(Rule::primitive, "n|42.5|").unwrap();
        let outer = p.into_iter().next().unwrap();
        let mut inner = outer.into_inner();
        let prefix = inner.next().unwrap();
        assert_eq!(prefix.as_str(), "n");
        let value = inner.next().unwrap();
        assert_eq!(value.as_str(), "42.5");
    }

    #[test]
    fn parse_bool_primitive_true() {
        let p = TySONParser::parse(Rule::primitive, "b|true|").unwrap();
        let outer = p.into_iter().next().unwrap();
        let mut inner = outer.into_inner();
        let prefix = inner.next().unwrap();
        assert_eq!(prefix.as_str(), "b");
    }

    #[test]
    fn parse_null_primitive() {
        let p = TySONParser::parse(Rule::primitive, "null").unwrap();
        let outer = p.into_iter().next().unwrap();
        assert_eq!(outer.as_str(), "null");
    }

    #[test]
    fn parse_primitive_no_prefix_just_value() {
        let p = TySONParser::parse(Rule::primitive, "|hello|").unwrap();
        let outer = p.into_iter().next().unwrap();
        let mut inner = outer.into_inner();
        let value = inner.next().unwrap();
        assert_eq!(value.as_str(), "hello");
    }

    // ─── P2.2: Map Deserialization ─────────────────────────────────

    #[test]
    fn parse_empty_map() {
        let p = TySONParser::parse(Rule::map, "m{}").unwrap();
        let outer = p.into_iter().next().unwrap();
        let mut inner = outer.into_inner();
        let prefix = inner.next().unwrap();
        assert_eq!(prefix.as_str(), "m");
    }

    #[test]
    fn parse_map_with_entries() {
        let p = TySONParser::parse(Rule::map, "m{s|k|:s|v|}").unwrap();
        let outer = p.into_iter().next().unwrap();
        assert_eq!(outer.as_rule(), Rule::map);
    }

    // ─── P2.3: Vector Deserialization ──────────────────────────────

    #[test]
    fn parse_empty_vector() {
        let p = TySONParser::parse(Rule::vector, "v[]").unwrap();
        let outer = p.into_iter().next().unwrap();
        let mut inner = outer.into_inner();
        let prefix = inner.next().unwrap();
        assert_eq!(prefix.as_str(), "v");
    }

    #[test]
    fn parse_vector_with_items() {
        let p = TySONParser::parse(Rule::vector, "v[s|a|,s|b|]").unwrap();
        let outer = p.into_iter().next().unwrap();
        assert_eq!(outer.as_rule(), Rule::vector);
    }

    // ─── P2.4: Modifier Parsing ────────────────────────────────────

    #[test]
    fn parse_modifier() {
        let p = TySONParser::parse(Rule::modifier, "asc(root)").unwrap();
        let outer = p.into_iter().next().unwrap();
        assert_eq!(outer.as_rule(), Rule::modifier);
    }

    #[test]
    fn parse_not_modifier() {
        let p = TySONParser::parse(Rule::modifier, "not(eq{root:b|true|})").unwrap();
        let outer = p.into_iter().next().unwrap();
        assert_eq!(outer.as_rule(), Rule::modifier);
    }

    // ─── P2.5: Journal Parsing (full deserialization) ──────────────

    #[test]
    fn deserialize_string_primitive_via_journal() {
        let result = TySONParser::parse(Rule::journal, "s|key|:s|hello|");
        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_map_via_transaction_string() {
        let result = TySONParser::parse(Rule::journal, "collection|test|:insert[m{s|k|:s|v|}]");
        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_find_query_via_journal() {
        let result = TySONParser::parse(Rule::journal, "collection|test|:find[eq{s|x|:s|y|}]");
        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_q_transaction() {
        let result = TySONParser::parse(Rule::journal, "collection|test|:q[insert[m{s|name|:s|Ann|}]]");
        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_insert_transaction() {
        let result = TySONParser::parse(Rule::journal, "collection|test|: insert[s|hello|];");
        assert!(result.is_ok(), "failed: {:?}", result.err());
    }

    #[test]
    fn deserialize_transaction_no_semicolon() {
        let result = TySONParser::parse(Rule::journal, "collection|test|: insert[s|hello|]");
        assert!(result.is_ok(), "failed: {:?}", result.err());
    }

    #[test]
    fn transaction_deserialize_works() {
        let txn = crate::storage::transaction::Transaction::deserialize(
            "".to_string(),
            "collection|test|: insert[s|hello|];".to_string(),
        );
        assert!(txn.is_ok(), "Transaction::deserialize failed: {:?}", txn.err());
    }

    #[test]
    fn transaction_deserialize_q_format() {
        let txn = crate::storage::transaction::Transaction::deserialize(
            "".to_string(),
            "collection|test|:q[insert[m{s|name|:s|Ann|}]]".to_string(),
        );
        assert!(txn.is_ok(), "q format failed: {:?}", txn.err());
    }

    #[test]
    fn parse_empty_journal() {
        let result = TySONParser::parse(Rule::journal, "");
        assert!(result.is_ok());
    }

    #[test]
    fn parse_pair() {
        let p = TySONParser::parse(Rule::pair, "s|k|:s|v|").unwrap();
        let outer = p.into_iter().next().unwrap();
        assert_eq!(outer.as_rule(), Rule::pair);
    }

    #[test]
    fn parse_invalid_primitive_prefix() {
        let p = TySONParser::parse(Rule::primitive, "x|bad|");
        assert!(p.is_ok()); // PEG parses it; semantics check happens later
    }

    #[test]
    fn parse_link_primitive() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let input = format!("l|{}|", uuid);
        let p = TySONParser::parse(Rule::primitive, &input).unwrap();
        let outer = p.into_iter().next().unwrap();
        let mut inner = outer.into_inner();
        let prefix = inner.next().unwrap();
        assert_eq!(prefix.as_str(), "l");
    }
}
