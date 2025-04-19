use crate::consts::*;
use anyhow::{Context, Result};
use clio::ClioPath;
use console::style;
use log::trace;
use regex::Regex;
use std::env;
use std::process::{Command, Output};

pub fn run_command(command: &str, args: &[&str]) -> Result<Output> {
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

pub fn parse_repository(repository: &str) -> Result<String> {
    let repo = repository.trim();

    // check if in ssh format
    if Regex::new(r"^git@([^/]+):([^/]+)/([^/]+)\.git$")?.is_match(repo) {
        return Ok(repo.to_string());
    }

    // check if in http(s) format
    if Regex::new(r"^https?://([^/]+)/([^/]+)/([^/]+)(?:/.*?)?$")?.is_match(repo) {
        return Ok(repo.to_string());
    }

    anyhow::bail!(
        "Unsupported repository URL. Supported formats: [{}, {}]",
        style("git@<host>:<org>/<repo>.git").bold(),
        style("http[s]://<host>/<org>/<repo>[/...]").bold(),
    );
}

pub fn validate_repository(repository: &str) -> Result<()> {
    parse_repository(repository).map(|_| ())
}

pub fn convert_to_ssh<S: AsRef<str>>(repository: S) -> Result<String> {
    let repo = repository.as_ref().trim();

    let re_ssh = Regex::new(r"^git@([^/]+):([^/]+)/([^/]+)\.git$")?;
    if re_ssh.is_match(repo) {
        return Ok(repo.to_string());
    }

    let re_https = Regex::new(r"^https?://([^/]+)/([^/]+)/([^/]+)(?:/.*)?$")?;
    if let Some(caps) = re_https.captures(repo) {
        return Ok(format!("git@{}:{}/{}.git", &caps[1], &caps[2], &caps[3].trim_end_matches(".git")));
    }

    anyhow::bail!("Only http(s) repository URLs are supported for conversion");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_ssh_url() {
        let url = "git@github.com:org/repo.git";
        let parsed = parse_repository(url).unwrap();
        assert_eq!(parsed, url.trim());
    }

    #[test]
    fn parses_valid_https_url() {
        let url = "https://github.com/org/repo";
        let parsed = parse_repository(url).unwrap();
        assert_eq!(parsed, url.trim());
    }

    #[test]
    fn parses_valid_https_url_with_trailing_path() {
        let url = "https://github.com/org/repo/tree/main";
        let parsed = parse_repository(url).unwrap();
        assert_eq!(parsed, url.trim());
    }

    #[test]
    fn trims_whitespace_before_parsing() {
        let url = "  git@github.com:org/repo.git  ";
        let parsed = parse_repository(url).unwrap();
        assert_eq!(parsed, "git@github.com:org/repo.git");
    }

    #[test]
    fn fails_on_invalid_url() {
        let url = "ftp://github.com/org/repo";
        let err = parse_repository(url).unwrap_err().to_string();
        assert!(
            err.contains("Unsupported repository URL"),
            "unexpected error message: {}",
            err
        );
    }

    // convert_to_ssh

    #[test]
    fn return_ssh() {
        let url = "git@github.com:org/repo.git";
        let ssh = convert_to_ssh(url).unwrap();
        assert_eq!(ssh, "git@github.com:org/repo.git");
    }

    #[test]
    fn converts_https_to_ssh() {
        let url = "https://github.com/org/repo";
        let ssh = convert_to_ssh(url).unwrap();
        assert_eq!(ssh, "git@github.com:org/repo.git");
    }

    #[test]
    fn converts_http_to_ssh() {
        let url = "http://gitlab.com/group/project";
        let ssh = convert_to_ssh(url).unwrap();
        assert_eq!(ssh, "git@gitlab.com:group/project.git");
    }

    #[test]
    fn strips_trailing_git_suffix() {
        let url = "https://bitbucket.org/team/repo.git";
        let ssh = convert_to_ssh(url).unwrap();
        assert_eq!(ssh, "git@bitbucket.org:team/repo.git");
    }

    #[test]
    fn ignores_extra_path_parts() {
        let url = "https://github.com/org/repo/tree/main";
        let ssh = convert_to_ssh(url).unwrap();
        assert_eq!(ssh, "git@github.com:org/repo.git");
    }

    #[test]
    fn trims_input() {
        let url = "  https://github.com/org/repo  ";
        let ssh = convert_to_ssh(url).unwrap();
        assert_eq!(ssh, "git@github.com:org/repo.git");
    }
}
