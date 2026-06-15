use std::fmt::Debug;

use quick_xml::{Reader, events::Event};

use crate::Mode;

/// XML 開始マーカー
const MARKER: &[u8] = b"<success>\n";

#[derive(PartialEq)]
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

impl Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Segment::Plain(b) => write!(f, "Plain({:?})", String::from_utf8_lossy(b)),
            Segment::Xml(b) => write!(f, "Xml({:?})", String::from_utf8_lossy(b)),
        }
    }
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

                let mut out: Vec<Segment> = Vec::new();
                let mut depth = 0i32;
                let mut cursor = 0usize; // ここ未満は処理済 & 未処理領域の先頭
                let mut doc_start: Option<usize> = None; // XML の内側なら開始位置。でなければ None
                {
                    let mut reader = Reader::from_reader(&self.buf[..]);
                    let mut ev_buf = Vec::new();

                    loop {
                        // read 前の位置 = 次イベントの開始オフセット
                        // Start で '<' の位置を控えておく
                        let pos = reader.buffer_position() as usize;
                        match reader.read_event_into(&mut ev_buf) {
                            Ok(Event::Start(_)) => {
                                if depth == 0 {
                                    // 文書開始
                                    if pos > cursor {
                                        // 直前までの depth 0 バイトはXMLではない -> Plain
                                        out.push(Segment::Plain(self.buf[cursor..pos].to_vec()));
                                    }
                                    cursor = pos;
                                    doc_start = Some(pos); // '<' の位置
                                }
                                depth += 1;
                            }
                            Ok(Event::End(_)) => {
                                depth -= 1;
                                if depth == 0 {
                                    // 文書完結。 read した後の位置 = '>' の直後 = 終端。
                                    let end = reader.buffer_position() as usize;
                                    if let Some(start) = doc_start.take() {
                                        out.push(Segment::Xml(self.buf[start..end].to_vec()));
                                        cursor = end;
                                    }
                                }
                            }
                            Ok(Event::Eof) => break, // 走査完了。未完成文書の可能性あり
                            Ok(_) => { /* Text, Empty: depth0 は末尾で処理 */ }
                            Err(_) => break, // TODO: Stage3
                        }
                    }
                } // reader drop

                match doc_start {
                    // 文書が開いたまま終わった = 未完成。完成した分は out へ。残りは持ち越し。
                    Some(_) => {
                        self.buf.drain(..cursor);
                        out
                    }
                    None => {
                        if out.is_empty() {
                            // 文書ゼロ = 純粋な Plain
                            let plain: Vec<u8> = self.buf.drain(..).collect();
                            if plain.is_empty() {
                                Vec::new()
                            } else {
                                vec![Segment::Plain(plain)]
                            }
                        } else {
                            // 文書のあとに残った depth 0 バイト(改行など)は次のfeedへ持ち越し
                            self.buf.drain(..cursor);
                            out
                        }
                    }
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
    // Xml モードで報酬コード(深さ0のバイト)だけが届いたら、
    // 文書を待たずに [Plain] として即吐き出す。
    #[test]
    fn emits_reward_code_as_plain_without_document() {
        let mut s = Scanner::new();

        // 下準備: Xml モードに入り buf を空にする
        s.feed(b"<success>\n</success>");
        s.feed(b"");

        // 報酬コードのみ。
        let reward = b"ADVTR.INC=5@999999|f95731ab88952dfa4cb326fb99c085f\n";
        assert_eq!(s.feed(reward), vec![Segment::Plain(reward.to_vec())]);
    }

    // Xml モードで報酬コードのあとに完結した文書が続いたら、
    // 1回の feed で [Plain, Xml] を順番どおり返す。
    #[test]
    fn returns_plain_and_xml_when_reward_code_precedes_document() {
        let mut s = Scanner::new();

        s.feed(b"<success>\n</success>");
        s.feed(b"");
        assert!(matches!(s.mode, Mode::Xml));

        let input = b"ADVTR.INC=5@999999|f95731ab88952dfa4cb326fb99c085f\n<success></success>";
        assert_eq!(
            s.feed(input),
            vec![
                Segment::Plain(b"ADVTR.INC=5@999999|f95731ab88952dfa4cb326fb99c085f\n".to_vec()),
                Segment::Xml(b"<success></success>".to_vec()),
            ]
        );
    }

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
