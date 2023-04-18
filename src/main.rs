use std::{fs};

use chrono::{NaiveDate, DateTime, Utc};
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

fn move_files(parse_files: &Vec<FileEntry>, target_root: &str) -> anyhow::Result<()> {

    let mut target_dirs = Vec::new();
    for entry in WalkDir::new(target_root) {
        let entry = entry?;
        if !entry.metadata()?.is_dir() {
            continue;
        }

        target_dirs.push(entry.path().display().to_string());
    }

    println!("target_root: {:?}", target_root);
    println!("target_dirs: {:?}", target_dirs);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let matches = Command::new("photosort")
        .arg(Arg::new("input").required(true))
        .arg(Arg::new("output").required(true))
        .get_matches();

    let input_path: &String = matches.get_one::<String>("input").unwrap();
    let input_files = list_files(input_path, Some(".JPG"))?;
    let parsed_files = parse_files(&input_files)?;

    let output_path: &String = matches.get_one::<String>("output").unwrap();
    move_files(&parsed_files, output_path)?;

    Ok(())
}
