use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    Deploy {
        stacks: Vec<String>,
        #[arg(short, long)]
        prune: bool,
    },
    Recreate {
        services: Vec<String>,
    },
    Rm {
        services: Vec<String>,
    },
    Rollback {
        services: Vec<String>,
    },
    Services,
    Nodes,
    Prune {
        #[arg(short, long)]
        volumes: bool,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}
