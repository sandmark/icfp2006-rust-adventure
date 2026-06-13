use std::io::{self, Write};

/// Renderer
use crate::scanner::Segment;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&mut self, seg: &Segment, out: &mut impl Write) -> io::Result<()> {
        match seg {
            Segment::Plain(bytes) => out.write_all(bytes),
            Segment::Xml(bytes) => out.write_all(bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Segment::Xml は中身を検証せず、バイト列をそのまま out へ書き出す(エコー)。
    // 入力をわざと不完全な XML スニペットにして振る舞いを定義している。
    #[test]
    fn echoes_xml_segment_verbatim() {
        let input = b"\n</success>";
        let xml = Segment::Xml(input.to_vec());
        let mut output: Vec<u8> = Vec::new();
        let mut r = Renderer::new();
        r.render(&xml, &mut output).expect("render failed");
        assert_eq!(output, b"\n</success>");
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
