use std::process::Command;

use exitcode::ExitCode;

/// Runs the process interactively allowing user to see stdout and use stdin
pub fn run_interactive(program: &str, args: Vec<&str>, cwd: Option<&str>) -> ExitCode {
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    match cmd.spawn() {
        Err(err) => {
            eprintln!("{}", err);
            exitcode::SOFTWARE
        }
        Ok(mut v) => match v.wait() {
            Err(_) => {
                eprintln!("failed to execute");
                exitcode::OSERR
            }
            Ok(v) => v.code().unwrap_or_default(),
        },
    }
}
