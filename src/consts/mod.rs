/// consts/mod.rs
///
/// # Author
/// Afnan Enayet
///
/// # Summary
/// The `consts.rs` file contains global/static constants to be used
/// throughout the entire program. This file shall not contain any functions
/// or anything that is not a static or constant declaration of a variable
///

pub mod tokens;

/// Default paths to be used for the program when no configuration file has been
/// specified (used to find binaries)
pub const DEFAULT_PATHS: &'static [&'static str] = &[
    "/usr/bin",
    "/usr/local/bin",
    "/usr/local/sbin",
    "/usr/sbin",
    "/bin",
];

/// The default shell prompt character
pub const SHELL_PROMPT: &'static str = "❯";

/// Default file path for config file
pub const DEF_CONFIG_FNAME: &'static str = ".ensh_config";
