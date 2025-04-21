use anyhow::Result;
use clap::Subcommand;
use indicatif::MultiProgress;

pub mod clone_project;

#[derive(Debug, Subcommand)]
pub enum Git {
    #[command(visible_alias = "cp")]
    CloneProject(clone_project::CloneProject),
}

impl Git {
    pub async fn run(&self, multi: &MultiProgress) -> Result<()> {
        match self {
            Git::CloneProject(args) => args.run(multi).await,
        }
    }
}
