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
/// for the required arguments to be changed without much refactoring.
///

mod consts;
mod interface;
mod parser;
mod shell;
mod cmd_dispatch;

use shell::Shell;
use consts::*;
use parser::expand_path;
use interface::print_out;

// Program wide constants
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// A structure that holds the arguments being passed in from the command
/// line
pub struct Args {
    config_file_path: Option<String>,
}

impl Args {
    /// Creates a new `Args` struct with all of the command line arguments
    /// represented as rust objects in the struct
    pub fn new(args: Option<String>) -> Args {
        // Return struct with extracted arguments
        Args { config_file_path: args }
    }
}

/// Main entry point for program
/// Initializes shell, dispatches shell loop, then calls exit function when
/// appropriate
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

/// Initialize shell using config file provided from arguments (if any)
/// If no config file was given, search for default config file path. If
/// default does not exist, create a default config file with default
/// paths
fn init_shell(config_fp: Option<&str>) -> Shell {
    let initial_prompt = format!("Enayet Shell | v{}\n", VERSION);
    print_out(initial_prompt.as_str());

    let normalized_fp = match config_fp {
        Some(s) => expand_path(s),
        None => expand_path("~/.ensh_config"),
    };

    // Initialize shell and load config options from file
    let mut shell = Shell::default();
    let mut def_path_vec: Vec<String> = Vec::new();

    for path in DEFAULT_PATHS {
        def_path_vec.push(path.to_string());
    }

    // Load PATH(S) into shell
    shell.load_paths(Some(normalized_fp.as_str()), &def_path_vec);

    // Set working directory to home or "/" if it fails
    if !cmd_dispatch::dispatch(&mut shell, "cd ~") {
        cmd_dispatch::dispatch(&mut shell, "cd /");
    }
    shell
}

/// Captures input from stdin and executes commands from input
/// displays output to shell as necessary. Returns if shell should
/// be terminated or continue for another loop iteration
fn shell_loop(shell: &mut Shell) -> bool {
    let exit_code = "exit".to_string();
    let working_dir = shell.get_cwd().to_owned();

    // Get command from user
    let input = interface::get_input(SHELL_PROMPT, &working_dir[..]);

    // Exit if necessary
    if input != exit_code {
        if !cmd_dispatch::dispatch(shell, &input[..]) {
            println!(":(\n");
        } else {
            println!("");
        }
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
    use std::env::temp_dir;

    // Test that shell can be initialized properly
    #[test]
    fn test_init_shell() {
        // A test vector of config paths
        let config_vec = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ];

        // Create filepath/PathBuf for temporary config file
        let mut temp_pb = temp_dir();
        temp_pb.push("lib_test");
        let temp_pb = temp_pb.as_path();
        let config_fp: &str = temp_pb.to_str().unwrap();

        // Create test config file
        super::parser::config::create_default_config(config_fp, &config_vec);

        // Initialize shell with string pointing to temp file just created
        let shell = init_shell(Some(config_fp));
    }
}
