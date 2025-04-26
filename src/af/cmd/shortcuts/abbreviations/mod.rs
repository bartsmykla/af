use anyhow::bail;
use clap::{Subcommand, ValueEnum};
use git2::Repository;
use log::debug;
use std::fmt::Debug;
use std::{iter, slice, vec};

const ORIGIN: &[&str] = &["origin"];
const UPSTREAM: &[&str] = &["upstream"];
const UPSTREAM_ORIGIN: &[&str] = &["upstream", "origin"];
const ORIGIN_UPSTREAM: &[&str] = &["origin", "upstream"];

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum GitPushRemote {
    #[default]
    Origin,
    OriginFirst,
    Upstream,
    UpstreamFirst,
}

impl IntoIterator for GitPushRemote {
    type Item = &'static str;
    type IntoIter = iter::Copied<slice::Iter<'static, &'static str>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            GitPushRemote::Upstream => UPSTREAM.iter().copied(),
            GitPushRemote::UpstreamFirst => UPSTREAM_ORIGIN.iter().copied(),
            GitPushRemote::Origin => ORIGIN.iter().copied(),
            GitPushRemote::OriginFirst => ORIGIN_UPSTREAM.iter().copied(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Abbreviation {
    /// Expands to: git checkout <default-branch> && git fetch <remote> && git merge --ff-only <remote>/<default-branch>
    #[command(name = "gcmff")]
    GitCheckoutMasterFetchFastForward,

    /// Expands to: git push <remote> <branch> [optional flags]
    #[command(name = "gp")]
    GitPush {
        /// Remote priority strategy when pushing (e.g., upstream first, origin first)
        #[arg(long = "remote", short, value_enum, default_value_t)]
        remote_priority: GitPushRemote,

        /// Push with --no-verify flag (skip pre-push hooks)
        #[arg(long, short)]
        no_verify: bool,

        /// Push with --force-with-lease flag (safe force push)
        #[arg(long, short)]
        force_with_lease: bool,

        /// Push with --force flag (unsafe force push, overrides remote)
        #[arg(long, short = 'F', conflicts_with = "force_with_lease")]
        force: bool,
    },
}

impl Abbreviation {
    pub fn run(&self) {
        match self {
            Abbreviation::GitCheckoutMasterFetchFastForward => match Repository::open_from_env() {
                Ok(repo) => match get_remote_and_default_branch(&repo, UPSTREAM_ORIGIN) {
                    Ok((remote, default_branch)) => {
                        let head = repo
                            .head()
                            .ok()
                            .and_then(|h| h.shorthand().map(str::to_string))
                            .unwrap_or_default();

                        if head != default_branch {
                            print!("git checkout {default_branch} && ");
                        }

                        print!(
                            "git fetch {remote} && git merge --ff-only {remote}/{default_branch}"
                        );
                    }
                    Err(err) => debug!("Failed to get remote and default branch: {err:#}"),
                },
                Err(err) => debug!("Failed to open repository from environment: {err:#}"),
            },
            Abbreviation::GitPush {
                remote_priority,
                no_verify,
                force_with_lease,
                force,
            } => match Repository::open_from_env() {
                Ok(repo) => match get_remote_and_default_branch(&repo, *remote_priority) {
                    Ok((remote, _)) => match repo.head() {
                        Ok(head) if head.is_branch() => {
                            if let Some(branch) = head.shorthand() {
                                let mut cmd = vec!["git", "push", &remote, branch];

                                if *no_verify {
                                    cmd.push("--no-verify");
                                }

                                if *force_with_lease {
                                    cmd.push("--force-with-lease");
                                } else if *force {
                                    cmd.push("--force");
                                }

                                return print!("{}", cmd.join(" "));
                            }

                            debug!("Branch name could not be determined");
                        }
                        Ok(_) => debug!("HEAD is not pointing to a branch"),
                        Err(err) => debug!("Failed to get HEAD: {err:#}"),
                    },
                    Err(err) => debug!("Failed to get remote and default branch: {err:#}"),
                },
                Err(err) => debug!("Failed to open repository from environment: {err:#}"),
            },
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Shortcut {
    /// Group of abbreviation subcommands (alias: a, abbr, abbreviation)
    #[command(visible_aliases = ["a", "abbr", "abbreviation"])]
    #[command(subcommand)]
    Abbreviations(Abbreviation),
}

impl Shortcut {
    pub fn run(&self) {
        match self {
            // Delegates to the selected abbreviation command
            Shortcut::Abbreviations(cmd) => cmd.run(),
        }
    }
}

/// Returns the first found remote (from provided list) and its default branch name
fn get_remote_and_default_branch<I, S>(
    repo: &Repository,
    remote_names: I,
) -> anyhow::Result<(String, String)>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str> + Debug,
{
    for remote_name in remote_names {
        let remote_name = remote_name.as_ref();
        debug!("Trying remote: {remote_name:?}");

        match repo.find_remote(remote_name) {
            Ok(_) => {
                let clean_pattern = format!("{remote_name}/");
                let revparse_spec = format!("{clean_pattern}HEAD");

                match repo.revparse_ext(&revparse_spec) {
                    Ok((_, Some(reference))) => {
                        if let Some(ref_short) = reference.shorthand() {
                            return Ok((
                                remote_name.to_string(),
                                ref_short.replace(&clean_pattern, ""),
                            ));
                        }

                        debug!("Reference shorthand is None for remote {remote_name:?}");
                    }
                    Ok((_, None)) => debug!("No reference found for spec {revparse_spec}"),
                    Err(err) => debug!("Failed to rev-parse {revparse_spec}: {err:#}"),
                }
            }
            Err(err) => debug!("Remote {remote_name:?} not found: {err:#}"),
        }
    }

    bail!("Could not find remote and default branch")
}
