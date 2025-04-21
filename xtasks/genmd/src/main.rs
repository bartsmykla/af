use af::{Cli, consts::AF};
use clap::CommandFactory;
use clap_markdown::{MarkdownOptions, help_markdown_command_custom};
use std::{env, ffi::OsString, fs, path::PathBuf};

const PATH_DOCS_DEFAULT: &str = "docs";
const PATH_MD_ENV_VAR: &str = "OUT_PATH_MD";

fn main() -> anyhow::Result<()> {
    let out_path_md: PathBuf = env::var_os(PATH_MD_ENV_VAR)
        .unwrap_or(OsString::from(PATH_DOCS_DEFAULT))
        .into();

    fs::create_dir_all(&out_path_md)?;

    let cmd = Cli::command()
        .find_subcommand(AF)
        .map(ToOwned::to_owned)
        .unwrap_or_else(Cli::command);

    let path = out_path_md.join(format!("{}.md", cmd.get_name()));
    let options = MarkdownOptions::new().show_footer(false);
    let markdown = help_markdown_command_custom(&cmd, &options);

    fs::write(path, markdown)?;

    Ok(())
}
