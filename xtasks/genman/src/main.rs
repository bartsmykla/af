use af::Cli;
use clap::CommandFactory;
use clap_mangen::Man;
use clap_markdown::{help_markdown_command_custom, MarkdownOptions};
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, fs};
use const_format::concatcp;

const PATH_DOCS_DEFAULT: &str = "docs";
const PATH_MAN_DEFAULT: &str = concatcp!(PATH_DOCS_DEFAULT, "/man/man1");
const PATH_MAN_ENV_VAR: &str = "OUT_PATH_MAN";
const PATH_MD_ENV_VAR: &str = "OUT_PATH_MD";

fn main() -> std::io::Result<()> {
    let cmd = Cli::command();

    // Prepare paths
    let out_path_md: PathBuf = env::var_os(PATH_MD_ENV_VAR)
        .unwrap_or(OsString::from(PATH_DOCS_DEFAULT))
        .into();
    
    let out_path_man: PathBuf = env::var_os(PATH_MAN_ENV_VAR)
        .unwrap_or(OsString::from(PATH_MAN_DEFAULT))
        .into();

    fs::create_dir_all(&out_path_md)?;
    fs::create_dir_all(&out_path_man)?;

    // Prepare Markdown
    let options = MarkdownOptions::new().show_footer(false);
    let markdown = help_markdown_command_custom(&cmd, &options);

    // Prepare Man
    let man = Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    // Write results to files
    fs::write(out_path_md.join(format!("{}.md", cmd.get_name())), markdown)?;
    fs::write(out_path_man.join(format!("{}.1", cmd.get_name())), buffer)?;

    Ok(())
}
