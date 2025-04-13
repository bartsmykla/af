use clap::value_parser;
use clap::{Args, ValueHint};
use clap::{Parser, Subcommand};
use clio::ClioPath;
use consts::*;

pub mod cmd;
pub mod consts;
pub mod ides;
pub mod repo;
pub mod utils;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = AF)]
#[command(version, about = "The afrael CLI tool", long_about = None)]
pub struct Cli {
    /// Top-level command to run
    #[command(subcommand)]
    pub command: Commands,

    /// Increase output verbosity (-v, -vv, -vvv, etc.)
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Enable debug output
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate shell completion scripts
    Completions {
        /// Target shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },

    /// Helper commands related to dotfiles (defaults to `dot ide` if no subcommand is used)
    Dot(DotArgs),

    /// Git-related helper commands
    #[command(visible_alias = "g")]
    Git {
        #[command(subcommand)]
        command: cmd::git::GitCommands,
    },

    /// Shortcut for `af git clone-project`
    #[command(name = "pgc")]
    ProjectGitClone(cmd::git::clone_project::GitCloneProjectArgs),
}

#[derive(Debug, Args)]
#[command(visible_alias = ".")]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
#[command(disable_help_subcommand = true)]
pub struct DotArgs {
    /// Optional dotfiles subcommand
    #[command(subcommand)]
    pub command: Option<DotCommands>,

    /// IDE-related options (used when no subcommand is given)
    #[command(flatten)]
    pub ide: DotCommandsIdeArgs,
}

#[derive(Debug, Subcommand)]
pub enum DotCommands {
    /// Open the dotfiles directory in an IDE
    ///
    /// If inside a JetBrains IDE, it will use that IDE to open the path.
    /// Otherwise, it tries to open in GoLand.
    Ide(DotCommandsIdeArgs),
}

#[derive(Debug, Args)]
pub struct DotCommandsIdeArgs {
    /// Path to the dotfiles directory (overrides $DOTFILES_PATH)
    #[arg(
        long,
        env = "DOTFILES_PATH",
        value_hint = ValueHint::DirPath,
        value_parser = value_parser!(ClioPath).exists().is_dir(),
    )]
    pub path: Option<ClioPath>,
}
