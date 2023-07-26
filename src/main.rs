use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    message: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("allow");
    Ok(())
}
