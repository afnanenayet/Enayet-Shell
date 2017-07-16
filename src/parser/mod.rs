/// mod.rs for parser
///
/// Contains helper functions to help with parsing user input
///
/// Mostly contains functions which help parse shell grammar and resolve 
/// pathnames
///

use std::io;
use std::env::home_dir;
use std::path::PathBuf;

pub mod config;
// Normalizes and expands path to its full absolute path, for use with 
// Rust's filesystem code
//
// Ex: ~/example -> /Users/user/example
pub fn norm_abs_path(path: &str) -> Result<String, io::Error> {
    let home_pb = home_dir().unwrap();
    let home_str: &str = home_pb.to_str().unwrap();
    let path_str = path.replace("~", home_str);
    let path = PathBuf::from(path_str).canonicalize().unwrap();
    Ok(path.as_os_str().to_str().unwrap().to_string())
}

// Condenses a path so that an absolute path is condensed and normalized 
// to a path relative to the home directory
pub fn condense_path(path: &str) -> Result<String, io::Error> {
    let home = home_dir().unwrap();
    let home_str = home.to_str().unwrap();
    let path_str = path.replace(home_str, "~");
    let result = PathBuf::from(path_str).canonicalize().unwrap();
    Ok(result.as_os_str().to_str().unwrap().to_string())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Tests that the norm_abs_path can convert from a relative path to an 
    // absolute path, given a valid path as an input
    #[test]
    fn test_norm_abs_path_valid_path() {
        let path = "~";
        let abs_path = norm_abs_path(path).unwrap();
        let correct_abs_path = "/Users/aenayet";
        assert_eq!(correct_abs_path, abs_path);
    }

    // Tests the norm_abs path with an path
    #[test]
    fn test_norm_abs_path_invalid_path() {
        let path = "invalid";
        let abs_path = norm_abs_path(path);

        // Should throw an error with an invalid path
        match abs_path {
            Ok(_) => panic!("Test failed"),
            Err(e) => println!("{}", e),
        }
    }
}

