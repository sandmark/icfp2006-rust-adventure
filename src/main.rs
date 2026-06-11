use anyhow::Result;
use clap::Parser;
use icfp2006_rust_adventure::session::Session;

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
    Session::new(&args.interpreter, &args.vm)?.start()?;
    Ok(())
}
