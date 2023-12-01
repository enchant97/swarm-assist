use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
    vec,
};

use clap::Parser;

pub mod args;
pub mod helpers;
pub mod runner;

use args::Args;
use glob::glob;
use helpers::home_path;
use runner::run_interactive;

fn command_deploy(stack_conf_root: &Path, stacks: &Vec<String>, prune: bool) {
    if stacks.is_empty() {
        for entry in glob(stack_conf_root.join("*.yml").to_str().unwrap())
            .expect("glob pattern processing failed")
        {
            match entry {
                Ok(path) => println!(
                    "{:?}",
                    path.file_name().expect("failed to get basename of path")
                ),
                Err(e) => println!("{:?}", e),
            }
        }
    } else {
        for exit_code in stacks.iter().map(|stack_name| {
            let compose_file = stack_conf_root.join(format!("{}.yml", stack_name));
            let mut command_args = vec!["stack", "deploy"];
            if prune {
                command_args.push("--prune");
            }
            command_args.extend(vec!["-c", compose_file.to_str().unwrap(), stack_name]);
            run_interactive(
                "docker",
                command_args,
                Some(stack_conf_root.to_str().unwrap()),
            )
        }) {
            if exitcode::is_error(exit_code) {
                exit(exit_code);
            }
        }
    }
}

fn main() {
    let stack_conf_root = env::var("STACK_CONF_ROOT")
        .map(PathBuf::from)
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
        args::Command::Deploy { stacks, prune } => command_deploy(&stack_conf_root, &stacks, prune),
        _ => {
            todo!()
        }
    }
}
