use anyhow::{Result, bail};
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

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree::Document;

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
