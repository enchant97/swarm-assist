use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Deploy a stack
    Deploy {
        stacks: Vec<String>,
        #[arg(short, long)]
        prune: bool,
    },
    /// List currently deployed stacks
    LsStacks,
    /// Show deployed services for specific stack
    PsStack { stack: String },
    /// Remove a stack
    RmStack { stacks: Vec<String> },
    /// Force a service to rebuild and pull fresh image
    Recreate { services: Vec<String> },
    /// Remove a deployed service
    Rm { services: Vec<String> },
    /// Revert a service to previous state
    Rollback { services: Vec<String> },
    /// List services
    Services,
    /// List swarm nodes
    Nodes,
    /// Prune current node's data
    Prune {
        #[arg(short, long)]
        volumes: bool,
    },
    /// Show Docker info for current node
    Info,
    /// Show stats for current node
    Stats,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}
