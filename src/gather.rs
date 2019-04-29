use std::error::Error;
use std::path::Path;

extern crate walkdir;

use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
         .file_name()
         .to_str()
         .map(|s| entry.depth() == 0 || !s.starts_with("."))
         .unwrap_or(false)
}

pub fn traverse(path: &Path) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| is_not_hidden(e))
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".rs") {
            println!("{}", entry.path().display());
        }
    }

    Ok(())
}