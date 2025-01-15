use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, PartialEq, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Command {
    Check {
        #[arg(short, long)]
        archive: Option<String>,
    },
    Run {
        #[arg(short, long)]
        archive: Option<String>,
    },
}

#[test]
fn explicit_file() {
    let x = Cli {
        file: Some("explicit.yaml".into()),
        command: Command::Check { archive: None },
    };
    let y = Cli::try_parse_from(["test", "-f", "explicit.yaml", "check"]).unwrap();
    assert_eq!(x, y);
}

#[test]
fn explicit_archive() {
    let x = Cli {
        file: None,
        command: Command::Check {
            archive: Some("archive".to_string()),
        },
    };
    let y = Cli::try_parse_from(["test", "check", "-a", "archive"]).unwrap();
    assert_eq!(x, y);
}
