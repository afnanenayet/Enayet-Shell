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

use std::io::{self, BufRead, Write};

// Program wide constants
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Entry point for shell
// Initializees the shell, then loops program, waiting for commands
fn main() {
    // Initialize shell
    init_shell();
    
    let mut valid_cmd = true;
    
    // Loop until user wants to exit
    while valid_cmd {
        valid_cmd = shell_loop();
    }
    
    // Returns an exit code to the shell
    let return_code = shell_exit();
    std::process::exit(return_code);
}

// Initialize shell, using config file provided from arguments (if any)
// If no config file was given, search for default config file path. If 
// default does not exist, create a default config file with default 
// paths
fn init_shell() {
    println!("Enayet Shell | v{}", VERSION);
}

// Captures input from stdin and executes commands from 
fn shell_loop() -> bool {
    let exit_code = "exit";
    let shell_prompt = ">";

    print_shell_prompt(shell_prompt);
    // Read input from stdin
    let stdin = io::stdin();

    // "elegant syntax"
    // TODO unsafe, change this
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Exit if necessary 
    if input != exit_code {
        println!("cmd: {}", input);
        return true;
    } else {
        return false;
    }
}

// Cleans up and exits the shell
fn shell_exit() -> i32 {
    println!("Thank you for using the Enayet shell");
    return 0;
}

// Prints shell prompt to STDOUT
fn print_shell_prompt(prompt: &str) {
    println!("(working directory)");
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
}

