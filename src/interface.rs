/// interface.rs    Afnan Enayet
///
/// The interface module handles most of the input/output for the shell
/// In general, it takes input from STDIN and displays output to STDOUT and
/// STDERR and abstracts away some of the details like formatting and shell
/// prompts

use std::io::{stderr, stdin, stdout, BufRead, Write};
use consts::tokens::OP_CONTINUE;

/// Prints some given output to stdout
pub fn print_out(output: &str) {
    let r = write!(&mut stdout(), "{}", output);
    if r.is_err() {
        print_err("failed to print to stdout");
    }
}

/// Prints some given output to stderr
pub fn print_err(output: &str) {
    let r = write!(&mut stderr(), "{}", output);
    r.expect("failed to print to stderr");
}

/// Retrieves a line (delimited by the '\n' character) from stdin, will also
/// print the shell prompt that prepends the input space. Note that this
/// operation can fail, leaving a None value
pub fn get_input(prompt: &str, working_dir: &str) -> String {
    print_shell_prompt(prompt, working_dir);
    let mut buf = String::new();
    let mut input = String::new();
    let stdin = stdin();

    // loop over stdin while the last character is OP_TOKEN after stripping
    // trailing whitespace
    loop {
        // Read a line from stdin
        stdin
            .lock()
            .read_line(&mut buf)
            .expect("could not read from stdin");
        buf = buf.trim().to_string(); // strip the newline

        // if we should continue input, take the next line, otherwise remove
        // the last continuation token
        if should_cont_input(buf.as_ref()) {
            buf.pop();
            input += buf.as_str();
            buf = String::new();
        } else {
            input += buf.as_str();
            break;
        }
    }
    input
}

/// Prints shell prompt to STDOUT
pub fn print_shell_prompt(prompt: &str, wd_str: &str) {
    let out_str = format!("({})\n{} ", wd_str, prompt);
    print_out(out_str.as_str());
    let r = stdout().flush();
    r.expect("failed to flush stdout");
}

/// Given some input string (from `stdin`), computes whether input should
/// continue or terminate.
///
/// This function uses the `OP_CONTINUE` token to determine whether input
/// should be consumed or the shell should continue to read more lines. If
/// a string ends with `OP_CONTINUE`, this token will be removed and the
/// next line will be attached to the input string as if it was a continuation
/// of the previous line.
fn should_cont_input(curr_input: &str) -> bool {
    let last_char = curr_input.chars().last().unwrap_or_default();
    last_char == OP_CONTINUE
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

    #[test]
    fn test_should_cont_input() {
        // TODO
        let test_string = "Hello, world!";
        assert!(!should_cont_input(test_string));

        let test_string = "hello, world\\";
        assert!(should_cont_input(test_string));
    }
}
