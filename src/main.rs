use std::{
    fs::{self, read_dir, remove_dir_all, remove_file, ReadDir},
    io,
    path::Path,
    time::{Duration, SystemTime},
};

fn is_directory_empty(path: &Path) -> std::io::Result<bool> {
    let entries = read_dir(path)?;
    for entry in entries.flatten() {
        let path = entry.path();
        if should_skip_file(&path) {
            continue;
        }
        return Ok(false);
    }
    Ok(true)
}

fn should_skip_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        let file_name = file_name.to_string_lossy();
        file_name.starts_with('.') || file_name == "$RECYCLE.BIN"
    } else {
        false
    }
}

fn process_file(entry: &fs::DirEntry, time_offset: SystemTime) -> io::Result<()> {
    let metadata = entry.metadata()?;
    let created = metadata.created()?;
    if metadata.is_file() && created < time_offset {
        remove_file(entry.path())?;
    }
    Ok(())
}

fn walk_dir(dir: ReadDir, time_offset: SystemTime) -> io::Result<()> {
    for entry in dir.flatten() {
        let path = entry.path();
        if should_skip_file(&path) {
            continue;
        }
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if is_directory_empty(&path)? {
                remove_dir_all(path)?;
            } else {
                walk_dir(read_dir(path)?, time_offset)?;
            }
        } else {
            process_file(&entry, time_offset)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let home_dir = std::env::var("HOME").expect("Error getting home directory");
    let downloads_dir =
        fs::read_dir(format!("{home_dir}/Downloads")).expect("Error reading downloads directory");
    let time_offset = SystemTime::now() - Duration::from_secs(60 * 60 * 24 * 7 * 2);

    walk_dir(downloads_dir, time_offset)?;

    Ok(())
}
