use std::{
    io::{Read, stdin, stdout},
    thread,
};

use anyhow::{Context, Result};
use clap::Parser;
use icfp2006_rust_adventure::{Renderer, relay, run};

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
    let mut child_stdout = child.stdout.take().context("taking child stdout pipe")?;
    let child_stdin = child.stdin.take().context("taking child stdin pipe")?;

    // UM writer: 親stdin -> 子stdin スレッド
    thread::spawn(move || relay(stdin(), child_stdin));

    // UM reader: 子stdout -> 親stdout (mainスレッド)
    // relay(child_stdout, stdout())?;

    let mut renderer = Renderer::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = child_stdout.read(&mut buf)?;
        if n == 0 {
            break;
        }
        renderer.feed(&buf[..n], &mut stdout())?;
    }

    Ok(())
}
