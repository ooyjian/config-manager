use std::fs;
use std::path::Path;

fn copy_dir<T: AsRef<Path>>(src_path_ref: T, dest_path_ref: T) {
    let src_path = src_path_ref.as_ref();
    let dest_path = dest_path_ref.as_ref();
    if !src_path.is_dir() {
        eprintln!("Error: {} is not a directory", src_path.display());
        return;
    }
    if !dest_path.is_dir() {
        eprintln!("Error: {} is not a directory", dest_path.display());
        return;
    }
    if src_path == dest_path {
        println!("src_path is the same as dest_path: {}", src_path.display());
        return;
    }
    println!("Copying {} to {}", src_path.display(), dest_path.display());

    let dir = dest_path.join(src_path.file_name().unwrap_or_default());
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Failed to create directory {}: {}", dir.display(), e);
    }

    match fs::read_dir(src_path) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                let last_element = entry.file_name();
                if path.is_dir() {
                    let child_dest = dest_path.join(last_element);
                    if let Err(e) = fs::create_dir_all(&child_dest) {
                        eprintln!("Failed to create directory {}: {}", child_dest.display(), e);
                        return;
                    }
                    println!("Created directory {}", child_dest.display());
                    copy_dir(path, child_dest);
                } else if path.is_file() {
                    if let Err(e) = fs::copy(&path, dest_path.join(last_element)) {
                        eprintln!(
                            "Failed to copy '{}' to '{}': {}",
                            path.display(),
                            dest_path.display(),
                            e
                        );
                    } else {
                        println!("Copied '{}' to '{}'", path.display(), dest_path.display());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "Error: Could not read source directory '{}': {}",
                src_path.display(),
                e
            );
        }
    }
}

fn main() {
    // the path we are looking for is $HOME/.config/{app_name}/possible-configs/{config_name}
    let app_name = std::env::args().nth(1).expect("application not specified");
    let config_name = std::env::args().nth(2).expect("config not specified");

    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not find home directory.");
            std::process::exit(1);
        }
    };
    let app_dir = home_dir.join(format!(".config/{}", app_name));
    if !app_dir.is_dir() {
        eprintln!("Error: '{}' is not a directory.", app_dir.display());
        std::process::exit(1);
    }

    let tmp_config_dir = home_dir.join(format!(".config/{}/tmp", app_name));
    if tmp_config_dir.is_dir() {
        if let Err(e) = fs::remove_dir_all(&tmp_config_dir) {
            eprintln!(
                "Failed to remove tmp directory '{}': {}",
                tmp_config_dir.display(),
                e
            );
            std::process::exit(1);
        }
    }
    if let Err(e) = fs::create_dir_all(&tmp_config_dir) {
        eprintln!(
            "Failed to create directory {}: {}",
            tmp_config_dir.display(),
            e
        );
        std::process::exit(1)
    }

    let possible_configs_dir = app_dir.join("possible-configs");
    let source_dir = possible_configs_dir.join(&config_name);
    if !source_dir.is_dir() {
        eprintln!(
            "Error: Configuration '{}' not found at '{}'",
            config_name,
            source_dir.display()
        );
        std::process::exit(1);
    }

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
            eprintln!(
                "Error: Could not read waybar directory '{}': {}",
                app_dir.display(),
                e
            );
            std::process::exit(1);
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
            eprintln!(
                "Error: Could not read source directory '{}': {}",
                source_dir.display(),
                e
            );
        }
    }
}
