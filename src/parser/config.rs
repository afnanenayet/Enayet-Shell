/// config.rs    Afnan Enayet
///
/// The config module provides helper/convenience functions to parse
/// the configuration file for the shell and write configuration file
/// to default directory or any other directory

use std::io;
use std::fs::{self, File};
use std::path::Path;
use std::io::{Write, BufReader, BufRead};
use consts::{DEF_CONFIG_FNAME, DEFAULT_PATHS};

/// Loads a list of include paths from the config file. The function takes
/// an optional string argument. If the argument is not present or the
/// provided argument cannot be opened, then the function will attempt
/// to load the default config file. If that cannot be loaded, the function
/// will create a default config file in the default path
pub fn load_paths_from_config(config_path: Option<&str>, def_paths: &Vec<String>) -> Vec<String> {
    // Try to load given path, or use default if no string was supplied
    let def_config_fp = format!("~/{}", DEF_CONFIG_FNAME);
    let config_path = config_path.unwrap_or(def_config_fp.as_str());
    let config_path = super::norm_abs_path(config_path).unwrap();

    // See if the file can be opened
    let default_exists = Path::new(DEF_CONFIG_FNAME).exists();

    // Open the default file if the supplied location fails
    let file = match File::open(&config_path) {
        Ok(file) => file,
        Err(_) => {
            // Try to read from default config
            // Create a new default config if necessary
            if !default_exists {
                create_default_config(DEF_CONFIG_FNAME, def_paths).unwrap();
            }
            File::open(DEF_CONFIG_FNAME).unwrap()
        }
    };

    let reader = BufReader::new(file);
    let mut result = Vec::new();

    // Read config file line by line and load into vector
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

/// Creates the default configuration file in the default location. Will
/// return error if for some reason cannot it cannot write default config to path
/// Expects `file_path` to be a valid file path, since it cannot verify the path 
/// of an unwritten file
pub fn create_default_config(file_path: &str, def_paths: &Vec<String>) -> Result<(), io::Error> {
    // Need to use openoptions to write to a file (the regular create file
    // creates a file in read-only mode)
    let mut file = File::create(file_path)?;

    // Write each path into the config file
    for line in def_paths {
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
    use std::env;
    use std::fs::remove_file;

    // Returns a vector of default paths for use in tests
    fn create_def_paths() -> Vec<String> {
        let paths: Vec<String> = [
            "/usr/bin".to_string(),
            "/usr/local/bin".to_string()
        ].to_vec();
        paths
    }

    // Test if the config file is able to write to the filesystem
    #[test]
    fn test_write_config_fs() {
        let tmp_dir = env::temp_dir();
        let default_paths = create_def_paths();
        let full_path = format!("{}/test_config_write", tmp_dir.to_str().unwrap());
        create_default_config(&full_path, &default_paths).unwrap();
    }

    // Test if the shell can read a config file from the filesystem
    #[test]
    fn test_read_config_fs() {
        let tmp_dir = env::temp_dir(); 
        test_write_config_fs(); // need to write file before we read it
        let path = format!("{}/test_config_write", tmp_dir.to_str().unwrap());
        let default_paths = create_def_paths();
        load_paths_from_config(Some(&path), &default_paths);
    }

    // Tests that the functions are able to both read and write to a file
    // correctly, as well as store the results in a vector
    #[test]
    fn verify_read_write_config() {
        let tmp_dir = env::temp_dir();
        let config_path = format!("{}/test_config_rw", tmp_dir.to_str().unwrap());
        let default_paths = create_def_paths();
        create_default_config(&config_path, &default_paths).expect("Unable to write config file");
        let paths = load_paths_from_config(Some(&config_path), &default_paths);
        assert_eq!(paths.len(), default_paths.len());
    }

    // Tests that function can create the default config if no config path is
    // supplied
    #[test]
    fn test_create_def_config() {
        let mut default_paths = Vec::new();
        let tmp_fp = env::temp_dir();
        let tmp_fp = format!("{}/{}", tmp_fp.to_str().unwrap(), DEF_CONFIG_FNAME);

        // Create vector for default path from array
        for path in DEFAULT_PATHS {
            default_paths.push(path.to_string());
        }
        create_default_config(tmp_fp.as_str(), &default_paths).unwrap();
    }
}
