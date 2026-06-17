//! # XML Parser
//!
//! XML 構造そのままだと縦長になるので便宜上 edn 形式で記述している。
//!
//! ## `switch xml`
//! ```text
//! {:success
//!  {:command
//!   {:switch "XML"}}}
//! ```
//!
//! ## `examine` (`go north`)
//!
//! ```text
//! {:success
//!  {:command
//!   {:go
//!    {:room
//!     {:name "Room With a Door"
//!      :description "You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north."
//!      :items
//!       [{:item
//!         {:name "pamphlet"
//!          :description "standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of "One Resident's Trash Is Another Resident's Treasure," unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day"
//!          :adjectives []
//!          :condition :pristine
//!          :piled_on
//!           [{:item
//!             {:name "manifesto"
//!              :description :redacted
//!              :adjectives []
//!              :condition :pristine
//!              :piled_on []
//! ```
use anyhow::{Context, Result, bail};
use roxmltree::Node;

/// DOM: アイテムの説明
#[derive(Debug, PartialEq)]
enum Description {
    Text(String),
    Redacted,
}

/// DOM: アイテムの見た目
#[derive(Debug, PartialEq)]
enum Adjective {
    Red,
    Green,
    Other(String),
}

/// DOM: アイテムの状態
#[derive(Debug, PartialEq)]
enum Condition {
    Pristine,
    Broken(Broken),
}

/// DOM: 壊れたアイテムと必要な依存関係
#[derive(Debug, PartialEq)]
struct Broken {
    condition: Box<Condition>,
    missing: Vec<Kind>,
}

#[derive(Debug, PartialEq)]
struct Kind {
    name: String,
    condition: Condition,
}

impl Broken {
    fn new(condition: Condition, missing: Vec<Kind>) -> Self {
        Self {
            condition: Box::new(condition),
            missing,
        }
    }
}

impl Kind {
    fn new(name: String, condition: Condition) -> Self {
        Self { name, condition }
    }
}

fn parse_description(node: Node) -> Result<Description> {
    if let Some(child) = node.first_element_child() {
        match child.tag_name().name() {
            "redacted" => Ok(Description::Redacted),
            _ => bail!("undefined description inner tag"),
        }
    } else {
        Ok(Description::Text(
            node.text().unwrap_or_default().to_owned(),
        ))
    }
}

fn parse_adjective(node: Node) -> Result<Adjective> {
    let text = node.text().unwrap_or_default().trim();
    match text {
        "red" => Ok(Adjective::Red),
        "green" => Ok(Adjective::Green),
        "" => bail!("empty adjective"),
        other => Ok(Adjective::Other(other.to_string())),
    }
}

fn parse_adjectives(node: Node) -> Result<Vec<Adjective>> {
    node.children()
        .filter(|child| child.is_element())
        .map(parse_adjective)
        .collect::<Result<Vec<_>>>()
}

fn parse_condition(node: Node) -> Result<Condition> {
    let child = node
        .first_element_child()
        .context("taking first element of condition")?;

    match child.tag_name().name() {
        "pristine" => Ok(Condition::Pristine),
        "broken" => {
            let condition_node = child
                .children()
                .find(|n| n.has_tag_name("condition"))
                .context("no condition tag in broken")?;
            let missing_node = child
                .children()
                .find(|n| n.has_tag_name("missing"))
                .context("no missing tag in broken")?;
            let condition = parse_condition(condition_node)?;
            let missing = parse_missing(missing_node)?;
            Ok(Condition::Broken(Broken::new(condition, missing)))
        }
        _ => bail!("malformed condition inner"),
    }
}

fn parse_kind(node: Node) -> Result<Kind> {
    let name_node = node
        .children()
        .find(|n| n.has_tag_name("name"))
        .context("kind without inner name")?;
    let condition_node = node
        .children()
        .find(|n| n.has_tag_name("condition"))
        .context("kind without inner condition")?;
    let condition = parse_condition(condition_node)?;
    let name = name_node.text().unwrap_or_default().trim().to_owned();

    Ok(Kind { name, condition })
}

