use clap::Subcommand;

pub mod clone_project;

#[derive(Debug, Subcommand)]
pub enum GitCommands {
    #[command(visible_alias = "cp")]
    CloneProject(clone_project::GitCloneProjectArgs),
}
