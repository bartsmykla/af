use af::{Cli, consts::AF};
use clap::{Command, CommandFactory};
use clap_mangen::Man;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

const PATH_MAN_DEFAULT: &str = "docs/man/man1";
const PATH_MAN_ENV_VAR: &str = "OUT_PATH_MAN";

fn main() -> anyhow::Result<()> {
    let out_path = env::var_os(PATH_MAN_ENV_VAR)
        .map(PathBuf::from)
        .unwrap_or_else(|| PATH_MAN_DEFAULT.into());

    fs::create_dir_all(&out_path)?;

    let cmd = Cli::command()
        .find_subcommand(AF)
        .map(ToOwned::to_owned)
        .unwrap_or_else(Cli::command);

    generate_man_pages(&cmd, &out_path, vec![cmd.get_name()])?;

    Ok(())
}

fn generate_man_pages(
    root_cmd: &Command,
    out_dir: &Path,
    prefixes: Vec<&str>,
) -> Result<PathBuf, io::Error> {
    let name = prefixes.join("-");
    let cmd = root_cmd.clone().name(&name).bin_name(prefixes.join(" "));

    for subcmd in cmd.get_subcommands() {
        generate_man_pages(
            subcmd,
            out_dir,
            [prefixes.as_slice(), &[subcmd.get_name()]].concat(),
        )?;
    }

    Man::new(cmd)
        .title(name.to_uppercase())
        .generate_to(out_dir)
}
