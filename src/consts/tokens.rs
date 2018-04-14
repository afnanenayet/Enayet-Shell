/// consts/tokens.rs
///
/// The consts/tokens package contains immutable constants that are part of the
/// `ensh` scripting language.

/// Token delineating pipe operation
pub const OP_PIPE: &'static str = "|";

/// Token delineating file redirection
pub const OP_REDIR: &'static str = ">";

/// Token delineating command separation
pub const OP_SEP: &'static str = ";";

/// Token delineating command continuation after newline
pub const OP_CONTINUE: &'static str = "\\";

/// An array of all valid shell language tokens
pub const OP_TOKENS: &'static [&'static str] = &[
    OP_PIPE,
    OP_REDIR,
    OP_SEP,
    OP_CONTINUE,
];
