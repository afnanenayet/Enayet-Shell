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
/// information. It's important to note that this module has side effects
/// and will affect the environment's working directory as a side effect
/// of changing the shell's working directory

use std::path::PathBuf;
use std::env;

use parser;

/// A shell and its associated information is associated here, including
/// the current working directory, the input history, and the paths that
/// the shell will search
#[derive(Debug)]
#[derive(Default)]
pub struct Shell {
    working_dir: PathBuf, // The current working directory
    input_history: Vec<String>, // The user's input history
    output_count: u64, // The number of lines outputted
    paths: Vec<String>, // The paths the shell will search for binaries
}

// Methods for shell
impl Shell {
    /// Default constructor for the shell. Will initialize with default
    /// values and return a Shell struct. Default initial working directory
    /// is `/`
    fn default() -> Shell {
        Shell {
            working_dir: PathBuf::from("/"),
            input_history: Vec::new(),
            output_count: 0,
            paths: Vec::new(),
        }
    }

    /// Change the shell's working directory. Will return an a boolean
    /// indicating whether the working directory was successfully changed
    /// or not
    pub fn change_working_dir(&mut self, wd: &str) -> bool {
        let path_obj = PathBuf::from(wd);

        // Only change wd if it exists
        if env::set_current_dir(&path_obj).is_ok() {
            self.working_dir = path_obj;
            true
        } else {
            false
        }
    }

    /// Returns a string representation of the shell's current working directory
    pub fn get_cwd(&self) -> &str {
        self.working_dir.to_str().unwrap()
    }


    /// Searches for and detects whether the binary exists, searching the paths
    /// that were loaded from the config file
    pub fn find_bin(&self, bin_name: &str) -> bool {
        // TODO multithread the lookup
        let mut bin_found = false;
        let bin_name = bin_name.trim();

        // Don't accept blank names
        if bin_name.is_empty() {
            return false;
        }

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

    /// Set include paths from a config file. Pass in a string with the path to
    /// the config file. Assumes the string is valid
    pub fn load_paths(&mut self, config_path: Option<&str>, default_paths: &Vec<String>) {
        self.paths = parser::config::load_paths_from_config(config_path, &default_paths);
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use super::parser::config::create_default_config;

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
        let mut tmp_dir: PathBuf = env::temp_dir();
        tmp_dir.push("config_r");
        let mut shell = Shell::default();
        // vector with paths we want to write
        let def_paths_vec = create_default_path_vec();
        let fp_str = tmp_dir.as_path().to_str().unwrap();

        parser::config::create_default_config(&fp_str, &def_paths_vec);
        shell.load_paths(Some(&fp_str), &def_paths_vec);
        assert!(shell.paths.len() > 0);
    }

    // Tests if the shell can look for a file, in this case a binary,
    // searching the paths set for the shell. Assumes cat is available
    // in one of the paths
    #[test]
    fn test_search_bin() {
        let mut shell = Shell::default();
        let def_paths_vec = create_default_path_vec();
        shell.paths = def_paths_vec;
        assert!(!shell.find_bin(""));
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

    // Tests that the working directory of the shell is properly output as a string
    #[test]
    fn test_print_wd() {
        let mut shell = Shell::default();
        shell.change_working_dir("/");
        assert_eq!(shell.get_cwd(), "/");
    }
}
