use anyhow::{Result, bail};
use roxmltree::Node;

/// DOM: アイテムの説明
#[derive(Debug, PartialEq)]
enum Description {
    Text(String),
    Redacted,
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

#[cfg(test)]
mod tests {
    use super::*;

    mod description {
        use super::*;
        use roxmltree::Document;

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
