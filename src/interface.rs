/// interface.rs    Afnan Enayet
///
/// The interface module handles most of the input/output for the shell
/// In general, it takes input from STDIN and displays output to STDOUT and
/// STDERR and abstracts away some of the details like formatting and shell
/// prompts

use std::io::{stderr, stdin, stdout, BufRead, Write};
use consts::tokens::OP_CONTINUE;

// Prints some given output to stdout
pub fn print_out(output: &str) {
    let r = write!(&mut stdout(), "{}", output);
    if r.is_err() {
        print_err("failed to print to stdout");
    }
}

// Prints some given output to stderr
pub fn print_err(output: &str) {
    let r = write!(&mut stderr(), "{}", output);
    r.expect("failed to print to stderr");
}

// Retrieves a line (delimited by the '\n' character) from stdin, will also
// print the shell prompt that prepends the input space. Note that this
// operation can fail, leaving a None value
pub fn get_input(prompt: &str, working_dir: &str) -> String {
    print_shell_prompt(prompt, working_dir);
    let mut input = String::new();

    // loop over stdin while the last character is `/`
    loop {
        // Read line from stdin
        let stdin = stdin();
        stdin
            .lock()
            .read_line(&mut input)
            .expect("could not read from stdin");
        input.trim().to_string(); // strip the newline

        // TODO fix this part
        let last_char = input.pop().unwrap().to_string();
        println!("last char: {}", last_char);
        if last_char != OP_CONTINUE {
            input.push_str(last_char.as_str());
            break;
        }
    }
    input
}

// Prints shell prompt to STDOUT
pub fn print_shell_prompt(prompt: &str, wd_str: &str) {
    println!("({})", wd_str);
    print!("{} ", prompt);
    let r = stdout().flush();
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
}
