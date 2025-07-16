// src/main.rs
/*
 * Main executable for ChronoChain
 */

use clap::Parser;
use chronochain::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChronoChain - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
