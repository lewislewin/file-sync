mod config;
mod file_scanner;
mod file_syncer;
mod logger;

use config::Config;
use file_scanner::compare_directories;
use file_syncer::synchronize_files;
use logger::log_action;

fn main() {
    let config = Config::new();

    println!("Starting synchronization...");
    let changes = compare_directories(&config.source_dir, &config.target_dir);

    for (file, state) in &changes {
        println!("{:?}: {}", state, file);
    }

    synchronize_files(
        &config.source_dir,
        &config.target_dir,
        changes.clone(),
        config.dry_run,
        config.delete,
    );

    for (file, state) in changes {
        log_action(&format!("{:?}: {}", state, file));
    }

    println!("Synchronization complete.");
}
