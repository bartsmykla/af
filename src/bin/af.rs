use std::time::SystemTime;
use clap::{CommandFactory, Parser};
use fern::colors::{Color, ColoredLevelConfig};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use af::{Cli, Commands, GitCommands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let multi = MultiProgress::new();

    let colors = ColoredLevelConfig::new()
        // use builder methods
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::TrueColor {
            r: 117,
            g: 195,
            b: 170,
        });

    let (level, logger) = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{left_bracket}{timestamp} {level} {target}{right_bracket} {message}",
                left_bracket = format_args!(
                    "\x1B[{}m[\x1B[0m",
                    Color::TrueColor {
                        r: 107,
                        g: 107,
                        b: 107
                    }
                    .to_fg_str(),
                ),
                timestamp = humantime::format_rfc3339_seconds(SystemTime::now()),
                level = colors.color(record.level()),
                target = record.target(),
                right_bracket = format_args!(
                    "\x1B[{}m]\x1B[0m",
                    Color::TrueColor {
                        r: 107,
                        g: 107,
                        b: 107
                    }
                    .to_fg_str(),
                ),
                message = message
            ))
        })
        .level(if cli.debug {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Error
        })
        .level_for("af", cli.verbose.log_level_filter())
        .chain(std::io::stdout())
        .into_log();

    LogWrapper::new(multi.clone(), logger).try_init()?;
    log::set_max_level(level);
    
    match cli.command {
        Commands::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout())
        }

        Commands::ProjectGitClone(args) => args.run(&multi).await?,

        Commands::Git(git) => match git.command {
            Some(GitCommands::CloneProject(args)) => args.run(&multi).await?,
            None => {}
        },
    }

    Ok(())
}
