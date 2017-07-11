/// lib.rs    Afnan Enayet
/// # Synopsis
/// The lib.rs file contains the main logic for the program. It assumes the 
/// comand line parameters that are passed in are valid (they should be verified 
/// in `main.rs`).
/// This also contains the arguments struct and its implementation. This allows
/// for the required arguments to be changed without much refactoring

use std::error::Error;

mod interface;
mod parser;
mod shell;

// Program wide constants
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// A structure that holds the arguments being passed in
pub struct Args {
    config_file_path: Option<String>,
}

impl Args {
    pub fn new(args: Option<String>) -> Args {
        // Return struct with extracted arguments
        Args {
            config_file_path: args,
        }
    }
}

// Main entry point for program 
pub fn run(args: Args) {
    init_shell();
    while shell_loop() {}
    shell_exit(0);
}

// Initialize shell, using config file provided from arguments (if any)
// If no config file was given, search for default config file path. If 
// default does not exist, create a default config file with default 
// paths
fn init_shell() -> Result<(), Box<Error>> {
    println!("Enayet Shell | v{}", VERSION);
    
    // TODO initialize shell, load options from config file
    Ok(())
}

// Captures input from stdin and executes commands from 
fn shell_loop() -> bool {
    let exit_code = "exit".to_string();

    // TODO move these to Shell struct
    let shell_prompt = ">";
    let working_dir = "~";

    // Get command from user
    let input = interface::get_input(shell_prompt, working_dir);

    // Exit if necessary 
    if input != exit_code {
        println!("cmd: {}", input); // TODO replace with command dispatch
        true
    } else {
        false
    }
}

// Cleans up and exits the shell with the specified exit code
fn shell_exit(exit_status: i32) {
    std::process::exit(exit_status);
}

