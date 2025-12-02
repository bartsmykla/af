use af::{Cli, utils};
use clap::Parser;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (level, logger) = utils::setup_logger(cli.log_level_filter());
    let multi = MultiProgress::new();

    LogWrapper::new(multi.clone(), logger).try_init()?;
    log::set_max_level(level);

    cli.run(multi).await
}
