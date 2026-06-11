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
            Segment::Xml(bytes) => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
