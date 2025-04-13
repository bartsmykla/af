# Command-Line Help for `af`

This document contains the help content for the `af` command-line program.

**Command Overview:**

- [`af`↴](#af)
- [`af completions`↴](#af-completions)
- [`af dot`↴](#af-dot)
- [`af dot ide`↴](#af-dot-ide)
- [`af git`↴](#af-git)
- [`af git clone-project`↴](#af-git-clone-project)
- [`af pgc`↴](#af-pgc)

## `af`

The afrael CLI tool

**Usage:** `af [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `completions` — Generate shell completion scripts
- `dot` — Helper commands related to dotfiles (defaults to `dot ide` if no subcommand is used)
- `git` — Git-related helper commands
- `pgc` — Shortcut for `af git clone-project`

###### **Options:**

- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity
- `--debug` — Enable debug output

## `af completions`

Generate shell completion scripts

**Usage:** `af completions <SHELL>`

###### **Arguments:**

- `<SHELL>` — Target shell to generate completions for

  Possible values: `bash`, `elvish`, `fish`, `nushell`, `powershell`, `zsh`

## `af dot`

Helper commands related to dotfiles (defaults to `dot ide` if no subcommand is used)

**Usage:** `af dot [OPTIONS]        dot ide [OPTIONS]`

###### **Subcommands:**

- `ide` — Open the dotfiles directory in an IDE

###### **Options:**

- `--path <PATH>` — Path to the dotfiles directory (overrides \$DOTFILES_PATH)

## `af dot ide`

Open the dotfiles directory in an IDE

If inside a JetBrains IDE, it will use that IDE to open the path. Otherwise, it tries to open in GoLand.

**Usage:** `af dot ide [OPTIONS]`

###### **Options:**

- `--path <PATH>` — Path to the dotfiles directory (overrides \$DOTFILES_PATH)

## `af git`

Git-related helper commands

**Usage:** `af git <COMMAND>`

###### **Subcommands:**

- `clone-project` — Clone a project repository and optionally open it in an IDE

## `af git clone-project`

Clone a project repository and optionally open it in an IDE

**Usage:** `af git clone-project [OPTIONS] [REPOSITORY_URL]`

###### **Arguments:**

- `<REPOSITORY_URL>` — The repository URL to clone (e.g. git@github.com:org/project.git)

###### **Options:**

- `--open-ide <OPEN_IDE>` — Open the cloned repository in a matching IDE if one is available

  Default value: `true`

  Possible values: `true`, `false`

- `-f`, `--force` — Force re-cloning even if the destination exists

- `--root-directory <ROOT_DIRECTORY>` — Root directory for placing the cloned project (uses \$PROJECTS_PATH if set)

- `--directory <DIRECTORY>` — Exact directory path to clone into (overrides root-directory)

- `--rename-origin <RENAME_ORIGIN>` — Rename remote “origin” to “upstream” after cloning

  Default value: `true`

  Possible values: `true`, `false`

## `af pgc`

Shortcut for `af git clone-project`

**Usage:** `af pgc [OPTIONS] [REPOSITORY_URL]`

###### **Arguments:**

- `<REPOSITORY_URL>` — The repository URL to clone (e.g. git@github.com:org/project.git)

###### **Options:**

- `--open-ide <OPEN_IDE>` — Open the cloned repository in a matching IDE if one is available

  Default value: `true`

  Possible values: `true`, `false`

- `-f`, `--force` — Force re-cloning even if the destination exists

- `--root-directory <ROOT_DIRECTORY>` — Root directory for placing the cloned project (uses \$PROJECTS_PATH if set)

- `--directory <DIRECTORY>` — Exact directory path to clone into (overrides root-directory)

- `--rename-origin <RENAME_ORIGIN>` — Rename remote “origin” to “upstream” after cloning

  Default value: `true`

  Possible values: `true`, `false`
