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
use std::path::PathBuf;

// Dispatches a command based on some sanitized input string (ex: "cd ~")
pub fn dispatch(shell: &mut Shell, command: &str) -> bool {
    // Tokenize command, splitting into words and spaces
    let tok_cmd = command.split(" ").collect::<Vec<_>>();

    // Execute internal function if necessary
    match tok_cmd[0] {
        "cd" => cd(tok_cmd[1], shell),
        _ => ex_bin(command),
    }
}

// Executes a binary/program that is present in the shell's path
// Returns whether the operation was successful
pub fn ex_bin(bin_name: &str) -> bool {
    // TODO
    // if binary is found in one of the shell's include directories, 
    // execute binary with argument, spawn process, etc
    // otherwise return false
    false // TODO change
}

// Changes the working directory of a Shell object to the path referenced by 
// the argument. Will return a boolean indicating whether the operation was 
// successful
pub fn cd(path: &str, shell: &mut Shell) -> bool {
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
