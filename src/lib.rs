use std::{
    io::{self, Read, Write},
    process::{Child, Command, Stdio},
};

/// ゲームエンジンのデータ構造 ≒ switch
enum Mode {
    /// デフォルト
    English,
    /// `switch xml`
    Xml,
}

/// 描画エンジン
pub struct Renderer {
    /// ゲームエンジンの出力データ構造
    mode: Mode,

    /// チャンクバッファ
    buf: Vec<u8>,
}

impl Renderer {
    /// デフォルト [Renderer] を返す。
    pub fn new() -> Self {
        Self {
            mode: Mode::English,
            buf: Vec::new(),
        }
    }

    /// reader が読み取った 1 チャンクを読み、描画結果を `writer` へ出力する。
    pub fn feed(&mut self, chunk: &[u8], writer: &mut impl Write) -> io::Result<()> {
        match self.mode {
            Mode::English => {
                todo!("境界判定 -> mode 切り替え -> 前半 writer / 後半 Xml 扱い");
            }
            Mode::Xml => {
                todo!("buf に追記 -> XML完結検出 -> parse -> render");
            }
        }
    }
}

pub fn run(interpreter_path: &str, vm_path: &str) -> anyhow::Result<Child> {
    let child = Command::new(interpreter_path)
        .arg(vm_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    Ok(child)
}

pub fn relay(mut reader: impl Read, mut writer: impl Write) -> io::Result<u64> {
    let mut buf = [0u8; 4096];
    let mut total: u64 = 0;
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            // EOF: um 終了
            break;
        }
        writer.write_all(&buf[..n])?;
        writer.flush()?;
        total += n as u64;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod renderer {
        use super::*;

        #[test]
        fn switch_xml_mode() {
            let mut r = Renderer::new();
            r.feed(b">: \n<success>\n  ", &mut io::sink()).unwrap();
            assert!(matches!(r.mode, Mode::Xml));
        }

        #[test]
        fn switch_xml_mode_splited() {
            let mut r = Renderer::new();
            r.feed(b">: \n<succ", &mut io::sink()).unwrap();
            r.feed(b"ess>\n", &mut io::sink()).unwrap();
            assert!(matches!(r.mode, Mode::Xml));
        }
    }

    mod relay {
        use super::relay;

        // 正常系: reader をそのまま writer に書く
        #[test]
        fn relays_bytes_verbatim() {
            let input: &[u8] = b"hello, um";
            let mut output: Vec<u8> = Vec::new();

            relay(input, &mut output).expect("relay failed");

            assert_eq!(output, b"hello, um");
        }
    }

    mod run {
        use super::run;

        // 異常系: 存在しないインタプリタは起動できない
        #[test]
        fn returns_err_when_interpreter_is_missing() {
            let result = run("/no/such/interpreter", "irrelevant/vm/path");
            assert!(result.is_err());
        }
    }
}
