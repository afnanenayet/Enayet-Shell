/// interface.rs    Afnan Enayet
///
/// The interface module handles most of the input/output for the shell
/// In general, it takes input from STDIN and displays output to STDOUT and
/// STDERR and abstracts away some of the details like formatting and shell
/// prompts

use std::io::{self, Write, BufRead};

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

    // Read line from stdin
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input).expect(
        "could not read from stdin",
        );
    input.trim().to_string() // strip the newline
}

// Prints shell prompt to STDOUT
pub fn print_shell_prompt(prompt: &str, wd_str: &str) {
    println!("({})", wd_str);
    print!("{} ", prompt);
    let r = io::stdout().flush();
    r.expect("failed to flush stdout");
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Tests if functions can properly print to STDOUT
    #[test]
    fn test_print_stdout() {
        print_out("test");
    }

    // Tests if function can properly print to STDERR
    #[test]
    fn test_print_stderr() {
        print_err("test");
    }

    // Tests if function can print shell prompt to STDOUT
    #[test]
    fn test_print_sh_prompt() {
        print_shell_prompt(">", "test");
    }

    // Tests if retrieving input from stdin can occur without panic
    // TODO see if there's any way to run a unit test taking stdin 
    // with cargo
    /*
       #[test]
       fn test_get_input() {
       get_input("test: ", "working dir");
       }
       */
}
