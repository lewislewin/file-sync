use std::fs;
use std::path::Path;

use crate::file_scanner::FileState;

pub fn synchronize_files(
    source: &str,
    target: &str,
    changes: Vec<(String, FileState)>,
    dry_run: bool,
    delete: bool,
) {
    for (file, state) in changes {
        let source_path = Path::new(source).join(&file);
        let target_path = Path::new(target).join(&file);

        match state {
            FileState::New | FileState::Modified => {
                println!("Copying: {}", file);
                if !dry_run {
                    if let Some(parent) = target_path.parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    fs::copy(&source_path, &target_path).unwrap();
                }
            }
            FileState::Deleted if delete => {
                println!("Deleting: {}", file);
                if !dry_run {
                    fs::remove_file(&target_path).unwrap();
                }
            }
            _ => {}
        }
    }
}
