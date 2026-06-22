use crate::{
    parser::{Response, parse},
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
        Response::Success(_) => "".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod response {
        use super::*;

        // 正常系: switch はモード切替の plumbing。seam を通しても out へは何も出ない。
        #[test]
        fn switch_renders_nothing() {
            let xml = b"<success><command><switch>XML</switch></command></success>";
            let seg = Segment::Xml(xml.to_vec());
            let mut out: Vec<u8> = Vec::new();
            let mut r = Renderer::new();
            r.render(&seg, &mut out).expect("render failed");
            assert!(out.is_empty());
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
