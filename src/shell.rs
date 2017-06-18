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

// The struct that holds shell information
#[derive(Debug)]
#[derive(Default)]
pub struct shell {
    // TODO 
    working_dir: String, // The current working directory 
    input_history: Vec<String>, // The user's input history
    output_count: u64, // The number of lines outputted
}

// Methods for shell
impl shell {
    // TODO: complete this class
    // Do these variables need to be explicitly initialized?

    // Change the shell's working directory. Will return an option which will 
    // indicate if there was an error. Supply a string with the filepath of 
    // the working directory
    fn change_working_dir(&self, wd: String) {
    }

    // Returns the number of commands that have already been inputted to the 
    // shell
    fn input_count(&self) -> usize {
        self.input_history.len()
    }
}

