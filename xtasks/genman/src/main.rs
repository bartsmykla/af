use af::Cli;
use clap::{Command, CommandFactory};
use clap_mangen::Man;
use std::{env, ffi::OsString, fs, path::PathBuf};

const PATH_MAN_DEFAULT: &str = "docs/man/man1";
const PATH_MAN_ENV_VAR: &str = "OUT_PATH_MAN";

fn main() -> anyhow::Result<()> {
    let root_cmd = Cli::command();

    let out_path_man: PathBuf = env::var_os(PATH_MAN_ENV_VAR)
        .unwrap_or_else(|| OsString::from(PATH_MAN_DEFAULT))
        .into();

    fs::create_dir_all(&out_path_man)?;

    generate_man_pages(&root_cmd, &out_path_man, root_cmd.get_name())?;

    Ok(())
}

fn generate_man_pages(cmd: &Command, out_dir: &PathBuf, prefix: &str) -> anyhow::Result<()> {
    let man = Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)?;

    fs::write(out_dir.join(format!("{}.1", prefix)), buffer)?;

    for sub in cmd.get_subcommands() {
        let sub_prefix = format!("{}-{}", prefix, sub.get_name());
        generate_man_pages(sub, out_dir, &sub_prefix)?;
    }

    Ok(())
}
