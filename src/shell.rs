/// shell.rs    
///
/// # Author
/// Afnan Enayet
///
/// # Summary
/// The ensh module provides a structure and implementation of a shell object.
/// It contains state variables that the shell needs to keep track of, such as: 
/// * the current working directory
/// * command history
/// * the number of lines that have been input and output
/// * the PATH directories
///
/// This object is used to represent a shell session and hold shell state 
/// information

use std::path::PathBuf;
use std::fs::File;
use std::io;

use parser;

// The default configuration path

// The struct that holds shell information
#[derive(Debug)]
#[derive(Default)]
pub struct Shell {
    // TODO 
    working_dir: PathBuf, // The current working directory 
    input_history: Vec<String>, // The user's input history
    output_count: u64, // The number of lines outputted
    paths: Vec<String>, // The paths the shell will search for binaries
}

// Methods for shell
impl Shell {
    // Default constructor for the shell. Will initialize with default 
    // values and return a Shell struct
    fn default() -> Shell {
        Shell {
            working_dir : PathBuf::from("~"),
            input_history : Vec::new(),
            output_count : 0,
            paths : Vec::new(),
        }
    }

    // Change the shell's working directory. Will return an a boolean 
    // indicating whether the working directory was successfully changed
    // or not
    pub fn change_working_dir(&mut self, wd: &str) -> bool {
        let path_obj = PathBuf::from(wd);

        // Only change wd if it exists
        if path_obj.exists() {
            self.working_dir = path_obj;
            true
        } else {
            false
        }
    }

    // Returns the number of commands that have already been inputted to the 
    // shell
    pub fn input_count(&self) -> u64 {
        self.input_history.len() as u64
    }

    // Conveys raw input/command to the shell
    pub fn cmd(&mut self, input: &str) {
        let path_obj = PathBuf::from(input);
    }

    // Detects whether the binary exists, searching the paths that were loaded
    // from the config file
    pub fn find_bin(&self, bin_name: &str) -> bool {
        let mut bin_found = false;

        // Searching every path in the paths vector for the binary
        for path in self.paths.clone() {
            let full_bin_path_str = path + "/" + bin_name;
            bin_found = PathBuf::from(full_bin_path_str).exists();

            if bin_found {
                return true;
            }
        }
        bin_found
    }

    // Set include paths for shell using path strings
    pub fn set_paths(&mut self, paths: Vec<String>) {
        self.paths = paths;
    }

    // Set include paths from a config file. Pass in a string with the path to 
    // the config file
    pub fn load_paths(&mut self, config_path: Option<&str>, 
                  default_paths: &Vec<String>) {
        self.paths = parser::config::load_paths_from_config(config_path, 
                                                            &default_paths); 
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Returns a vector with some sample default paths for the purposes of 
    // testing
    fn create_default_path_vec() -> Vec<String> {
        vec![
            "/usr/bin".to_string(),
            "/usr/local/bin".to_string(),
            "/bin/".to_string(),
        ]
    }

    // Check if shell initializes properly
    #[test]
    fn test_default_shell_init() {
        let shell = Shell::default();
    }

    // Tests if shell can load any paths from the config file
    #[test]
    fn test_load_paths() {
        let mut shell = Shell::default();
        let def_paths_vec = create_default_path_vec();
        shell.load_paths(Some("unit_test_files/config_r"), &def_paths_vec);
        assert!(shell.paths.len() > 0);
    }

    // Tests if the shell can look for a file, in this case a binary, 
    // searching the paths set for the shell. Assumes top is available 
    // in one of the paths
    #[test]
    fn test_search_bin() {
        let mut shell = Shell::default();
        let def_paths_vec = create_default_path_vec();
        shell.paths = def_paths_vec;
        assert!(shell.find_bin("cat"));
    }

    // tests to see if the shell can properly change working directories
    #[test]
    fn test_cwd() {
        // Shell test setup
        let mut shell = Shell::default();
        let def_paths_vec = create_default_path_vec();
        shell.paths = def_paths_vec;
        assert!(shell.change_working_dir("/"));
    }
}

