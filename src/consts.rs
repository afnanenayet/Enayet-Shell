/// consts.rs
///
/// # Author
/// Afnan Enayet
///
/// # Summary
/// The `consts.rs` file contains global/static constants to be used
/// throughout the entire program. This file shall not contain any functions
/// or anything that is not a static or constant declaration of a variable
///

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

// immutable language tokens

/// Token delineating pipe operation
pub const OP_PIPE: &'static str = "|";

/// Token delineating file redirection
pub const OP_REDIR: &'static str = ">";

/// A hash set of all valid shell language tokens
pub const OP_TOKENS: &'static [&'static str] = &[OP_PIPE, OP_REDIR];
