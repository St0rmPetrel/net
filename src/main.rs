use anyhow::Result;
use clap::{Parser, Subcommand};

mod cmd;
mod net;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send ICMP ECHO_REQUEST packets to network host
    Ping(cmd::Ping),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Ping(cmd) => cmd.exec()?,
    }
    Ok(())
}
