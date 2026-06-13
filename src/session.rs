use std::{
    io::{self, Read, Write, stdin, stdout},
    process::{Child, Command, Stdio},
    thread,
};

use anyhow::Context;

use crate::{command::boot_script, renderer::Renderer, scanner::Scanner};

/// Session
pub struct Session {
    child: Child,
    scanner: Scanner,
    renderer: Renderer,
}

impl Session {
    pub fn new(interpreter: &str, vm: &str) -> anyhow::Result<Self> {
        let child = Command::new(interpreter)
            .arg(vm)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        Ok(Self {
            child: child,
            scanner: Scanner::new(),
            renderer: Renderer::new(),
        })
    }
    pub fn start(&mut self) -> anyhow::Result<()> {
        let mut child_stdout = self
            .child
            .stdout
            .take()
            .context("taking child stdout pipe")?;

        let mut child_stdin = self.child.stdin.take().context("taking child stdin pipe")?;

        // Boot adventure
        for cmd in boot_script() {
            // TODO: ロガーが欲しい。入力が消えちゃう。
            child_stdin.write_all(&cmd.encode())?;
        }

        thread::spawn(move || relay(stdin(), child_stdin));

        // stdout scan & render
        let mut buf = [0u8; 4096];
        let mut out = stdout();
        loop {
            let n = child_stdout.read(&mut buf)?;
            if n == 0 {
                break;
            }
            let segments = self.scanner.feed(&buf[..n]);
            for seg in segments {
                self.renderer.render(&seg, &mut out)?;
            }
            out.flush()?;
        }

        Ok(())
    }
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

    // 正常系: reader をそのまま writer に書く
    #[test]
    fn relays_bytes_verbatim() {
        let input: &[u8] = b"hello, um";
        let mut output: Vec<u8> = Vec::new();

        relay(input, &mut output).expect("relay failed");

        assert_eq!(output, b"hello, um");
    }

    // 異常系: 存在しないインタプリタは起動できない
    #[test]
    fn returns_err_when_interpreter_is_missing() {
        let session = Session::new("/no/such/interpreter", "irrelevant/vm/path");
        assert!(session.is_err());
    }
}
