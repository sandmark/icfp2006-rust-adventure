use crate::{
    parser::{Command, Response, parse},
    scanner::Segment,
    translator::{DataBase, translate_text},
};
use std::io::{self, Write};

/// Renderer
pub struct Renderer {
    db: DataBase,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            db: DataBase::new(),
        }
    }

    pub fn render(&mut self, seg: &Segment, out: &mut impl Write) -> io::Result<()> {
        match seg {
            Segment::Plain(bytes) => out.write_all(bytes),
            Segment::Xml(bytes) => match parse(bytes) {
                Err(e) => out.write_all(format!("[PARSE ERROR] {}\n", e.to_string()).as_bytes()),
                Ok(resp) => out.write_all(self.render_response(resp).as_bytes()),
            },
        }
    }

    fn render_response(&self, resp: Response) -> String {
        match resp {
            Response::Help(s) => format!("[HELP] {s}"),
            Response::Error(s) => format!("[ERROR] {s}"),
            Response::Failed(s) => format!("[FAILED] {s}"),
            Response::Success(cmd) => match cmd {
                Command::Switch(mode) => format!("`switch` mode: {mode}"),
                Command::Look(r) => format!("{}\n", r.to_japanese(&self.db)),
                Command::Go(r) => format!("{}\n", r.to_japanese(&self.db)),
                Command::Show(items) => {
                    let coll = items.iter().map(|i| i.to_string()).collect::<Vec<_>>();
                    let s = if coll.is_empty() {
                        "何も持っていない"
                    } else {
                        &coll.join("\n")
                    };
                    format!("--- Inventory {}/6---\n\n{s}\n", items.len())
                }
                Command::Take(item) => format!("{} を拾った\n", item.name),
                Command::Examine(item) => format!("{item}"),
                Command::Incinerate(item) => format!("{} を火葬した…\n", item.name),
                Command::Combine(items) => {
                    format!("{} と {} を組み合わせた\n", items[0].name, items[1].name)
                }

                Command::Use(item, message) => format!(
                    "{} を使った\n---\n{}",
                    item.name,
                    translate_text(&self.db, &message.to_owned())
                ),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod response {
        use super::*;
        use crate::parser::{Condition, Description, Item, Room};

        #[test]
        fn renders_use() {
            let resp = Response::Success(Command::Use(
                Item {
                    name: "manifesto".to_owned(),
                    description: Description::Redacted,
                    adjectives: vec![],
                    condition: Condition::Pristine,
                    piled_on: None,
                },
                "Used manifesto.".to_owned(),
            ));
            assert!(!Renderer::new().render_response(resp).contains("TODO"));
        }

        #[test]
        fn renders_examine() {
            let resp = Response::Success(Command::Examine(Item {
                name: "manifesto".to_owned(),
                description: Description::Redacted,
                adjectives: vec![],
                condition: Condition::Pristine,
                piled_on: None,
            }));
            assert!(!Renderer::new().render_response(resp).contains("TODO"))
        }

        #[test]
        fn renders_take() {
            let resp = Response::Success(Command::Take(Item {
                name: "manifesto".to_owned(),
                description: Description::Redacted,
                adjectives: vec![],
                condition: Condition::Pristine,
                piled_on: None,
            }));
            assert!(!Renderer::new().render_response(resp).contains("TODO"))
        }

        #[test]
        fn renders_incinerate() {
            let resp = Response::Success(Command::Incinerate(Item {
                name: "manifesto".to_owned(),
                description: Description::Redacted,
                adjectives: vec![],
                condition: Condition::Pristine,
                piled_on: None,
            }));
            assert!(!Renderer::new().render_response(resp).contains("TODO"))
        }

        #[test]
        fn renders_combine() {
            let resp = Response::Success(Command::Combine(vec![
                Item {
                    name: "foo".to_owned(),
                    description: Description::Redacted,
                    adjectives: vec![],
                    condition: Condition::Pristine,
                    piled_on: None,
                },
                Item {
                    name: "bar".to_owned(),
                    description: Description::Redacted,
                    adjectives: vec![],
                    condition: Condition::Pristine,
                    piled_on: None,
                },
            ]));
            assert!(!Renderer::new().render_response(resp).contains("TODO"))
        }

        #[test]
        fn renders_show_inventory() {
            let resp = Response::Success(Command::Show(vec![]));
            assert!(!Renderer::new().render_response(resp).contains("TODO"))
        }

        #[test]
        fn renders_go_room() {
            let resp = Response::Success(Command::Go(Room {
                name: "testroom".to_owned(),
                description: Description::Redacted,
                items: vec![],
            }));
            assert!(!Renderer::new().render_response(resp).contains("TODO"))
        }

        #[test]
        fn renders_look_room() {
            let resp = Response::Success(Command::Look(Room {
                name: "testroom".to_owned(),
                description: Description::Redacted,
                items: vec![],
            }));
            assert!(!Renderer::new().render_response(resp).contains("TODO"))
        }

        // 正常系: switch はモード切替
        #[test]
        fn switch_renders_nothing() {
            let resp = Response::Success(Command::Switch("XML".to_owned()));
            assert_eq!(Renderer::new().render_response(resp), "`switch` mode: XML");
        }

        // 正常系: Error も封筒を剥がし、中の散文をそのまま描画する
        #[test]
        fn renders_error_text() {
            let resp = Response::Error("Huh? Try 'help'.".to_owned());
            assert_eq!(
                Renderer::new().render_response(resp),
                "[ERROR] Huh? Try 'help'."
            );
        }

        // 正常系: Help は XML 封筒を剥がし、中の散文をそのまま描画する
        #[test]
        fn renders_help_text() {
            let resp = Response::Help("examine: Inspect an item.".to_owned());
            assert_eq!(
                Renderer::new().render_response(resp),
                "[HELP] examine: Inspect an item."
            );
        }
    }

    #[test]
    fn renders_segment_to_out() {
        let input = b"hello, um";
        let plain = Segment::Plain(input.to_vec());
        let mut output: Vec<u8> = Vec::new();
        let mut r = Renderer::new();
        r.render(&plain, &mut output).expect("render failed");
        assert_eq!(output, b"hello, um");
    }
}
