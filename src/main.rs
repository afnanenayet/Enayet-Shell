/// The Enayet Shell is a shell written in rust as a personal project
/// Afnan Enayet, 2017
///
/// # Configuration
/// The main file serves as the entry point to the shell. The shell will loook 
/// at a config file, `~/.ensh_config`
///
/// # Arguments
/// * `config` - the configuration file to be loaded with this shell session. If 
/// no argument is supplied, the shell will look for the default config file, 
/// which is defined above. If it cannot find that configuration file, it will 
/// create a config file with the defaults. The config file contains include paths.
/// The shell will search these paths for executables.
///
/// # Return value
/// * 0 - no errors
/// * 1 - error reading config file
///

use std::io;

// Program wide constants
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Entry point for shell
// Initializees the shell, then loops program, waiting for commands
fn main() {
    init_shell();
}

// Initialize shell, using config file provided from arguments (if any)
// If no config file was given, search for default config file path. If 
// default does not exist, create a default config file with default 
// paths
fn init_shell() {
    println!("Enayet Shell, v{}", VERSION);
}
