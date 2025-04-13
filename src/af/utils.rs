use crate::consts::*;
use anyhow::Context;
use clio::ClioPath;
use console::style;
use log::trace;
use std::env;
use std::process::{Command, Output};

pub fn run_command(command: &str, args: &[&str]) -> anyhow::Result<Output> {
    let output = Command::new(command)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute command: {} {:?}", command, args))?;

    trace!(
        "Running '{} {}'",
        style(command).bold(),
        style(args.join(" ")).bold()
    );
    trace!("  Status: {}", output.status);
    trace!("  Stdout: {}", String::from_utf8_lossy(&output.stdout));
    trace!("  Stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(output)
}

pub fn format_directory(directory: &ClioPath) -> String {
    directory
        .display()
        .to_string()
        .replace(env::var(HOME).unwrap_or_default().as_str(), "~")
}
