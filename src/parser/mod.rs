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
// Rust's filesystem code. If path is invalid, inaccessible, or 
// nonexistent, the function will throw an error
//
// Ex: ~/example -> /Users/user/example
pub fn norm_abs_path(path: &str) -> Result<String, io::Error> {
    let home_pb = home_dir().unwrap();
    let home_str: &str = home_pb.to_str().unwrap();
    let path_str = path.replace("~", home_str);
    let path = PathBuf::from(path_str).canonicalize();

    // If path exists
    match path {
        Ok(path) => Ok(path.as_os_str().to_str().unwrap().to_string()),
        Err(e) => Err(e),
    } 
}

// Condenses a path so that an absolute path is condensed and normalized 
// to a path relative to the home directory
pub fn condense_path(path: &str) -> Result<String, io::Error> {
    let home = get_home_str().unwrap();
    let home_str = home.as_str();

    let path_str = path.replace(home_str, "~");
    let result = PathBuf::from(path_str).canonicalize();
    
    // Check if path exists, if so, return string representation
    match result {
        Ok(path) => Ok(path.as_os_str().to_str().unwrap().to_string()),
        Err(e) => Err(e),
    }
}

// Gets the string representation for the path to the home directory. In Unix, 
// this is generally the HOME variable 
fn get_home_str() -> Option<String> {
    let home = home_dir();

    // If home variable is found, convert path to string. Otherwise return 
    // an error
    match home {
        Some(home) => Some(home.to_str().unwrap().to_string()),
        None => None,
    } 
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
        let abs_path = norm_abs_path(path).unwrap(); // this should not panic
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

    // Tests norm_abs_path with a path that is already expanded and does not 
    // need to be expanded at all
    #[test]
    fn test_norm_abs_path_with_abs_path() {
        let path = "/";
        let abs_path = norm_abs_path(path);
        assert_eq!(path, abs_path.unwrap());
    }

    // Tests if path can be condensed with a valid path name
    #[test]
    fn test_condense_path_valid_path() {
        let path = "/Users/aenayet/";
        let correct_condensed_path = "~";
        let condensed_path = condense_path(path);
        assert_eq!(correct_condensed_path, condensed_path.unwrap());
    }

    // Tests if path can be condensed with an invalid path name
    #[test]
    fn test_condense_path_invalid_path() {
        let path = "nonexistent";
        let condensed_path = condense_path(path);

        match condensed_path {
            Ok(_) => panic!("Test failed"),
            Err(e) => println!("{}", e),
        }
    }

    // Tests if path condense works properly with a valid path that doesn't 
    // need to be condensed
    #[test]
    fn test_condense_path_already_condensed() {
        let path = "~";
        let condensed_path = condense_path(path).unwrap();
        assert_eq!(path, condensed_path);
    }

    // Tests if function can properly retrieve the HOME path string
    #[test]
    fn test_get_home_path_str() {
        let home_str = get_home_str();
        assert!(home_str.is_some());
    }
}

