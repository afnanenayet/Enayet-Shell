/// consts/tokens.rs
///
/// The consts/tokens package contains immutable constants that are part of the
/// `ensh` scripting language.

/// Token delineating pipe operation
pub const OP_PIPE: char = '|';

/// Token delineating file redirection
pub const OP_REDIR: char = '>';

/// Token delineating command separation
pub const OP_SEP: char = ';';

/// Token delineating command continuation after newline
pub const OP_CONTINUE: char = '\\';

/// An array of all valid shell language tokens
pub const OP_TOKENS: &'static [char] = &[OP_PIPE, OP_REDIR, OP_SEP, OP_CONTINUE];
