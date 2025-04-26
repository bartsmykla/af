use anyhow::bail;
use clap::Subcommand;
use git2::Repository;
use log::debug;

const REMOTE_NAMES: [&str; 2] = ["upstream", "origin"];

#[derive(Debug, Subcommand)]
pub enum Abbreviation {
    /// Expands to: git checkout <default-branch> && git fetch <remote> && git merge --ff-only <remote>/<default-branch>
    #[command(name = "gcmff")]
    GitCheckoutMasterFetchFastForward,
}

impl Abbreviation {
    pub fn run(&self) {
        match self {
            Abbreviation::GitCheckoutMasterFetchFastForward => match Repository::open_from_env() {
                Ok(repo) => match get_remote_and_default_branch(&repo) {
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

/// Returns the first found remote (from REMOTE_NAMES) and its default branch name
fn get_remote_and_default_branch(repo: &Repository) -> anyhow::Result<(String, String)> {
    for remote_name in REMOTE_NAMES {
        if repo.find_remote(remote_name).is_ok() {
            let clean_pattern = format!("{remote_name}/");
            let revparse_spec = format!("{clean_pattern}HEAD");

            if let Ok((_, Some(reference))) = repo.revparse_ext(&revparse_spec) {
                if let Some(ref_short) = reference.shorthand() {
                    return Ok((
                        remote_name.to_string(),
                        ref_short.replace(&clean_pattern, ""),
                    ));
                }
            }
        }
    }

    // No matching remote and HEAD found
    bail!("Could not find remote and default branch")
}
