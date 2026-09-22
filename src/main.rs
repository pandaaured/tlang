pub mod grammar;
pub mod parsing;
pub mod scanning;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    input: PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!();
}
