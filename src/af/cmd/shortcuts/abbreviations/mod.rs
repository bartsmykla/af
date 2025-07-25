use crate::consts::HEAD;
use crate::consts::{
    CHECKOUT, DIFF, FETCH, FF_ONLY, FORCE, FORCE_WITH_LEASE, GIT, MERGE, NO_VERIFY, ORIGIN_SLICE,
    ORIGIN_UPSTREAM_SLICE, PBCOPY, PUSH, UPSTREAM_ORIGIN_SLICE, UPSTREAM_SLICE,
};
use anyhow::bail;
use clap::{Subcommand, ValueEnum};
use git2::Repository;
use log::debug;
use std::fmt::Debug;
use std::{iter, slice, vec};

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
            GitPushRemote::Upstream => UPSTREAM_SLICE.iter().copied(),
            GitPushRemote::UpstreamFirst => UPSTREAM_ORIGIN_SLICE.iter().copied(),
            GitPushRemote::Origin => ORIGIN_SLICE.iter().copied(),
            GitPushRemote::OriginFirst => ORIGIN_UPSTREAM_SLICE.iter().copied(),
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

    /// Expands to: git diff <reference> -- <files> [optionally copy to clipboard]
    #[command(name = "gd")]
    GitDiff {
        /// Specific files to diff (if omitted, diffs all changes)
        #[arg(long, short)]
        files: Option<Vec<String>>,

        /// Git reference to diff against (e.g. HEAD)
        #[arg(long, short)]
        reference: Option<String>,

        /// Copy the diff output to clipboard using pbcopy
        #[arg(long, short)]
        pbcopy: bool,
    },
}

impl Abbreviation {
    pub fn run(&self) {
        match self {
            Abbreviation::GitCheckoutMasterFetchFastForward => match Repository::open_from_env() {
                Ok(repo) => match get_remote_and_default_branch(&repo, UPSTREAM_ORIGIN_SLICE) {
                    Ok((remote, default_branch)) => {
                        let head = repo
                            .head()
                            .ok()
                            .and_then(|h| h.shorthand().map(str::to_string))
                            .unwrap_or_default();

                        if head != default_branch {
                            print!("{GIT} {CHECKOUT} {default_branch} && ");
                        }

                        print!(
                            "{GIT} {FETCH} {remote} && {GIT} {MERGE} {FF_ONLY} {remote}/{default_branch}"
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
                                let mut cmd = vec![GIT, PUSH, &remote, branch];

                                if *no_verify {
                                    cmd.push(NO_VERIFY);
                                }

                                if *force_with_lease {
                                    cmd.push(FORCE_WITH_LEASE);
                                } else if *force {
                                    cmd.push(FORCE);
                                }

                                return print!("{}", cmd.join(" "));
                            }

                            debug!("Branch name could not be determined");
                        }
                        Ok(_) => debug!("{HEAD} is not pointing to a branch"),
                        Err(err) => debug!("Failed to get {HEAD}: {err:#}"),
                    },
                    Err(err) => debug!("Failed to get remote and default branch: {err:#}"),
                },
                Err(err) => debug!("Failed to open repository from environment: {err:#}"),
            },
            Abbreviation::GitDiff {
                files,
                reference,
                pbcopy,
            } => {
                if Repository::open_from_env().is_err() {
                    debug!("Failed to open repository from environment");
                    return;
                }

                let mut cmd = vec![GIT, DIFF];
                
                if let Some(ref_name) = reference {
                    cmd.push(ref_name);
                }
                
                if let Some(files) = files {
                    cmd.push("--");
                    cmd.extend(files.iter().map(String::as_str));
                }

                if *pbcopy {
                    cmd.extend(vec!["|", PBCOPY]);
                }

                print!("{}", cmd.join(" "))
            }
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
                let revparse_spec = format!("{clean_pattern}{HEAD}");

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
