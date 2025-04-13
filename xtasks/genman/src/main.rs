use af::Cli;
use clap::{Command, CommandFactory};
use clap_mangen::Man;
use std::{env, fs, path::{Path, PathBuf}};

const PATH_MAN_DEFAULT: &str = "docs/man/man1";
const PATH_MAN_ENV_VAR: &str = "OUT_PATH_MAN";

fn main() -> anyhow::Result<()> {
    let root_cmd = Cli::command();
    let out_path = env::var_os(PATH_MAN_ENV_VAR)
        .map(PathBuf::from)
        .unwrap_or_else(|| PATH_MAN_DEFAULT.into());

    fs::create_dir_all(&out_path)?;

    generate_man_pages(&root_cmd, &out_path, vec![root_cmd.get_name()])?;
    
    Ok(())
}

fn generate_man_pages(cmd: &Command, out_dir: &Path, prefixes: Vec<&str>) -> anyhow::Result<()> {
    let man_cmd = cmd.clone().name(prefixes.join(" "));
    let mut buffer = Vec::new();
    Man::new(man_cmd.clone()).render(&mut buffer)?;
    
    fs::write(out_dir.join(format!("{}.1", prefixes.join("-"))), buffer)?;

    for sub in man_cmd.get_subcommands() {
        let mut new_prefixes = prefixes.clone();
        new_prefixes.push(sub.get_name());
        generate_man_pages(sub, out_dir, new_prefixes)?;
    }

    Ok(())
}
