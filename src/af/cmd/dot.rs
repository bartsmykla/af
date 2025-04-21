use crate::consts::{GO, XPC_SERVICE_NAME};
use crate::{ides, utils};
use anyhow::{anyhow, Result};
use clap::{ValueHint, value_parser, Args, Subcommand};
use clio::ClioPath;
use regex::Regex;
use std::env;

#[derive(Debug, Args)]
#[command(visible_alias = ".")]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
#[command(disable_help_subcommand = true)]
pub struct Dot {
    /// Optional dotfiles subcommand
    #[command(subcommand)]
    pub command: Option<DotCommands>,

    /// IDE-related options (used when no subcommand is given)
    #[command(flatten)]
    pub ide: Ide,
}

impl Dot {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Some(DotCommands::Ide(args)) => args.run(),
            None => self.ide.run(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum DotCommands {
    /// Open the dotfiles directory in an IDE
    ///
    /// If inside a JetBrains IDE, it will use that IDE to open the path.
    /// Otherwise, it tries to open in GoLand.
    Ide(Ide),
}

#[derive(Debug, Args)]
pub struct Ide {
    /// Path to the dotfiles directory (overrides $DOTFILES_PATH)
    #[arg(
        long,
        env = "DOTFILES_PATH",
        value_hint = ValueHint::DirPath,
        value_parser = value_parser!(ClioPath).exists().is_dir(),
    )]
    pub path: Option<ClioPath>,
}

impl Ide {
    pub fn run(&self) -> Result<()> {
        let re = Regex::new(r"application\.com\.jetbrains\.(\w+)(?:-.+)?(?:\.\d+)*")?;
        let xpc_service_name = env::var(XPC_SERVICE_NAME).unwrap_or_default();
        let ide = re
            .captures(&xpc_service_name)
            .map(|c| c.get(1).map_or("", |m| m.as_str()));

        let ides = ides::list();

        let index = ides
            .binary_search(&ide.unwrap_or(ides::get(GO).unwrap()))
            .map_err(|e| anyhow!("{:?}", e))?;

        if let Some(p) = &self.path {
            utils::run_command(ides[index], &[p.to_str().unwrap()])?;
        }

        Ok(())
    }
}
