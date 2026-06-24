use crate::{
    parser::{Command, Response, parse},
    scanner::Segment,
};
use std::io::{self, Write};

/// Renderer
pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&mut self, seg: &Segment, out: &mut impl Write) -> io::Result<()> {
        match seg {
            Segment::Plain(bytes) => out.write_all(bytes),
            Segment::Xml(bytes) => match parse(bytes) {
                Err(e) => out.write_all(format!("[PARSE ERROR] {}\n", e.to_string()).as_bytes()),
                Ok(resp) => out.write_all(render_response(&resp).as_bytes()),
            },
        }
    }
}

fn render_response(resp: &Response) -> String {
    match resp {
        Response::Help(s) => format!("[HELP] {s}"),
        Response::Error(s) => format!("[ERROR] {s}"),
        Response::Failed(s) => format!("[FAILED] {s}"),
        Response::Success(cmd) => match cmd {
            Command::Switch(r) => format!("`switch` mode: {r}"),
            Command::Look(r) => format!("{r}\n"),
            Command::Go(r) => format!("{r}\n"),
            Command::Show(items) => {
                let coll = items.iter().map(|i| i.to_string()).collect::<Vec<_>>();
                let s = if coll.is_empty() {
                    "何も持っていない"
                } else {
                    &coll.join("\n")
                };
                format!("--- Inventory ---\n\n{s}\n")
            }
            Command::Take(item) => {
                format!("{} を拾った\n", item.name)
            }
            other => format!("[RENDER TODO] not implemented {other:?}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod response {
        use super::*;
        use crate::parser::{Condition, Description, Item, Room};

        #[test]
        fn renders_take() {
            let resp = Response::Success(Command::Take(Item {
                name: "manifesto".to_owned(),
                description: Description::Redacted,
                adjectives: vec![],
                condition: Condition::Pristine,
                piled_on: None,
            }));
            assert!(!render_response(&resp).contains("TODO"))
        }

        #[test]
        fn renders_show_inventory() {
            let resp = Response::Success(Command::Show(vec![]));
            assert!(!render_response(&resp).contains("TODO"))
        }

        #[test]
        fn renders_go_room() {
            let resp = Response::Success(Command::Go(Room {
                name: "testroom".to_owned(),
                description: Description::Redacted,
                items: vec![],
            }));
            assert!(!render_response(&resp).contains("TODO"))
        }

        #[test]
        fn renders_look_room() {
            let resp = Response::Success(Command::Look(Room {
                name: "testroom".to_owned(),
                description: Description::Redacted,
                items: vec![],
            }));
            assert!(!render_response(&resp).contains("TODO"))
        }

        // 正常系: switch はモード切替
        #[test]
        fn switch_renders_nothing() {
            let resp = Response::Success(Command::Switch("XML".to_owned()));
            assert_eq!(render_response(&resp), "`switch` mode: XML");
        }

        // 正常系: Error も封筒を剥がし、中の散文をそのまま描画する
        #[test]
        fn renders_error_text() {
            let resp = Response::Error("Huh? Try 'help'.".to_owned());
            assert_eq!(render_response(&resp), "[ERROR] Huh? Try 'help'.");
        }

        // 正常系: Help は XML 封筒を剥がし、中の散文をそのまま描画する
        #[test]
        fn renders_help_text() {
            let resp = Response::Help("examine: Inspect an item.".to_owned());
            assert_eq!(render_response(&resp), "[HELP] examine: Inspect an item.");
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
