use std::{fs::{self, create_dir, create_dir_all}, collections::HashMap, path::{Path, PathBuf}};

use chrono::{NaiveDate, DateTime, Utc, Datelike};
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

struct OutpathCache {
    root: String,
    date_path_cache: HashMap<NaiveDate, String>
}

impl OutpathCache {
    fn new(root: &str) -> OutpathCache {
        OutpathCache { root: root.to_string(), date_path_cache: HashMap::new() }
    }

    fn get_or_create(&mut self, date: &NaiveDate) -> anyhow::Result<String> {
        if let Some(path) = self.date_path_cache.get(date) {
            return Ok(path.clone())
        }

        let search_path = Path::new(&self.root).join(&date.year().to_string()).join(format!("{}-{:0>2}-{:0>2}", &date.year(), &date.month(), &date.day()));
        let year_path = Path::new(&self.root).join(&date.year().to_string());

        //println!("searchpath: {:?} yearpath: {:?}", search_path, year_path);

        if !year_path.exists() {
            let search_path = search_path.to_str().unwrap().to_string();

            create_dir_all(&search_path)?;
            self.date_path_cache.insert(date.clone(), search_path.clone());
            return Ok(search_path);
        }


        let prefix = format!("{}/{}-{:0>2}-{:0>2}", &date.year(), &date.year(), &date.month(), &date.day());
        for dir in WalkDir::new(&year_path).min_depth(1).max_depth(1) {
            let dir = dir?;

            if !dir.file_type().is_dir() {
                continue
            }

            let possible_path = dir.path().strip_prefix(&self.root)?.to_str().unwrap();

            if possible_path.starts_with(&prefix) {
                self.date_path_cache.insert(date.clone(), dir.path().display().to_string());
                return Ok(dir.path().display().to_string())
            }
        }


        let search_path = search_path.to_str().unwrap().to_string();
        create_dir_all(&search_path)?;
        self.date_path_cache.insert(date.clone(), search_path.clone());
        return Ok(search_path);
    }
}

fn move_files(parsed_files: &Vec<FileEntry>, target_root: &str, dry_run: &bool) -> anyhow::Result<()> {
    let mut path_cache = OutpathCache::new(target_root);

    for file in parsed_files {
        let target_path = path_cache.get_or_create(&file.target_date)?;
        
        let file_name = Path::new(&file.source).file_name().unwrap();
        let target_path = Path::new(&target_path).join(&file_name);

        println!("file: {:?} date: {:?} target: {:?}",file.source, file.target_date, target_path);

        if *dry_run {
            continue;
        }

        if fs::rename(&file.source, &target_path).is_err() {
            fs::copy(&file.source, &target_path)?;
            fs::remove_file(&file.source)?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let matches = Command::new("photosort")
        .arg(Arg::new("input").required(true))
        .arg(Arg::new("output").required(true))
        .arg(Arg::new("dry-run").short('d').help("Dry run. Does move/copy/delelte any files."))
        .get_matches();

    let input_path: &String = matches.get_one::<String>("input").unwrap();
    let dry_run: &bool = matches.get_one::<bool>("dry-run").unwrap();
    let input_files = list_files(input_path, Some(".JPG"))?;
    let parsed_files = parse_files(&input_files)?;

    let output_path: &String = matches.get_one::<String>("output").unwrap();
    move_files(&parsed_files, output_path, dry_run)?;

    Ok(())
}
