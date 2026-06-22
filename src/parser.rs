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
use roxmltree::{Document, Node};

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

#[derive(Debug, PartialEq)]
pub enum Command {
    Switch(String),
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Success(Command),
    Error(String),
    Help(String),
    Failed(String),
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

/// - switch
/// ```text
/// {:command {:switch string?}}
/// ```
fn parse_command(node: Node) -> Result<Command> {
    let child = node.first_element_child().context("empty command tag")?;
    match child.tag_name().name() {
        "switch" => Ok(Command::Switch(
            child
                .text()
                .context("empty switch result")?
                .trim()
                .to_owned(),
        )),
        other => {
            let src = &child.document().input_text()[child.range()];
            bail!("NOT IMPLEMENTED: {other}\n{src}");
        }
    }
}

fn parse_response_failed(node: Node) -> Result<Response> {
    let reason = node
        .children()
        .find(|n| n.has_tag_name("reason"))
        .context("failed tag has no reason")?
        .text()
        .context("reason tag has no text")?
        .trim();
    let command = node
        .children()
        .find(|n| n.has_tag_name("command"))
        .context("failed tag has no command")?
        .first_element_child()
        .context("command tag has no children")?;
    match command.tag_name().name().trim() {
        "go" => {
            let params = command.text().unwrap_or_default().trim();
            Ok(Response::Failed(format!(
                "failed to `go ({params})`: {reason}\n"
            )))
        }
        other => {
            let src = &node.document().input_text()[node.range()];
            bail!("failed to `{other}`:\n{src}\n");
        }
    }
}

/// - error
/// ```text
/// {:error
///  {:response string?}}
/// ```
///
/// - help
/// ```text
/// {:help string?}
/// ```
///
/// - success
/// ```text
/// {:success
///  {:command
///   {:switch string?}}}
/// ```
fn parse_response(node: Node) -> Result<Response> {
    match node.tag_name().name() {
        "error" => Ok(Response::Error(
            node.children()
                .find(|n| n.has_tag_name("response"))
                .context("error has no response")?
                .text()
                .context("response has no text")?
                .trim()
                .to_owned(),
        )),
        "help" => Ok(Response::Help(
            node.text().context("no message in help")?.trim().to_owned(),
        )),
        "success" => {
            let command_node = node
                .children()
                .find(|n| n.has_tag_name("command"))
                .context("success has no command")?;
            let command = parse_command(command_node)?;
            Ok(Response::Success(command))
        }
        "failed" => parse_response_failed(node),
        other => {
            let src = &node.document().input_text()[node.range()];
            bail!("unknown response: {other}\n{src}")
        }
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

    Ok(Kind::new(name, condition))
}

fn parse_missing(node: Node) -> Result<Vec<Kind>> {
    node.children()
        .filter(|n| n.is_element())
        .map(parse_kind)
        .collect::<Result<Vec<_>>>()
}

fn sanitize_help(text: &str) -> Result<String> {
    let inner = text
        .strip_prefix("<help>")
        .and_then(|s| s.strip_suffix("</help>"))
        .context("malformed <help> tags")?
        .trim();
    let escaped = inner.replace('<', "&lt;").replace('>', "&gt;");
    Ok(format!("<help>{escaped}</help>"))
}

pub fn parse(bytes: &[u8]) -> Result<Response> {
    let text = str::from_utf8(bytes)?.trim();

    let xml = if text.starts_with("<help>") {
        sanitize_help(text)?
    } else {
        text.to_owned()
    };

    let doc = Document::parse(&xml).context("parsing xml from bytes")?;
    parse_response(doc.root_element())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod entry {
        use super::*;

        // <help> に含まれる `<command>` をタグとしてパースしない
        #[test]
        fn test_help_command() {
            let bytes = b"<help>\nTry 'help <command>'\n</help>";
            assert_eq!(
                parse(bytes).unwrap(),
                Response::Help("Try 'help <command>'".to_owned())
            );
        }

        // 正常系: bytes を str→Document 化し、root から Response を組む（ここでは error 経路）
        #[test]
        fn test_bytes() {
            let bytes = b"<error><response>Huh?</response></error>";
            assert_eq!(parse(bytes).unwrap(), Response::Error("Huh?".to_owned()));
        }
    }
    mod response {
        use super::*;

        // 異常系: 不明なレスポンスの XML をエラーに含める
        #[test]
        fn test_unknown() {
            let input = "<unknown><message>unknown message</message></unknown>";
            let doc = Document::parse(input).unwrap();
            let msg = format!("{}", parse_response(doc.root_element()).unwrap_err());

            assert_eq!(
                msg,
                "unknown response: unknown\n<unknown><message>unknown message</message></unknown>"
            );
        }

        // 正常系: <failed> をパース
        #[test]
        fn test_failed() {
            let input = "<failed><command><go>north</go></command><reason>there is no way north from here</reason></failed>";
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_response(doc.root_element()).unwrap(),
                Response::Failed(
                    "failed to `go (north)`: there is no way north from here\n".to_string()
                )
            );
        }

        // 正常系: <help> 直下のテキストを trim して Response::Help に
        #[test]
        fn test_help() {
            let input = "<help>\n  examine: Inspect an item or your environment. Synonyms include ex, x, look, and l.\n </help>";
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_response(doc.root_element()).unwrap(),
                Response::Help("examine: Inspect an item or your environment. Synonyms include ex, x, look, and l.".to_owned())
            );
        }

        // 正常系: <error> 内の <response> テキストを trim して Response::Error に
        #[test]
        fn test_error() {
            let input = "<error> <response>\nHuh? Try 'help'.\n</response></error>";
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_response(doc.root_element()).unwrap(),
                Response::Error("Huh? Try 'help'.".to_owned())
            );
        }

        // 正常系: <success> → <command> → <switch> を辿り Response::Success(Command::Switch) に
        #[test]
        fn test_success() {
            let input = "\n<success>\n <command>\n<switch>XML</switch></command></success>";
            let doc = Document::parse(input).unwrap();
            assert_eq!(
                parse_response(doc.root_element()).unwrap(),
                Response::Success(Command::Switch("XML".to_owned()))
            );
        }
    }
    mod command {
        use super::*;

        // 異常系: 未実装コマンドはタグ名だけでなく元 XML 片をエラーに載せる
        #[test]
        fn test_unimplemented_command_dumps_xml() {
            let input = "<command><test_command><room>x</room></test_command></command>";
            let doc = Document::parse(input).unwrap();
            let msg = format!("{}", parse_command(doc.root_element()).unwrap_err());
            assert_eq!(
                msg,
                "NOT IMPLEMENTED: test_command\n<test_command><room>x</room></test_command>"
            );
        }

        // 正常系: <command> 直下の <switch> テキストを trim して Command::Switch に
        #[test]
        fn test_switch() {
            let input = "\n<command>\n<switch>\n XML\n</switch></command>";
            let doc = Document::parse(input).unwrap();

            assert_eq!(
                parse_command(doc.root_element()).unwrap(),
                Command::Switch("XML".to_owned())
            );
        }
    }
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
