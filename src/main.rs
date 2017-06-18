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

mod interface;
mod shell;


// Program wide constants
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Entry point for shell
// Initializees the shell, then loops program, waiting for commands
fn main() {
    // Initialize shell
    init_shell();

    while shell_loop() {}

    // Exit shell with return code
    shell_exit(0);
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