fn parse_missing(node: Node) -> Result<Vec<Kind>> {
    node.children()
        .filter(|n| n.is_element())
        .map(parse_kind)
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree::Document;

    mod kind {
        use super::*;

        // 正常系: <missing> 内の <kind> 群 → Vec<Kind>（複数）
        #[test]
        fn test_missing_collects_kinds() {
            let input = "<missing>\n <kind><name>transistor</name><condition><pristine></pristine></condition></kind>\n <kind><name>antenna</name><condition><pristine></pristine></condition></kind>\n </missing>";
            let doc = Document::parse(input).unwrap();

            assert_eq!(
                parse_missing(doc.root_element()).unwrap(),
                vec![
                    Kind {
                        name: String::from("transistor"),
                        condition: Condition::Pristine
                    },
                    Kind {
                        name: String::from("antenna"),
                        condition: Condition::Pristine
                    },
                ]
            );
        }

        // 正常系: <kind> は name と condition を持つ → Kind に落ちる
        #[test]
        fn test_kind() {
            let input = "<kind>\n<name>antenna</name>\n<condition><pristine></pristine></condition>\n</kind>";
            let doc = Document::parse(input).unwrap();

            assert_eq!(
                parse_kind(doc.root_element()).unwrap(),
                Kind {
                    name: String::from("antenna"),
                    condition: Condition::Pristine,
                }
            );
        }
    }
    mod condition {
        use super::*;

        #[test]
        fn test_broken() {
            let input = "<condition><broken>\n  <condition><pristine></pristine></condition>\n  <missing>\n<kind>\n<name>\nantenna</name>\n<condition>\n<pristine></pristine>\n</condition>\n</kind>\n</missing>\n</broken>\n</condition>\n";
            let doc = Document::parse(input).unwrap();

            assert_eq!(
                parse_condition(doc.root_element()).unwrap(),
                Condition::Broken(Broken {
                    condition: Box::new(Condition::Pristine),
                    missing: vec![Kind::new("antenna".into(), Condition::Pristine)]
                })
            );
        }

        #[test]
        fn test_pristine() {
            let input = "\n<condition> <pristine> </pristine></condition>";
            let doc = Document::parse(input).unwrap();

            assert_eq!(
                parse_condition(doc.root_element()).unwrap(),
                Condition::Pristine
            );
        }
    }
    mod adjectives {
        use super::*;

        // 異常系: 中の <adjective> が空なら、その Err が plural 層まで伝播する
        #[test]
        fn test_empty_inner_adjective_is_err() {
            let input = "<adjectives><adjective></adjective></adjectives>";
            let doc = Document::parse(input).unwrap();

            assert!(parse_adjectives(doc.root_element()).is_err());
        }

        // 正常系: カタログ済み 1 件 → 要素 1 個の Vec
        #[test]
        fn test_one_covered_adjective() {
            let input = "<adjectives><adjective>red</adjective></adjectives>";
            let doc = Document::parse(input).unwrap();

            assert_eq!(
                parse_adjectives(doc.root_element()).unwrap(),
                vec![Adjective::Red]
            );
        }

        // 正常系: 子要素が無ければ空の Vec
        #[test]
        fn test_empty_adjectives() {
            let input = "<adjectives></adjectives>";
            let doc = Document::parse(input).unwrap();
            assert!(parse_adjectives(doc.root_element()).unwrap().is_empty());
        }

        // 正常系: 要素間の空白 text node は is_element で除かれ、空の Vec (偽 Err にしない)
        #[test]
        fn test_whitespace_only_is_empty() {
            let input = "\n<adjectives>\n              </adjectives>\n";
            let doc = Document::parse(input).unwrap();
            assert!(parse_adjectives(doc.root_element()).unwrap().is_empty());
        }
    }
    mod adjective {
        use super::*;

        // 異常系: 中身が空 (trim 後に "") なら Err
        #[test]
        fn test_empty_adjective() {
            let input = "<adjective></adjective>";
            let doc = Document::parse(input).unwrap();
            assert!(parse_adjective(doc.root_element()).is_err());
        }

        // 正常系: 未カタログ語は Other(String) に保持される
        #[test]
        fn test_unknown_adjective() {
            let input = "<adjective>unknown</adjective>";
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_adjective(doc.root_element()).unwrap(),
                Adjective::Other(String::from("unknown"))
            );
        }

        // 正常系: 前後の空白を trim してカタログ語に一致させる
        #[test]
        fn test_trims_surrounding_whitespace() {
            let input = "<adjective>\n        green\n       </adjective>";
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_adjective(doc.root_element()).unwrap(),
                Adjective::Green
            );
        }

        // 正常系: カタログ語は対応する variant へ (表駆動)
        #[test]
        fn test_covered_adjectives() {
            for (input, want) in [("red", Adjective::Red), ("green", Adjective::Green)] {
                let xml = format!("<adjective>{input}</adjective>");
                let doc = Document::parse(&xml).unwrap();

                assert_eq!(
                    parse_adjective(doc.root_element()).unwrap(),
                    want,
                    "input={input:?}"
                );
            }
        }
    }
    mod description {
        use super::*;

        #[test]
        fn test_undefined_inner_tag() {
            let input = "<description><undefined/></description>";
            let doc = Document::parse(input).unwrap();
            assert!(parse_description(doc.root_element()).is_err(),)
        }
        #[test]
        fn test_redacted() {
            let input = "<description><redacted/></description>";
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_description(doc.root_element()).unwrap(),
                Description::Redacted
            );
        }

        #[test]
        fn test_text() {
            let input = "<description>inner text</description>";
            let text = String::from("inner text");
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_description(doc.root_element()).unwrap(),
                Description::Text(text)
            );
        }
    }
}
