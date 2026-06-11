use std::{
    io::{stdin, stdout},
    thread,
};

use anyhow::{Context, Result};
use clap::Parser;
use icfp2006_rust_adventure::{relay, run};

#[derive(Parser, Debug)]
#[clap(author = "sandmark", version, about)]
/// Application configuration
struct Args {
    /// path to the UMIX interpreter
    #[arg(short = 'i')]
    interpreter: String,

    /// path to the VM file
    #[arg()]
    vm: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut child = run(args.interpreter.as_str(), args.vm.as_str())?;
    let child_stdout = child.stdout.take().context("taking child stdout pipe")?;
    let child_stdin = child.stdin.take().context("taking child stdin pipe")?;

    // UM writer: 親stdin -> 子stdin スレッド
    thread::spawn(move || relay(stdin(), child_stdin));

    // UM reader: 子stdout -> 親stdout (mainスレッド)
    relay(child_stdout, stdout())?;

    Ok(())
}
