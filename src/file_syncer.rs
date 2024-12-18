use std::fs::File;
use std::{fs, io};
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
        let source_path: std::path::PathBuf = Path::new(source).join(&file);
        let target_path: std::path::PathBuf = Path::new(target).join(&file);

        match state {
            FileState::New | FileState::Modified => {
                println!("Copying: {}", file);
                if !dry_run {
                    if let Some(parent) = target_path.parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    fs::copy(&source_path, &target_path).unwrap();

                    // Copy file metadata
                    copy_metadata(&source_path, &target_path).unwrap();
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

fn copy_metadata(source: &Path, target: &Path) -> io::Result<()> {
    let metadata: fs::Metadata = fs::metadata(source)?;
    let modified_time: std::time::SystemTime = metadata.modified()?;

    let _file = File::open(target)?;
    filetime::set_file_mtime(target, filetime::FileTime::from_system_time(modified_time))
}