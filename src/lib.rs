mod command;
mod parser;
mod renderer;
mod scanner;
pub mod session;
mod translator;

/// ゲームの出力モード。
/// `switch` コマンドの引数であり、 scanner の解釈対象。
pub enum Mode {
    English,
    Xml,
}
