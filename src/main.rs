// git-sheets CLI: For Excel sufferers who deserve version control
//
// Usage:
//   git-sheets snapshot <file.csv> -m "message"
//   git-sheets diff <snapshot1> <snapshot2>
//   git-sheets verify <snapshot>
//   git-sheets init
//   git-sheets status

use clap::Parser;
use gitsheets::Cli;

fn main() {
    let cli = Cli::parse();

    // The CLI module handles all command execution
    if let Err(e) = cli.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
