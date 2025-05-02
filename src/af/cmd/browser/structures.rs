use crate::consts::{BREW, FLAG_PREFIX, FLAG_VERSION};
use Kind::*;

use anyhow::{anyhow, bail};
use clap::ValueEnum;
use glob::glob;
use log::{debug, trace};
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fmt,
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Kind {
    Brave,
    Chrome,
    Edge,
    Firefox,
    Opera,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

struct BrowserPath(Kind, PathBuf);

impl From<(Kind, PathBuf)> for BrowserPath {
    fn from(value: (Kind, PathBuf)) -> Self {
        let (kind, path) = value;
        Self(kind, path)
    }
}

impl Kind {
    pub fn all() -> Vec<Kind> {
        Self::value_variants().to_vec()
    }

    fn with_path(path: PathBuf) -> Option<BrowserPath> {
        path.file_name()
            .and_then(OsStr::to_str)
            .and_then(Self::from_bin)
            .map(|kind| BrowserPath(kind, path.to_owned()))
    }

    fn bin(&self) -> &str {
        match self {
            Chrome => "Google Chrome",
            Edge => "Microsoft Edge",
            Brave => "Brave Browser",
            Opera => "Opera",
            Firefox => "firefox",
        }
    }

    fn from_bin(bin: &str) -> Option<Kind> {
        Kind::value_variants()
            .iter()
            .cloned()
            .find(|&k| k.bin() == bin)
    }

    fn app(&self) -> &str {
        match self {
            Chrome => "Google Chrome.app",
            Edge => "Microsoft Edge.app",
            Brave => "Brave Browser.app",
            Opera => "Opera.app",
            Firefox => "Firefox.app",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Chrome => "Google Chrome",
            Edge => "Microsoft Edge",
            Brave => "Brave Browser",
            Opera => "Opera",
            Firefox => "Mozilla Firefox",
        }
    }

    fn short_name(&self) -> &str {
        match self {
            Chrome => "Chrome",
            Edge => "Edge",
            Brave => "Brave",
            Opera => "Opera",
            Firefox => "Firefox",
        }
    }

    fn applications_path(&self) -> PathBuf {
        Path::new("/Applications")
            .join(self.app())
            .join("Contents/MacOS")
            .join(self.bin())
    }

    fn parse_version(&self, version: &str) -> anyhow::Result<(String, String)> {
        let version = version.trim();
        let name = self.name().to_string();

        if self == &Opera {
            return Ok((name, version.to_string()));
        }

        version
            .strip_prefix(&name)
            .map(str::trim)
            .map(|v| (name, v.to_string()))
            .ok_or(anyhow!("invalid version: {version}"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Browser {
    name: String,
    kind: Kind,
    path: PathBuf,
    version: String,
}

impl Display for Browser {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<name_max$} {:<version_max$} {}",
            self.name,
            self.version,
            self.path.display(),
            name_max = Kind::value_variants()
                .iter()
                .map(Kind::name)
                .map(str::len)
                .max()
                .unwrap_or_default(),
            version_max = 13,
        )
    }
}

impl Browser {
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn version(&self) -> String {
        format!("{} {}", self.name, self.version)
    }
}

/// Entry point to find all known browser executables
pub fn find_browsers(extra_dirs: &[PathBuf], kinds: &[Kind], all: bool) -> Vec<Browser> {
    let mut all_kind_paths = vec![];

    // 1. Hardcoded .app locations
    all_kind_paths.extend(find_in_known_locations(kinds));

    // 2. Homebrew
    if let Some(homebrew_prefix) = detect_homebrew_prefix() {
        all_kind_paths.extend(find_in_homebrew(&homebrew_prefix, kinds));
    }

    // 3. Extra user-provided dirs
    all_kind_paths.extend(find_in_custom_dirs(extra_dirs, kinds));

    // Deduplicate by <binary> --version
    let mut seen_unique = BTreeMap::new();
    let mut seen_all = BTreeMap::new();

    for BrowserPath(kind, path) in all_kind_paths {
        match get_browser(&kind, &path) {
            Ok(browser) => {
                // insert returns None if the key was not already present
                if seen_unique
                    .insert(browser.version(), browser.clone())
                    .is_none()
                {
                    debug!("Found browser: {:#?}", browser);
                }

                seen_all.insert(format!("{browser}"), browser.clone());
            }
            Err(err) => debug!("{} {FLAG_VERSION} error: {err}", path.display()),
        }
    }

    let result = match all {
        true => seen_all,
        false => seen_unique,
    };

    result.values().cloned().collect()
}

pub fn find_browser(extra_dirs: &[PathBuf], kind: Kind) -> anyhow::Result<Browser> {
    match find_browsers(extra_dirs, &[kind], false).first() {
        Some(b) => Ok(b.clone()),
        None => bail!("No browser found for {}", kind),
    }
}

/// Helper to extract a normalized version string from `--version` output
fn get_browser(kind: &Kind, path: &Path) -> anyhow::Result<Browser> {
    Command::new(path)
        .arg(FLAG_VERSION)
        .output()
        .map_err(|e| anyhow!(e))
        .and_then(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            trace!(
                "{} --version: (stdout: {}) (stderr: {})",
                path.display(),
                stdout.trim(),
                stderr.trim(),
            );

            if output.status.success() {
                let (name, version) = kind.parse_version(stdout.trim())?;
                return Ok(Browser {
                    name,
                    version,
                    kind: *kind,
                    path: path.to_path_buf(),
                });
            }

            bail!("non-zero exit code: {}", output.status.code().unwrap_or(-1));
        })
}

/// Look in `/Applications/...` for known browser `.app` bundles
fn find_in_known_locations(kinds: &[Kind]) -> Vec<BrowserPath> {
    kinds
        .par_iter()
        .cloned()
        .filter(|kind| kind.applications_path().exists())
        .map(|kind| (kind, kind.applications_path()).into())
        .collect()
}

/// Use `brew --prefix` to detect Homebrew installation
fn detect_homebrew_prefix() -> Option<PathBuf> {
    Command::new(BREW)
        .arg(FLAG_PREFIX)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| PathBuf::from(s.trim()))
}

