use std::collections::HashMap;
use std::fs::{self, Metadata};
use std::hash::Hasher;
use std::path::Path;
use std::io::{Read};
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone)]
pub enum FileState {
    New,
    Modified,
    Unchanged,
    Deleted,
}

/// Scan a directory and return a map of relative paths to file metadata
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

/// Compare the metadata and optionally hashes of two files
fn files_are_different(source_path: &Path, target_path: &Path, source_meta: &Metadata, target_meta: &Metadata) -> bool {
    if source_meta.len() != target_meta.len() {
        return true; // File sizes are different
    }

    // If file sizes are the same, check modification times as a quick heuristic
    if source_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH)
        != target_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH)
    {
        return true;
    }

    // As a last resort, compare file contents using a hash
    let source_hash = compute_file_hash(source_path);
    let target_hash = compute_file_hash(target_path);

    source_hash != target_hash
}

/// Compute a simple hash of the file content
fn compute_file_hash(path: &Path) -> Option<u64> {
    let mut file = fs::File::open(path).ok()?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut buffer = [0; 8192];

    while let Ok(read_bytes) = file.read(&mut buffer) {
        if read_bytes == 0 {
            break;
        }
        hasher.write(&buffer[..read_bytes]);
    }

    Some(hasher.finish())
}

/// Compare source and target directories
pub fn compare_directories(source: &str, target: &str) -> Vec<(String, FileState)> {
    let source_files = scan_directory(source);
    let target_files = scan_directory(target);

    let mut changes = Vec::new();

    for (path, source_meta) in &source_files {
        if let Some(target_meta) = target_files.get(path) {
            let source_path = Path::new(source).join(path);
            let target_path = Path::new(target).join(path);

            if files_are_different(&source_path, &target_path, source_meta, target_meta) {
                changes.push((path.clone(), FileState::Modified));
            } else {
                changes.push((path.clone(), FileState::Unchanged));
            }
        } else {
            changes.push((path.clone(), FileState::New));
        }
    }

    // Handle deleted files
    for (path, _) in &target_files {
        if !source_files.contains_key(path) {
            changes.push((path.clone(), FileState::Deleted));
        }
    }

    changes
}
