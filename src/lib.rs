/// lib.rs    
/// 
/// # Author
/// Afnan Enayet
///
/// # Synopsis
/// The lib.rs file contains the main logic for the program. It assumes the 
/// comand line parameters that are passed in are valid (they should be verified 
/// in `main.rs`).
/// This also contains the arguments struct and its implementation. This allows
/// for the required arguments to be changed without much refactoring
///

mod consts;
mod interface;
mod parser;
mod shell;
mod cmd_dispatch;

use shell::Shell;
use consts::*;
use parser::norm_abs_path;
use cmd_dispatch::dispatch;

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
    // Convert from Option<String> to Option<&str>
    let config_fp: Option<&str> = match args.config_file_path.as_ref() {
        Some(s) => Some(s),
        None => None,
    };

    // Initialize shell and load config
    let mut shell = init_shell(config_fp);

    // Run everything that needs to run while the shell is operating
    while shell_loop(&mut shell) {}
    shell_exit(0);
}

// Initialize shell, using config file provided from arguments (if any)
// If no config file was given, search for default config file path. If 
// default does not exist, create a default config file with default 
// paths
fn init_shell(config_fp: Option<&str>) -> Shell {
    println!("Enayet Shell | v{}", VERSION);

    let normalized_fp = match config_fp {
        Some(s) => norm_abs_path(s).unwrap(),
        None => norm_abs_path("~/.ensh_config").unwrap(),
    };
    
    // Initialize shell and load config options from file
    let mut shell = Shell::default();
    let mut def_path_vec: Vec<String> = Vec::new(); 
    
    for path in DEFAULT_PATHS {
        def_path_vec.push(path.to_string());
    }

    shell.load_paths(config_fp, &def_path_vec);
    shell
}

// Captures input from stdin and executes commands from 
fn shell_loop(shell: &mut Shell) -> bool {
    let exit_code = "exit".to_string();
    let working_dir = shell.get_cwd().to_owned();

    // Get command from user
    let input = interface::get_input(SHELL_PROMPT, &working_dir[..]);

    // Exit if necessary 
    if input != exit_code {
        // TODO update with actual output as necessary
        if cmd_dispatch::dispatch(shell, &input[..]) {
            println!("Command was successful");
        } else {
            println!("Command failed");
        }
        println!(""); // print blank line
        true
    } else {
        false
    }
}

// Cleans up and exits the shell with the specified exit code
fn shell_exit(exit_status: i32) {
    std::process::exit(exit_status);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test that shell can be initialized properly
    #[test]
    fn test_init_shell() {
        let shell = init_shell(None);
    }
}
