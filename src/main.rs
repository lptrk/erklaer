use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

struct FileCache {
    cache: HashMap<PathBuf, String>,
}

impl FileCache {
    fn new() -> FileCache {
        FileCache {
            cache: HashMap::new(),
        }
    }

    fn get_file_content(&mut self, path: &PathBuf) -> io::Result<String> {
        if let Some(content) = self.cache.get(path) {
            Ok(content.clone())
        } else {
            let mut file = File::open(path)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            if let Ok(text) = String::from_utf8(contents) {
                self.cache.insert(path.clone(), text.clone());
                Ok(text)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "[WARN] File does not contain valid UTF-8-data.",
                ))
            }
        }
    }

    fn print_cache_contents(&self) {
        for (path, content) in self.cache.iter() {
            println!("[INFO] Path: {}", path.display());
        }
    }

    fn get_single_file(&self, file_name: &str) {
        let mut found = false;
        let file_name = file_name.trim();

        println!("[INFO] Cached data:");
        for path in self.cache.keys() {
            println!("  - {}", path.display());
        }

        for (path, content) in self.cache.iter() {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().to_lowercase() == file_name.to_lowercase() {
                    println!("Content'{}':\n{}", path.display(), content);
                    found = true;
                    break;
                }
            }
        }

        if !found {
            println!("[WARN] File '{}' not found in Cache.", file_name);
        }
    }
}

fn read_files_recursively<P: AsRef<Path>>(path: P, cache: &mut FileCache) -> io::Result<()> {
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path_buf = entry.path().to_path_buf();
        if path_buf.is_file()
            && !path_buf.to_string_lossy().contains("git")
            && !path_buf.to_string_lossy().contains("svg")
            && !path_buf.to_string_lossy().contains("png")
            && !path_buf.to_string_lossy().contains("jpg")
            && !path_buf.to_string_lossy().contains("DS_Store")
            && !path_buf.to_string_lossy().contains(".wav")
            && !path_buf.to_string_lossy().contains("provisionprofile")
        {
            if let Err(e) = cache.get_file_content(&path_buf) {
                println!("[ERROR] Error reading'{}': {}", path_buf.display(), e);
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("[CMD] Repo: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[ERROR] Failed to read line");

    let mut cache = FileCache::new();
    let first_start = Instant::now();

    read_files_recursively(&input.trim(), &mut cache)?;
    let first_duration = first_start.elapsed();

    let second_start = Instant::now();
    cache.print_cache_contents();
    let second_duration = second_start.elapsed();

    println!("[CMD] Search for a file: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[ERROR] Failed to read line");
    let third_start = Instant::now();

    cache.get_single_file(&input);

    let third_duration = third_start.elapsed();

    println!("[INFO] Time elapsed getting repo: {:?}", first_duration);
    println!(
        "[INFO] Time elapsed getting from cache: {:?}",
        second_duration
    );
    println!(
        "[INFO] Time elapsed getting a single file from cache: {:?}",
        third_duration
    );
    Ok(())
}
