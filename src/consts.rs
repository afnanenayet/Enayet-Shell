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

// Default paths to be used for the program when no configuration file has been 
// specified (used to find binaries)
const DEFAULT_PATHS: &'static [&'static str] = &[
    "/usr/bin",
    "/usr/local/bin",
    "/usr/local/sbin",
    "/usr/sbin",
    "/bin",
]

