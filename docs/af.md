# Command-Line Help for `af`

This document contains the help content for the `af` command-line program.

**Command Overview:**

- [`af`↴](#af)
- [`af completions`↴](#af-completions)
- [`af git`↴](#af-git)
- [`af git clone-project`↴](#af-git-clone-project)
- [`af pgc`↴](#af-pgc)

## `af`

The afrael CLI tool

**Usage:** `af [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `completions` — Generate shell completions
- `git` — Collection of helper subcommands for git
- `pgc` — Alias to `af git clone-project`

###### **Options:**

- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity
- `--debug`

## `af completions`

Generate shell completions

**Usage:** `af completions <SHELL>`

###### **Arguments:**

- `<SHELL>` — The shell to generate the completions for

  Possible values: `bash`, `elvish`, `fish`, `nushell`, `powershell`, `zsh`

## `af git`

Collection of helper subcommands for git

**Usage:** `af git [COMMAND]`

###### **Subcommands:**

- `clone-project` — Clone Project

## `af git clone-project`

Clone Project

**Usage:** `af git clone-project [OPTIONS] [REPOSITORY_URL]`

###### **Arguments:**

- `<REPOSITORY_URL>`

###### **Options:**

- `--open-ide <OPEN_IDE>` — Should open the repository in matching IDE (if found and available)

  Default value: `true`

  Possible values: `true`, `false`

- `-f`, `--force`

- `--root-directory <ROOT_DIRECTORY>`

- `--directory <DIRECTORY>`

- `--rename-origin <RENAME_ORIGIN>`

  Default value: `true`

  Possible values: `true`, `false`

## `af pgc`

Alias to `af git clone-project`

**Usage:** `af pgc [OPTIONS] [REPOSITORY_URL]`

###### **Arguments:**

- `<REPOSITORY_URL>`

###### **Options:**

- `--open-ide <OPEN_IDE>` — Should open the repository in matching IDE (if found and available)

  Default value: `true`

  Possible values: `true`, `false`

- `-f`, `--force`

- `--root-directory <ROOT_DIRECTORY>`

- `--directory <DIRECTORY>`

- `--rename-origin <RENAME_ORIGIN>`

  Default value: `true`

  Possible values: `true`, `false`
