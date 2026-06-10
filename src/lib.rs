use std::{
    io::{self, Read, Write},
    process::{Child, Command, Stdio},
};

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
