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
    // TODO: complete this class
    // Do these variables need to be explicitly initialized?

    // Change the shell's working directory. Will return an option which will 
    // indicate if there was an error. Supply a string with the filepath of 
    // the working directory
    fn change_working_dir(&mut self, wd: &String) {
        self.working_dir = PathBuf::from(wd);
    }

    // Returns the number of commands that have already been inputted to the 
    // shell
    fn input_count(&self) -> u64 {
        self.input_history.len() as u64
    }

    // Set include paths for shell using path strings
    fn set_paths(&mut self, paths: Vec<String>) {
        self.paths = paths;
    }

    // Set include paths from a config file. Pass in a string with the path to 
    // the config file
    fn load_paths(&mut self, config_path: Option<String>) {
        let paths = parser::config::load_paths_from_config(Some("test")); 
    }
}

