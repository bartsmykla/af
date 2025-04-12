use anyhow::{anyhow, Result};
use clap::{
    Args,
    ValueHint,
    value_parser,
};
use clio::ClioPath;
use dialoguer::{
    Confirm,
    Input,
    FuzzySelect,
    theme::ColorfulTheme,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{debug, info, trace};
use regex::Regex;
use std::{env, fs};
use std::time::Duration;
use console::style;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository, StatusOptions};
use git2::build::RepoBuilder;
use thiserror::Error;
use crate::{ides, utils};
use crate::repo::Repo;

#[derive(Debug, Args)]
#[command(about = "Clone Project")]
pub struct GitCloneProjectArgs {
    #[arg(
        value_parser = parse_repository,
    )]
    repository_url: Option<String>,

    #[arg(
        long,
        default_value_t = true,
        require_equals = true,
        help = "Should open the repository in matching IDE (if found and available)"
    )]
    open_ide: std::primitive::bool,

    #[arg(long, short)]
    force: bool,

    #[arg(
        long,
        env = "PROJECTS_PATH",
        required_unless_present = "directory",
        value_hint = ValueHint::DirPath,
        value_parser = value_parser!(ClioPath).exists().is_dir(),
    )]
    root_directory: Option<ClioPath>,

    #[arg(
        long,
        value_hint = ValueHint::DirPath
    )]
    directory: Option<ClioPath>,

    #[arg(long, default_value_t = true, require_equals = true)]
    rename_origin: std::primitive::bool,
}

impl GitCloneProjectArgs {
    pub async fn run(&self, multi_progress: &MultiProgress) -> Result<()> {
        trace!("Arguments: {:?}", self);

        let repository_url = match &self.repository_url {
            Some(url) => url.clone(),
            None => {
                let theme = &ColorfulTheme::default();

                let mut input = Input::with_theme(theme)
                    .with_prompt("Provide project's repository url you wish to clone")
                    .validate_with(|a: &String| validate_repository(a));

                let clipboard = cli_clipboard::get_contents()
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                if validate_repository(&clipboard).is_ok() {
                    input = input.default(clipboard);
                }

                input.interact()?
            }
        };

        let repo = Repo::parse(&repository_url)?;
        let directory = self
            .directory
            .clone()
            .or_else(|| {
                self.root_directory
                    .clone()
                    .map(|root| root.clone().join(repo.org).join(repo.name))
            })
            .ok_or_else(|| {
                anyhow!("At least one of --directory or --root-directory must be provided")
            })?;

        // Clone repository with progress
        let cloned_repo_maybe = clone_repository(
            multi_progress,
            &repo,
            &repository_url,
            &directory,
            self.force,
        )
        .await;

        if let Err(err) = &cloned_repo_maybe {
            if let Some(err) = err.downcast_ref::<CloneRepositoryError>() {
                return match err {
                    CloneRepositoryError::OperationCancelled => {
                        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt("Do you want to open the repository in IDE?")
                            .interact()?;

                        if !confirmed {
                            return Ok(());
                        }

                        self.open_ide_maybe(&repo, &directory).await
                    }
                };
            }
        }

        let cloned_repo = cloned_repo_maybe?;

        // Rename origin if required
        if self.rename_origin {
            cloned_repo.remote_rename("origin", "upstream")?;
        }

        // Open IDE if requested
        self.open_ide_maybe(&repo, &directory).await?;

        Ok(())
    }

