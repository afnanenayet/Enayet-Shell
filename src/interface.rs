/// interface.rs    Afnan Enayet
///
/// The interface module handles most of the input/output for the shell
/// In general, it takes input from STDIN and displays output to STDOUT and 
/// STDERR and abstracts away some of the details like formatting and shell 
/// prompts

use std::io::{self, Write};

// Prints some given output to stdout 
pub fn print_out(output: &str) {
    println!("{}", output);
}

// Prints some given output to stderr
pub fn print_err(output: &str) {
    let r = writeln!(&mut io::stderr(), "{}", output);
    r.expect("failed to print to stderr");
}

// Retrieves a line (delimited by the '\n' character) from stdin, will also 
// print the shell prompt that prepends the input space. Note that this 
// operation can fail, leaving a None value
pub fn get_input(prompt: &str, working_dir: &str) -> String {
    print_shell_prompt(prompt, working_dir);
    let input = stdin.lock().lines().next().unwrap();
    input.expect("unable to retrieve input from stdin");
    Ok(input)
}

// Prints shell prompt to STDOUT
pub fn print_shell_prompt(prompt: &str, wd_str: &str) {
    println!("({})", wd_str);
    print!("{} ", prompt);
    let r = io::stdout().flush();
    r.expect("failed to flush stdout");
}

