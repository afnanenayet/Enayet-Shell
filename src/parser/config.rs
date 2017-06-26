/// config.rs    Afnan Enayet
///
/// The config module provides helper/convenience functions to parse 
/// the configuration file for the shell 

use std::io;
use std::fs::{File, OpenOptions};
use std::path::{Path};
use std::io::{Write, BufReader, BufRead};

// Loads a list of include paths from the config file. The function takes 
// an optional string argument. If the argument is not present or the 
// provided argument cannot be opened, then the function will attempt 
// to load the default config file. If that cannot be loaded, the function 
// will create a default config file in the default path
pub fn load_paths_from_config(config_path: Option<&str>) -> Vec<String> {
    let default_config_path = "~/.ensh_config";

    // Try to load given path, or use default if no string was supplied
    let config_path = config_path.unwrap_or(default_config_path);

    // See if the file can be opened
    let file = File::open(config_path);
    let default_exists = Path::new(default_config_path).exists();

    // Open the default file if the supplied location fails
    let file = match file {
        Ok(file) => file,
        Err(_) => { 
            // Try to read from default config
            // Create a new default config if necessary
            if default_exists {
                File::open(default_config_path).unwrap()
            } else {
                create_default_config(default_config_path).unwrap();
                File::open(default_config_path).unwrap()
            }
        }
    };

    let reader = BufReader::new(file);
    let mut result = Vec::new();

    // Read config file line by line, adding paths that exist
    for line in reader.lines().enumerate() {
        if Path::new(default_config_path).exists() {
            let line_result = line.1;
            result.push(line_result.unwrap().to_string());
        }
    }
    result
}

// Creates the default configuration file in the default location. Will 
// panic if for some reason cannot write default config to path
fn create_default_config(file_path: &str) -> Result<(), io::Error> {
    // Need to use openoptions to write ot a file (the regular create file
    // creates a file in read-only mode)
    let path_obj = Path::new(file_path);
    let mut path_str = path_obj.to_str().unwrap();
    let mut file = File::create(path_str)?;

    // Write each path as a line to the file
    let default_paths = vec! [
        "/usr/bin",
        "/usr/local/bin",
        "/usr/local/sbin",
        "/bin",
        "/usr/sbin",
    ];

    for line in default_paths {
        file.write(line.as_bytes())?; 
        file.write("\n".as_bytes())?;
    }
    file.sync_all()?;
    Ok(())
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test if the config file is able to write to the filesystem
    #[test]
    fn test_write_config_fs() {
        create_default_config("test_config").expect("Unable to write file");
    }


    // Test if the shell can read a config file from the filesystem
    #[test]
    fn test_read_config_fs() {
        load_paths_from_config(Some("/Users/aenayet/test_config"));
    }

    // Tests that the functions are able to both read and write to a file 
    // correctly with the proper output
    #[test]
    fn verify_read_write_config() {
        let config_path = "./test_config_rw";
        create_default_config(config_path).expect("Unable to write config file");
        let paths = load_paths_from_config(Some(config_path));
        assert_eq!(paths.len(), 1);
    }
}