    /// Opens the cloned project in an IDE if available.
    async fn open_ide_maybe(&self, repo: &Repo<'_>, directory: &ClioPath) -> Result<()> {
        if !self.open_ide {
            return Ok(());
        }

        let re = Regex::new(r"application\.com\.jetbrains\.(\w+)(?:-.+)?(?:\.\d+)*")?;
        let xpc_service_name = env::var("XPC_SERVICE_NAME").unwrap_or_default();

        let ide = match repo.find_ide().await? {
            Some(ide) => Some(ide),
            None => re
                .captures(&xpc_service_name)
                .map(|c| c.get(1).map_or("", |m| m.as_str())),
        };

        debug!("Detected IDE: {:?}", ide);

        let ides = ides::list();

        let index = ides
            .binary_search(&ide.unwrap_or(ides[0]))
            .map_err(|e| anyhow!("{:?}", e))?;

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an IDE to open the project, or press 'Esc' to skip")
            .default(index)
            .items(&ides)
            .interact_opt()?;

        if let Some(selected) = selection {
            utils::run_command(ides[selected], &[directory.to_str().unwrap()])?;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
enum CloneRepositoryError {
    #[error("Operation Cancelled")]
    OperationCancelled,
}

/// Clones a repository with real-time progress updates.
async fn clone_repository(
    mp: &MultiProgress,
    repo: &Repo<'_>,
    url: &str,
    directory: &ClioPath,
    force: bool,
) -> Result<Repository> {
    if directory.exists() && directory.read_dir()?.next().is_some() {
        if let Ok(r) = Repository::open(directory.to_path_buf()) {
            debug!("{} is a git repository", utils::format_directory(directory));

            let dirty = !r
                .statuses(Some(StatusOptions::new().include_untracked(true)))?
                .is_empty();

            if dirty {
                debug!(
                    "{} contains uncommitted changes",
                    utils::format_directory(directory)
                );

                let are_you_sure = format!(
                    "{}{}{}",
                    style("Are you ").yellow(),
                    style(" REALLY ").bold().red().reverse(),
                    style(" sure you want to continue and remove it?").yellow()
                );

                let msg = format!(
                    "{} exists and is a Git repository with uncommitted changes. {}",
                    style(utils::format_directory(directory)).bold(),
                    are_you_sure,
                );

                let confirmed = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(msg.as_str())
                    .interact()?;

                if !confirmed {
                    debug!("Aborting");
                    return Err(CloneRepositoryError::OperationCancelled.into());
                }
            } else if !force {
                let msg = format!(
                    "{} is a Git repository in a clean state. {}",
                    style(utils::format_directory(directory)).bold(),
                    style("Are you sure you want to continue and remove it?").yellow(),
                );

                let confirmed = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(msg.as_str())
                    .interact()?;

                if !confirmed {
                    debug!("Aborting");
                    return Err(CloneRepositoryError::OperationCancelled.into());
                }
            }
        }

        info!(
            "Removing existing directory: {}",
            style(utils::format_directory(directory)).bold(),
        );
        fs::remove_dir_all(directory.to_path_buf())?;
    }

    let pb = mp.add(ProgressBar::no_length().with_message("Cloning"));
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template(
            "{msg:.green.bold} {spinner}[{elapsed_precise}] {wide_bar} {percent:>3}%",
        )?
            .tick_strings(&["⢎  ", "⠎⠁ ", "⠊⠑ ", "⠈⠱ ", " ⡱ ", "⢀⡰ ", "⢄⡠ ", "⢆⡀ ", ""]),
    );

    let mut callbacks = RemoteCallbacks::new();

    callbacks
        .credentials(|_url, _username_from_url, _allowed| Cred::ssh_key_from_agent(repo.username));

    callbacks.transfer_progress(|progress| {
        if pb.length().is_none() {
            pb.set_length(progress.total_objects() as u64);
        }

        pb.set_position(progress.received_objects() as u64);

        true
    });

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_opts);

    info!(
        "Cloning {} into {}",
        style(repo.short_format()).bold(),
        style(utils::format_directory(directory)).bold(),
    );
    let cloned_repo = builder.clone(url, directory)?;

    pb.finish_and_clear();

    pb.println(format!(
        "Project {} was cloned to {}",
        style(repo.short_format()).bold(),
        style(utils::format_directory(directory)).bold(),
    ));

    mp.remove(&pb);

    Ok(cloned_repo)
}

fn parse_repository(repository: &str) -> Result<String> {
    let repository = repository.trim();
    let re = Regex::new(r"^git@(.+):(.+)/(.+)\.git$")?;

    re.captures(repository)
        .map(|_| repository.to_string())
        .ok_or_else(|| {
            anyhow!(
                "Unsupported repository URL. Supported format: {}",
                style("git@<host>:<org>/<repo>.git").bold()
            )
        })
}

fn validate_repository(repository: &str) -> Result<()> {
    parse_repository(repository).map(|_| ())
}