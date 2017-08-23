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

extern crate ensh;

use std::env;
use ensh::{Args, run};

// Entry point for shell
// Initializees the shell, then loops program, waiting for commands
fn main() {
    // Command line args
    let mut args: Vec<String> = env::args().collect();

    // Checking to see if argument was given, if not:
    // make the arg_str a None
    let arg_str: Option<String> = match args.len() {
        1 => None,
        _ => Some(args.remove(1)),
    };
    let arg_struct: Args = Args::new(arg_str);

    // Run program
    run(arg_struct);
}
