use af::Cli;
use clap::CommandFactory;
use clap_markdown::{help_markdown_command_custom, MarkdownOptions};
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, fs};

const PATH_DOCS_DEFAULT: &str = "docs";
const PATH_MD_ENV_VAR: &str = "OUT_PATH_MD";

fn main() -> anyhow::Result<()> {
    let cmd = Cli::command();

    // Prepare paths
    let out_path_md: PathBuf = env::var_os(PATH_MD_ENV_VAR)
        .unwrap_or(OsString::from(PATH_DOCS_DEFAULT))
        .into();

    fs::create_dir_all(&out_path_md)?;

    // Prepare Markdown
    let options = MarkdownOptions::new().show_footer(false);
    let markdown = help_markdown_command_custom(&cmd, &options);

    // Write results to files
    fs::write(out_path_md.join(format!("{}.md", cmd.get_name())), markdown)?;

    Ok(())
}
