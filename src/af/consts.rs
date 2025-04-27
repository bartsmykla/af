use fern::colors::Color;

// Common
pub const AF: &str = "af";

// Env vars
pub const XPC_SERVICE_NAME: &str = "XPC_SERVICE_NAME";
pub const HOME: &str = "HOME";

// Languages
pub const C: &str = "c";
pub const CPP: &str = "c++";
pub const GO: &str = "go";
pub const JS: &str = "javascript";
pub const RUBY: &str = "ruby";
pub const RUST: &str = "rust";

// IDEs
pub const CLION: &str = "clion";
pub const GOLAND: &str = "goland";
pub const RUBYMINE: &str = "rubymine";
pub const RUSTROVER: &str = "rustrover";
pub const WEBSTORM: &str = "webstorm";

// Commands
pub const WHICH: &str = "which";
pub const GIT: &str = "git";
pub const PBCOPY: &str = "pbcopy";

// Git specific
pub const HEAD: &str = "HEAD";
pub const ORIGIN: &str = "origin";
pub const UPSTREAM: &str = "upstream";
pub const ORIGIN_SLICE: &[&str] = &[ORIGIN];
pub const UPSTREAM_SLICE: &[&str] = &[UPSTREAM];
pub const UPSTREAM_ORIGIN_SLICE: &[&str] = &[UPSTREAM, ORIGIN];
pub const ORIGIN_UPSTREAM_SLICE: &[&str] = &[ORIGIN, UPSTREAM];
pub const PUSH: &str = "push";
pub const FETCH: &str = "fetch";
pub const MERGE: &str = "merge";
pub const CHECKOUT: &str = "checkout";
pub const DIFF: &str = "diff";
pub const NO_VERIFY: &str = "--no-verify";
pub const FORCE_WITH_LEASE: &str = "--force-with-lease";
pub const FF_ONLY: &str = "--ff-only";
pub const FORCE: &str = "--force";

// Colors
pub const GREY: Color = Color::TrueColor {
    r: 107,
    g: 107,
    b: 107,
};
pub const MUTED_TEAL: Color = Color::TrueColor {
    r: 117,
    g: 195,
    b: 170,
};