use std::{fs, time::UNIX_EPOCH, os::unix::prelude::MetadataExt};

use chrono::{Date, TimeZone, NaiveDate, DateTime, Utc};
use clap::{Arg, Command};
use walkdir::WalkDir;

fn list_files(path: &str, ends_with : Option<&str>) -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?.path().display().to_string();

        if let Some(ends_with) = ends_with {
            if !entry.ends_with(ends_with) {
                continue;
            }
        }

        result.push(entry);
    }
    Ok(result)
}

#[derive(Debug)]
struct FileEntry {
    source: String,
    target_date: NaiveDate
}

fn parse_files(input_files : &Vec<String>) -> anyhow::Result<Vec<FileEntry>> {
    let mut result = Vec::new();

    for file in input_files {
        let metadata = fs::metadata(file)?;
        let modified = metadata.modified()?;
        let target_date: DateTime<Utc> = modified.into();

        result.push(FileEntry { source: file.clone(), target_date: target_date.date_naive() });
    }

    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let matches = Command::new("photosort")
        .arg(Arg::new("input").required(true))
        .get_matches();

    let input_path: &String = matches.get_one::<String>("input").unwrap();
    let input_files = list_files(input_path, Some(".JPG"))?;

    let parsed_files = parse_files(&input_files)?;

    println!("Files: {:?}", parsed_files);
    Ok(())
}
