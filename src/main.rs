use std::io::{self, BufRead};

use anyhow::Result;
use clap::Parser;
use rust9000::{Decision, Filter};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(long)]
    stats: bool,

    message: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut filter = Filter::new();

    if let Some(message) = args.message {
        print_decision(filter.check(&message));
        return Ok(());
    }

    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        print_decision(filter.check(&line));
    }

    if args.stats {
        let stats = filter.stats();
        eprintln!("{} allowed, {} rejected", stats.allowed, stats.rejected);
    }

    Ok(())
}

fn print_decision(decision: Decision) {
    match decision {
        Decision::Allow => println!("allow"),
        Decision::Reject => println!("reject"),
    }
}
