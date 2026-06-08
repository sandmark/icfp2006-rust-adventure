use std::{
    io::{self, Read, Write},
    process::{Child, Command},
};

fn run(interpreter_path: &str, vm_path: &str) -> anyhow::Result<Child> {
    let child = Command::new(interpreter_path).spawn()?;

    Ok(child)
}

fn relay(mut reader: impl Read, mut writer: impl Write) -> io::Result<u64> {
    io::copy(&mut reader, &mut writer)
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
