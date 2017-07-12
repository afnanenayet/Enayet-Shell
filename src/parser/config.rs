/// config.rs    Afnan Enayet
///
/// The config module provides helper/convenience functions to parse 
/// the configuration file for the shell 

use std::io;
use std::fs::{self, File, OpenOptions};
use std::path::{Path};
use std::io::{Write, BufReader, BufRead};

// Loads a list of include paths from the config file. The function takes 
// an optional string argument. If the argument is not present or the 
// provided argument cannot be opened, then the function will attempt 
// to load the default config file. If that cannot be loaded, the function 
// will create a default config file in the default path
pub fn load_paths_from_config(config_path: Option<&str>, def_paths: &Vec<String>) 
    -> Vec<String> {
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
            // TODO debug remove
            println!("Could not read file");

            // Try to read from default config
            // Create a new default config if necessary
            if default_exists {
                File::open(default_config_path).unwrap()
            } else {
                create_default_config(default_config_path, def_paths).unwrap();
                File::open(default_config_path).unwrap()
            }
        }
    };

    let reader = BufReader::new(file);
    let mut result = Vec::new();

    // Read config file line by line
    for line in reader.lines().enumerate() {
        let line_result = line.1;
        let path_str = line_result.unwrap().to_string().clone();

        // Check if path is valid and add to vector
        if Path::new(&path_str).exists() {
            result.push(path_str);
        }
    }
    result
}

// Creates the default configuration file in the default location. Will 
// panic if for some reason cannot write default config to path
fn create_default_config(file_path: &str, def_paths: &Vec<String>) 
    -> Result<(), io::Error> {
    // Need to use openoptions to write ot a file (the regular create file
    // creates a file in read-only mode)
    let path_obj = Path::new(file_path);
    let mut path_str = path_obj.to_str().unwrap();
    let mut file = File::create(path_str)?;

    for line in def_paths {
        file.write(line.as_bytes())?; 
        file.write("\n".as_bytes())?;
    }
    file.sync_all()?;
    Ok(())
}

// Copies the default config file to the specified path
fn cp_def_config(src_path: &str, dest_path: &str) -> Result<u64, io::Error> {
    fs::copy(src_path, dest_path)
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    // Returns a vector of default paths for use in tests
    fn create_def_paths() -> Vec<String> {
        let paths: Vec<String> = [
            "/usr/bin".to_string(),
            "/usr/local/bin".to_string(),
        ].to_vec();
        paths
    }

    // Test if the config file is able to write to the filesystem
    #[test]
    fn test_write_config_fs() {
        let default_paths = create_def_paths();
        create_default_config("test_config", &default_paths)
            .expect("Unable to write file");
    }

    // Test if the shell can read a config file from the filesystem
    #[test]
    fn test_read_config_fs() {
        let default_paths = create_def_paths();
        load_paths_from_config(Some("unit_test_files/config_r"), &default_paths);
    }

    // Tests that the functions are able to both read and write to a file 
    // correctly, as well as store the results in a vector
    #[test]
    fn verify_read_write_config() {
        let config_path = "test_config_rw";
        let default_paths = create_def_paths();
        create_default_config(config_path, &default_paths)
            .expect("Unable to write config file");
        let paths = load_paths_from_config(Some(config_path), &default_paths);
        assert_eq!(paths.len(), 2);
    }
}