/// Look under `$(brew --prefix)/bin` and `Caskroom` for known browser executables
fn find_in_homebrew(prefix: &Path, kinds: &[Kind]) -> Vec<BrowserPath> {
    let bin_dir = prefix.join("bin");
    let caskroom_dir = prefix.join("Caskroom");

    let bin_matches = find_executables_in_dir(&bin_dir);
    let cask_matches = scan_caskroom(&caskroom_dir, kinds);

    bin_matches.into_iter().chain(cask_matches).collect()
}

/// Search Caskroom for any known browsers
fn scan_caskroom(caskroom: &Path, kinds: &[Kind]) -> Vec<BrowserPath> {
    if !caskroom.is_dir() {
        return Vec::new();
    }

    fs::read_dir(caskroom)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let entry_name = entry.file_name().to_string_lossy().to_lowercase();

            kinds.iter().cloned().find_map(|kind| {
                if !entry_name.contains(kind.bin()) {
                    return None;
                }

                let pattern = entry
                    .path()
                    .join("latest")
                    .join("*.app/Contents/MacOS")
                    .join(kind.bin());

                glob(pattern.to_string_lossy().as_ref())
                    .ok()?
                    .flatten()
                    .next()
                    .map(|p| (kind, p).into())
            })
        })
        .collect()
}

/// Search user-provided directories for `.app/Contents/MacOS/<bin>`
fn find_in_custom_dirs(dirs: &[PathBuf], kinds: &[Kind]) -> Vec<BrowserPath> {
    dirs.par_iter()
        .flat_map(|base| {
            fs::read_dir(base)
                .into_iter()
                .flatten()
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    let is_app = path.extension().and_then(|e| e.to_str()) == Some("app");

                    if !is_app {
                        return None;
                    }

                    for kind in kinds {
                        let exec = path.join("Contents/MacOS").join(kind.bin());
                        if exec.exists() {
                            return Some(BrowserPath(*kind, exec));
                        }
                    }

                    None
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Find exact executable names in a flat directory
fn find_executables_in_dir(dir: &Path) -> Vec<BrowserPath> {
    if !dir.is_dir() {
        return vec![];
    }

    fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter_map(Kind::with_path)
        .collect()
}
