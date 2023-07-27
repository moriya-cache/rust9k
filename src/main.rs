use anyhow::Result;
use clap::Parser;
use rust9000::{Decision, Filter};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    message: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut filter = Filter::new();

    match filter.check(&args.message) {
        Decision::Allow => println!("allow"),
        Decision::Reject => println!("reject"),
    }

    Ok(())
}
