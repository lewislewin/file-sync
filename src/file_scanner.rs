use std::collections::HashMap;
use std::fs::Metadata;

#[derive(Debug, PartialEq, Clone)]
pub enum FileState {
    New,
    Modified,
    Unchanged,
    Deleted,
}

pub fn scan_directory(path: &str) -> HashMap<String, Metadata> {
    let mut files = HashMap::new();
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let relative_path = entry.path().strip_prefix(path).unwrap().to_string_lossy().to_string();
        files.insert(relative_path, entry.metadata().unwrap());
    }
    files
}

pub fn compare_directories(
    source: &str,
    target: &str,
) -> Vec<(String, FileState)> {
    let source_files = scan_directory(source);
    let target_files = scan_directory(target);

    let mut changes = Vec::new();

    for (path, source_meta) in &source_files {
        if let Some(target_meta) = target_files.get(path) {
            if source_meta.modified().unwrap() != target_meta.modified().unwrap() {
                changes.push((path.clone(), FileState::Modified));
            } else {
                changes.push((path.clone(), FileState::Unchanged));
            }
        } else {
            changes.push((path.clone(), FileState::New));
        }
    }

    if target_files.len() > source_files.len() {
        for (path, _) in &target_files {
            if !source_files.contains_key(path) {
                changes.push((path.clone(), FileState::Deleted));
            }
        }
    }

    changes
}
