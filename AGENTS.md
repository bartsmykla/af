# Repository Guidelines

## Project Structure & Module Organization
- `src/bin/af.rs` is the Tokio async entrypoint; `src/af/lib.rs` wires Clap’s multicall CLI (`Cli`/`Applet`).
- Feature code lives under `src/af/cmd/` (git helpers, browser tools, dotfiles, shortcuts), with shared helpers in `src/af/utils.rs` and constants in `src/af/consts.rs`.
- Generated docs live in `docs/` (`af.md`) and man pages in `docs/man/`; regenerate via xtasks. Companion crates: `af-alfred-workflow/` (Alfred workflow) and `xtasks/gen{md,man}` for doc generation.

## Build, Test, and Development Commands
- `cargo build` / `cargo build --release` – compile the CLI.
- `cargo run -- <subcommand>` – run locally (use `--help` to explore subcommands like `git clone-project`).
- `cargo test --all --verbose` – run all tests (mirrors CI in `.github/workflows/test.yaml`).
- `cargo fmt` and `cargo clippy --all-targets --all-features` – format and lint; run before PRs.
- Docs: `cargo genmd` (markdown), `cargo genman` (man page), or `mise run gen` for everything; `mise run clean::gen` cleans outputs.

## Coding Style & Naming Conventions
- Rust edition 2024; default rustfmt settings (4‑space indent). Keep modules small and prefer `anyhow::Context` for errors.
- Logging via `fern`/`indicatif_log_bridge`; use `trace!/debug!` sparingly and respect `cli.log_level_filter()`.
- Naming: modules/files snake_case, constants SCREAMING_SNAKE, types UpperCamelCase, functions snake_case. Favor expressive subcommand names that mirror existing ones.

## Testing Guidelines
- Unit tests live next to code (example in `src/af/utils.rs`); add targeted cases for URL parsing and command-side effects.
- Use `cargo test -p af` when iterating on the main crate; include async tests only when necessary.
- When adding CLI flags, add regression tests where practical and update docs generation.

## Commit & Pull Request Guidelines
- Follow Conventional Commits used in history (`feat:`, `fix:`, `chore(deps): ...`, `refactor:`); releases are tagged `vX.Y.Z`.
- PRs should describe the change, linked issue (if any), tests run, and user-facing impacts. Include screenshots or sample CLI output for UX changes.
- Keep diffs focused; run format/lint/tests before opening. Update `docs/af.md`/`docs/man/` when CLI surface changes.

## Configuration & Environment Tips
- Key env vars: `PROJECTS_PATH` (clone root), `DOTFILES_PATH` (dotfiles dir), `HOMEBREW_PREFIX` (Homebrew location, defaults to `/opt/homebrew`).
- Avoid committing generated `target/` artifacts or local `tmp/` scratch files; honor `.gitignore`.
