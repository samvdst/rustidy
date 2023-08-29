extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use std::fs::{self};

struct File {
    name: String,
    created: DateTime<Utc>,
}

fn main() {
    let home_dir = std::env::var("HOME").expect("Error getting home directory");
    let downloads_dir =
        fs::read_dir(format!("{home_dir}/Downloads")).expect("Error reading downloads directory");

    let mut files: Vec<File> = Vec::new();
    for dir_entry in downloads_dir {
        let entry = dir_entry.expect("Error reading entry");
        let meta = entry.metadata().expect("Error reading metadata");

        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with('.') || file_name == "$RECYCLE.BIN" {
            continue;
        }

        files.push(File {
            name: entry.file_name().into_string().unwrap(),
            created: meta.created().unwrap().into(),
        });
    }

    files.into_iter().for_each(|file| {
        println!(
            "{}: {}",
            file.created.format("%Y-%m-%d %H:%M:%S"),
            file.name
        );
    });
}
