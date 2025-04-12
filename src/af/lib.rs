use clap::{Args, Parser, Subcommand};

pub mod repo;
pub mod ides;
pub mod cmd;
pub mod utils;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "af")]
#[command(version, about = "The afrael CLI tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },

    Git(GitArgs),

    #[command(name = "pgc", about = "Alias to `af git clone-project`")]
    ProjectGitClone(cmd::git::clone_project::GitCloneProjectArgs),
}

#[derive(Debug, Args)]
#[command(about = "Collection of helper subcommands for git", long_about = None)]
pub struct GitArgs {
    #[command(subcommand)]
    pub command: Option<GitCommands>,
}

#[derive(Debug, Subcommand)]
pub enum GitCommands {
    #[command(visible_alias = "cp")]
    CloneProject(cmd::git::clone_project::GitCloneProjectArgs),
}
