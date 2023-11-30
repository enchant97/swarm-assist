use std::{env, path::PathBuf, vec};

use clap::Parser;

pub mod args;
pub mod helpers;
pub mod runner;

use args::Args;
use helpers::home_path;
use runner::run_interactive;

fn main() {
    let stack_conf_root = env::var("STACK_CONF_ROOT")
        .map(|v| PathBuf::from(v))
        .unwrap_or_else(|_| {
            home_path()
                .expect("failed to get home directory")
                .join("stack.yml.d")
        });
    if !stack_conf_root.is_dir() {
        println!("configured stack root {:?} does not exist", stack_conf_root);
        return;
    }
    let args = Args::parse();
    match args.command {
        args::Command::Deploy { stacks, prune } => {
            if stacks.len() == 0 {
                todo!()
            } else {
                for stack_name in stacks {
                    let compose_file = stack_conf_root.join(format!("{}.yml", stack_name));
                    let mut command_args = vec!["stack", "deploy"];
                    if prune {
                        command_args.push("--prune");
                    }
                    command_args.extend(vec!["-c", compose_file.to_str().unwrap(), &stack_name]);
                    run_interactive(
                        "docker",
                        command_args,
                        Some(stack_conf_root.to_str().unwrap()),
                    );
                }
            }
        }
        _ => {
            todo!()
        }
    }
}
