use quick_xml::{Reader, events::Event};

use crate::Mode;

/// XML 開始マーカー
const MARKER: &[u8] = b"<success>\n";

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
                self.buf.extend_from_slice(chunk);

                let mut depth = 0i32;
                let mut doc_end: Option<usize> = None;

                {
                    // quick-xmlのReaderはbuf全体を読むため、借用ブロックを作る。
                    // ev_bufはquick-xmlがイベントを書き出すための作業バッファ。
                    let mut reader = Reader::from_reader(&self.buf[..]);
                    let mut ev_buf = Vec::new();

                    loop {
                        match reader.read_event_into(&mut ev_buf) {
                            Ok(Event::Start(_)) => {
                                depth += 1;
                            }
                            Ok(Event::End(_)) => {
                                depth -= 1;
                            }
                            Ok(Event::Eof) => break, // buf 走査完了 (xml doc未完成の可能性あり)
                            Ok(_) => { /* Text, Empty */ }
                            Err(_) => break, // TODO: 一時的。あとで直す
                        }

                        if depth == 0 {
                            doc_end = Some(reader.buffer_position() as usize); // 1 文書
                            break;
                        }
                    }
                } // reader dropped

                // XML が完結したか？
                match doc_end {
                    Some(end) => {
                        let doc = self.buf[..end].to_vec(); // 1文書ぶんを切り出し
                        self.buf.drain(..end); // 残りは持ち越し
                        vec![Segment::Xml(doc)]
                    }
                    None => Vec::new(), // まだ完結していない → 次のfeedへ
                }
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
    // Mode::Xml で分割された XML 文書が渡ってきたときは Segment::Xml を返す。
    #[test]
    fn returns_single_xml_segment_for_chunked_document() {
        let mut s = Scanner::new();
        s.feed(b"<success>\n"); // MARKER で xml mode
        s.feed(b"  <command>\n"); // 途中
        s.feed(b"    <switch>\n      XML\n    </switch>\n"); // まだ途中

        let last = b"  </command>\n</success>\n"; // ここで完結
        let doc = b"<success>\n  <command>\n    <switch>\n      XML\n    </switch>\n  </command>\n</success>";

        assert_eq!(s.feed(last), vec![Segment::Xml(doc.to_vec())]);
    }

    // Xml モードで文書が1つ完成したら、その文書を1つの Segment::Xml にして返す。
    #[test]
    fn returns_single_xml_segment_for_complete_document() {
        let mut s = Scanner::new();

        s.feed(b"<success>\n"); // switch to xml mode

        let body =
            b"  <command>\n    <switch>\n      XML\n    </switch>\n  </command>\n</success>\n";
        let doc = b"<success>\n  <command>\n    <switch>\n      XML\n    </switch>\n  </command>\n</success>";
        assert_eq!(s.feed(body), vec![Segment::Xml(doc.to_vec())]);
    }

    // Mode::English で完結した XML 文書が渡ってきたときは空 Plain をひとつ含む vec を返す。
    // モード切り替えという副作用のみで、 Segment::Xml は返さない。
    #[test]
    fn returns_blank_plain_with_complete_xml() {
        let mut s = Scanner::new();
        let input = b"<success>\n  <command>\n    <switch>\n      XML\n    </switch>\n  </command>\n</success>\n";
        let result: Vec<Segment> = vec![Segment::Plain(b"".into())];
        assert_eq!(s.feed(input), result);
    }

    // XML 開始タグを見たら mode を切り替える
    #[test]
    fn switches_xml_mode() {
        let mut s = Scanner::new();
        s.feed(b">: \n<success>\n  ");
        assert!(matches!(s.mode, Mode::Xml));
    }

    // XML 開始タグが chunk をまたいでいても mode を切り替える
    #[test]
    fn switches_xml_mode_splited() {
        let mut s = Scanner::new();
        s.feed(b">: \n<succ");
        s.feed(b"ess>\n");
        assert!(matches!(s.mode, Mode::Xml));
    }

    // 末尾が MARKER でなければ emit する
    #[test]
    fn emits_non_markers() {
        let mut s = Scanner::new();
        let bytes = b";login: ";
        assert_eq!(s.feed(bytes), vec![Segment::Plain(bytes.to_vec())]);
    }
}
