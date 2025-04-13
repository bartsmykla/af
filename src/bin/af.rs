use af::{Cli, Commands, DotCommands, cmd, consts::*, ides, utils};
use anyhow::anyhow;
use clap::{CommandFactory, Parser};
use fern::colors::{Color, ColoredLevelConfig};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use regex::Regex;
use std::env;
use std::time::SystemTime;

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

        Commands::Dot(dot) => match dot.command.unwrap_or(DotCommands::Ide(dot.ide)) {
            DotCommands::Ide(args) => {
                let re = Regex::new(r"application\.com\.jetbrains\.(\w+)(?:-.+)?(?:\.\d+)*")?;
                let xpc_service_name = env::var(XPC_SERVICE_NAME).unwrap_or_default();
                let ide = re
                    .captures(&xpc_service_name)
                    .map(|c| c.get(1).map_or("", |m| m.as_str()));

                let ides = ides::list();

                let index = ides
                    .binary_search(&ide.unwrap_or(ides::get(GO).unwrap()))
                    .map_err(|e| anyhow!("{:?}", e))?;

                if let Some(p) = args.path {
                    utils::run_command(ides[index], &[p.to_str().unwrap()])?;
                }
            }
        },

        Commands::Git { command } => match command {
            cmd::git::GitCommands::CloneProject(args) => args.run(&multi).await?,
        },

        // Aliases
        Commands::ProjectGitClone(args) => args.run(&multi).await?,
    }

    Ok(())
}
