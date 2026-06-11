/// XML 開始マーカー
const MARKER: &[u8] = b"<success>\n";

/// ゲームエンジンのデータ構造 ≒ switch
enum Mode {
    /// デフォルト
    English,
    /// `switch xml`
    Xml,
}

#[derive(Debug, PartialEq)]
pub enum Segment {
    /// English 出力。素通し対象。
    Plain(Vec<u8>),

    /// 完結した XML ドキュメント。
    Xml(Vec<u8>),
}

/// 描画エンジン
pub struct Scanner {
    /// ゲームエンジンの出力データ構造
    mode: Mode,

    /// チャンクバッファ
    buf: Vec<u8>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            mode: Mode::English,
            buf: Vec::new(),
        }
    }
    pub fn feed(&mut self, chunk: &[u8]) -> Vec<Segment> {
        match self.mode {
            Mode::English => {
                self.buf.extend_from_slice(chunk);
                match find_subslice(&self.buf, MARKER) {
                    Some(pos) => {
                        // marker より前は Segment::Plain
                        let plain = self.buf[..pos].to_vec(); // 素通し部分は Plain で取り出す
                        self.buf = self.buf.split_off(pos); // marker以降は buf に持ち越し
                        self.mode = Mode::Xml; // モード切り替え
                        vec![Segment::Plain(plain)] // 返すのは Plain のみ
                    }
                    None => {
                        // 見つからないが末尾は marker の途中かもしれない。
                        // marker の prefix を探す。
                        let mut keep = 0;
                        for k in (1..MARKER.len()).rev() {
                            if self.buf.ends_with(&MARKER[..k]) {
                                keep = k;
                                break;
                            }
                        }

                        let emit = self.buf.len() - keep;
                        let plain = self.buf[..emit].to_vec();
                        self.buf.drain(..emit);
                        vec![Segment::Plain(plain)]
                    }
                }
            }
            Mode::Xml => {
                todo!("XML をまとめる")
            }
        }
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switches_xml_mode() {
        let mut s = Scanner::new();
        s.feed(b">: \n<success>\n  ");
        assert!(matches!(s.mode, Mode::Xml));
    }

    #[test]
    fn switches_xml_mode_splited() {
        let mut s = Scanner::new();
        s.feed(b">: \n<succ");
        s.feed(b"ess>\n");
        assert!(matches!(s.mode, Mode::Xml));
    }

    // 正常系: 末尾が MARKER でなければ emit
    #[test]
    fn emits_non_markers() {
        let mut s = Scanner::new();
        let bytes = b";login: ";
        assert_eq!(s.feed(bytes), vec![Segment::Plain(bytes.to_vec())]);
    }
}
