use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

use crate::helpers::{self, copy_dir, get_app_dir};

pub fn apply<P: AsRef<Path>>(app: String, config: String, home_dir: P) -> io::Result<()> {
    let app_dir = get_app_dir(&home_dir, &app)?;
    let tmp_config_dir = helpers::reset_tmp_dir(&home_dir, &app)?;
    let source_dir = helpers::select_config(&home_dir, &app, &config)?;

    println!("Reading from {}", app_dir.display());
    match fs::read_dir(&app_dir) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                let last_element = entry.file_name();
                if last_element != "possible-configs" && last_element != "tmp" {
                    let dest_path = tmp_config_dir.join(last_element);
                    if let Err(e) = fs::rename(&path, dest_path) {
                        eprintln!("Failed to move to tmp '{}': {}", path.display(), e);
                    } else {
                        println!("Moved {} to tmp", path.display());
                    }
                }
            }
        }
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Error: Could not read waybar directory '{}': {}",
                    app_dir.display(),
                    e
                ),
            ));
        }
    }

    // Copy files from the source directory
    println!("Reading from {}", source_dir.display());
    match fs::read_dir(&source_dir) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let source_path = entry.path();
                println!("source path: {}", source_path.display());
                if source_path.is_file() {
                    let file_name = entry.file_name();
                    if let Err(e) = fs::copy(&source_path, app_dir.join(file_name)) {
                        eprintln!(
                            "Failed to copy '{}' to '{}': {}",
                            source_path.display(),
                            app_dir.display(),
                            e
                        );
                    } else {
                        println!(
                            "Copied '{}' to '{}'",
                            source_path.display(),
                            app_dir.display()
                        );
                    }
                } else if source_path.is_dir() {
                    copy_dir(&source_path, &app_dir);
                }
            }
        }
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Error: Could not read source directory '{}': {}",
                    source_dir.display(),
                    e
                ),
            ));
        }
    }

    Ok(())
}

pub fn add<P: AsRef<Path>>(app: String, config: String, home_dir: P) -> io::Result<()> {
    let app_dir = get_app_dir(home_dir, &app)?;
    let source_dir = app_dir.join(format!("possible-configs/{}", config));
    let config_exists = fs::exists(&source_dir)?;
    if config_exists {
        return Err(Error::new(
            ErrorKind::Other,
            format!("The config path {} exists", &source_dir.display()),
        ));
    }
    fs::create_dir_all(source_dir) 
}

pub fn delete<P: AsRef<Path>>(app: String, config: String, home_dir: P) -> io::Result<()> {
    let app_dir = get_app_dir(home_dir, &app)?;
    let source_dir = app_dir.join(format!("possible-configs/{}", config));
    let config_exists = fs::exists(&source_dir)?;
    if config_exists {
        fs::remove_dir_all(source_dir)?; 
    }
    Ok(())
}
