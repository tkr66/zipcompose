mod command;
mod compose;
mod manifest;

use std::io::Result;

use std::path::Path;

use clap::Parser;
use command::Cli;
use command::Command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let manifest = match cli.file {
        Some(f) => manifest::read(&f)?,
        None => manifest::read(Path::new("zip-compose.yaml"))?,
    };
    match cli.command {
        Command::Check { archive } => match archive {
            Some(a) => compose::check(&manifest, &a)?,
            None => compose::check_all(&manifest)?,
        },
        Command::Run { archive } => match archive {
            Some(a) => compose::run(&manifest, &a)?,
            None => compose::run_all(&manifest)?,
        },
    }

    Ok(())
}
