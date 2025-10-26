use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn copy_dir<T: AsRef<Path>>(src_path_ref: T, dest_path_ref: T) {
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

pub fn get_app_dir<P: AsRef<Path>>(home_dir: P, app: &String) -> io::Result<PathBuf> {
    let app_dir = home_dir.as_ref().join(format!(".config/{}", app));
    if !app_dir.is_dir() {
        return Err(Error::new(
            ErrorKind::NotADirectory,
            format!("Error: '{}' is not a directory.", app_dir.display()),
        ));
    }
    Ok(app_dir)
}

pub fn reset_tmp_dir<P: AsRef<Path>>(home_dir: P, app: &String) -> io::Result<PathBuf> {
    let tmp_config_dir = home_dir.as_ref().join(format!(".config/{}/tmp", app));
    if tmp_config_dir.is_dir() {
        fs::remove_dir_all(&tmp_config_dir)?;
    }
    fs::create_dir_all(&tmp_config_dir)?;
    Ok(tmp_config_dir)
}

pub fn select_config<P: AsRef<Path>>(
    home_dir: P,
    app: &String,
    config: &String,
) -> io::Result<PathBuf> {
    let app_dir = get_app_dir(home_dir, app)?;
        let source_dir = app_dir.join(format!("possible-configs/{}", config));
    if !source_dir.is_dir() {
        return Err(Error::new(
            ErrorKind::NotADirectory,
            format!(
                "Error: Configuration '{}' not found at '{}'",
                config,
                source_dir.display()
            ),
        ));
    }
    Ok(source_dir)
}
