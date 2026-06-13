use crate::Mode;

pub enum Command {
    /// "login" や "./adventure" など fire-and-forget 用、戻り値なし
    Raw(String),

    /// ゲーム内コマンド。戻り値 (XML) あり。将来 lazy で成功捕捉する。
    Switch(Mode),
}

impl Command {
    /// 子 stdin へ送るバイト列へエンコードする。
    pub fn encode(&self) -> Vec<u8> {
        self.dispatch().as_bytes().to_vec()
    }

    fn dispatch(&self) -> &str {
        match self {
            Self::Raw(s) => s,
            Self::Switch(mode) => Self::switch(mode),
        }
    }

    fn switch(mode: &Mode) -> &'static str {
        match mode {
            Mode::English => "switch english\n",
            Mode::Xml => "switch xml\n",
        }
    }
}

/// 起動シーケンスを 1 コマンドずつ生産する lazy stream.
pub fn boot_script() -> impl Iterator<Item = Command> {
    [
        Command::Raw("howie\n".into()),
        Command::Raw("xyzzy\n".into()),
        Command::Raw("./adventure\n".into()),
        Command::Switch(Mode::Xml),
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 正常系: encode: Command::Raw をパススルー
    #[test]
    fn encode_relays_commands_raw() {
        let input = "hello, um";
        let cmd = Command::Raw(input.into());
        assert_eq!(cmd.encode(), input.as_bytes());
    }

    // 正常系: encode: Command::Switch(XML) 展開
    #[test]
    fn switch_xml_mapped_as_bytes() {
        let cmd = Command::Switch(Mode::Xml);
        assert_eq!(cmd.encode(), "switch xml\n".as_bytes());
    }

    // 正常系: encode: Command::Switch(English) 展開
    #[test]
    fn switch_english_mapped_as_bytes() {
        let cmd = Command::Switch(Mode::English);
        assert_eq!(cmd.encode(), "switch english\n".as_bytes());
    }
}
