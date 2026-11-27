#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-ydata-profiling", about = "ydata-profiling automated dataset profiling for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-ydata-profiling: no entries configured."),
        Commands::Status => println!("sigmaos-ydata-profiling: ready."),
    }
}
