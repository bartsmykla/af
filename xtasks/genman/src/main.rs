use af::Cli;
use clap::CommandFactory;
use clap_mangen::Man;
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, fs};

const PATH_MAN_DEFAULT: &str = "/docs/man/man1";
const PATH_MAN_ENV_VAR: &str = "OUT_PATH_MAN";

fn main() -> anyhow::Result<()> {
    let cmd = Cli::command();

    let out_path_man: PathBuf = env::var_os(PATH_MAN_ENV_VAR)
        .unwrap_or(OsString::from(PATH_MAN_DEFAULT))
        .into();

    fs::create_dir_all(&out_path_man)?;
    
    // Prepare Man
    let man = Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    // Write results to files
    fs::write(out_path_man.join(format!("{}.1", cmd.get_name())), buffer)?;

    Ok(())
}
