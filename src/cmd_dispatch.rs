/// # Command Dispatch module
///
/// ## Author
/// Afnan Enayet
///
/// # Summary
///
/// The command dispatch module contains internal commands/utilities for the 
/// shell. This includes utilities like `cd`. These commands perform an action 
/// on a Shell object. It will determine whether a function is an internal shell 
/// function, or a binary that needs to be executed through another process.
///

use shell::Shell;
use parser::norm_abs_path;
use std::process::Command;
use std::str::from_utf8;

// Dispatches a command based on some sanitized input string (ex: "cd ~")
pub fn dispatch(shell: &mut Shell, cmd: &str) -> bool {
    // Tokenize command, splitting into words and spaces
    let tok_cmd = cmd.split(" ").collect::<Vec<_>>();

    // Execute internal function if necessary
    match tok_cmd[0] {
        "cd" => cd(tok_cmd[1], shell),
        _ => ex_bin(cmd, shell),
    }
}

// Executes a binary/program that is present in the shell's path
// Returns whether the operation was successful
fn ex_bin(cmd: &str, shell: &mut Shell) -> bool {
    // if binary is found in one of the shell's include directories, 
    // execute binary with argument, spawn process, etc
    // otherwise return false

    // Tokenize command, splitting by space
    let tok_cmd = cmd.split(" ").collect::<Vec<_>>();

    // look to see if binary exists. If it does, then execute command. Otherwise 
    // return false
    if shell.find_bin(tok_cmd[0]) {
        // TODO make this more modular, return output rather than printing directly 
        // from here
        let process = Command::new(tok_cmd[0])
            .args(&tok_cmd[1..])
            .output()
            .expect("Failed to execute process");

        let output_vec = process.stdout;
        let output_str = from_utf8(&output_vec).unwrap();
        print!("{}", output_str);
        println!("");
        true
    } else {
        false 
    }
}

// Changes the working directory of a Shell object to the path referenced by 
// the argument. Will return a boolean indicating whether the operation was 
// successful
fn cd(path: &str, shell: &mut Shell) -> bool {
    let abs_path = norm_abs_path(path);

    // If path has an issue then return false, don't try 
    if abs_path.is_err() {
        false
    } else {
        shell.change_working_dir(abs_path.unwrap().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for test functions that returns an initialized shell
    fn initialize_shell() -> Shell {
        Shell::default()
    }

    // Tests that shell's working directory can be changed to a valid path
    #[test]
    fn test_cwd_valid_path() {
    }

    // Test that the shell's working directory can't be changed to an invalid 
    // path
    #[test]
    fn test_cwd_invalid_path() {
    }
}
