/// config.rs    Afnan Enayet
///
/// The config module provides helper/convenience functions to parse 
/// the configuraiton file for the shell 

use std::io;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Write, BufReader, BufRead};

// Loads a list of include paths from the config file. The function takes 
// an optional string argument. If the argument is not present or the 
// provided argument cannot be opened, then the function will attempt 
// to load the default config file. If that cannot be loaded, the function 
// will create a default config file in the default path
pub fn load_paths_from_config(config_path: Option<&str>) -> Vec<String> {
    let DEFAULT_CONFIG_PATH = "~/.ensh_config";

    // Try to load given path, or use default if no string was supplied
    let config_path = config_path.unwrap_or(DEFAULT_CONFIG_PATH);

    // See if the file can be opened
    let file = File::open(config_path);
    let default_exists = Path::new(DEFAULT_CONFIG_PATH).exists();

    // Open the default file if the supplied location fails
    let file = match file {
        Ok(file) => file,
        Err(_) => { 
            // Try to read from default config
            // Create a new default config if necessary
            if default_exists {
                File::open(DEFAULT_CONFIG_PATH).unwrap()
            } else {
                create_default_config(DEFAULT_CONFIG_PATH).unwrap();
                File::open(DEFAULT_CONFIG_PATH).unwrap()
            }
        }
    };

    let reader = BufReader::new(file);
    let mut result = Vec::new();

    // Read config file line by line, adding paths that exist
    for line in reader.lines().enumerate() {
        if Path::new(DEFAULT_CONFIG_PATH).exists() {
            let line_result = line.1;
            result.push(line_result.unwrap().to_string());
        }
    }
    result
}

// Creates the default configuration file in the default location. Will 
// panic if for some reason cannot write default config to path
fn create_default_config(file_path: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path).unwrap();

    let DEFAULT_PATHS = vec! [
        "hello",
    ];

    for line in DEFAULT_PATHS {
        file.write(line.as_bytes()).unwrap(); 
    }
    file.flush()?;
    Ok(())
}

// unit tests
#[cfg(test)]
mod tests {
}

