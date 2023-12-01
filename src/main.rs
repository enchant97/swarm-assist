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

fn command_remove_stack(stacks: &[String]) {
    let mut command_args = vec!["stack", "rm"];
    command_args.extend(stacks.into_iter().map(|v| v.as_str()));
    let exit_code = run_interactive("docker", command_args, None);
    if exitcode::is_error(exit_code) {
        exit(exit_code);
    }
}

fn command_recreate(services: &[String]) {
    for exit_code in services.iter().map(|service_name| {
        run_interactive(
            "docker",
            vec!["service", "update", "--force", service_name],
            None,
        )
    }) {
        if exitcode::is_error(exit_code) {
            exit(exit_code);
        }
    }
}

fn command_remove_service(services: &[String]) {
    let mut command_args = vec!["service", "rm"];
    command_args.extend(services.into_iter().map(|v| v.as_str()));
    let exit_code = run_interactive("docker", command_args, None);
    if exitcode::is_error(exit_code) {
        exit(exit_code);
    }
}

fn command_rollback(services: &[String]) {
    for exit_code in services.iter().map(|service_name| {
        run_interactive("docker", vec!["service", "rollback", service_name], None)
    }) {
        if exitcode::is_error(exit_code) {
            exit(exit_code);
        }
    }
}

fn command_list_services() {
    let exit_code = run_interactive("docker", vec!["service", "ls"], None);
    if exitcode::is_error(exit_code) {
        exit(exit_code);
    }
}

fn command_list_nodes() {
    let exit_code = run_interactive("docker", vec!["node", "ls"], None);
    if exitcode::is_error(exit_code) {
        exit(exit_code);
    }
}

fn command_system_prune(volumes: bool) {
    let mut command_args = vec!["system", "prune", "--all"];
    if volumes {
        command_args.push("--volumes");
    }
    let exit_code = run_interactive("docker", command_args, None);
    if exitcode::is_error(exit_code) {
        exit(exit_code);
    }
}

fn command_system_info() {
    let exit_code = run_interactive("docker", vec!["system", "info"], None);
    if exitcode::is_error(exit_code) {
        exit(exit_code);
    }
}

fn command_stats() {
    let exit_code = run_interactive("docker", vec!["stats"], None);
    if exitcode::is_error(exit_code) {
        exit(exit_code);
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
        args::Command::RmStack { stacks } => command_remove_stack(&stacks),
        args::Command::Recreate { services } => command_recreate(&services),
        args::Command::Rm { services } => command_remove_service(&services),
        args::Command::Rollback { services } => command_rollback(&services),
        args::Command::Services => command_list_services(),
        args::Command::Nodes => command_list_nodes(),
        args::Command::Prune { volumes } => command_system_prune(volumes),
        args::Command::Info => command_system_info(),
        args::Command::Stats => command_stats(),
    }
}
