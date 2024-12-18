mod config;
mod file_scanner;
mod file_syncer;
mod logger;

use config::Config;
use file_scanner::compare_directories;
use file_syncer::synchronize_files;
use logger::log_action;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source directory
    #[arg(short, long)]
    source: String,

    /// Target directory
    #[arg(short, long)]
    target: String,
}

fn main() {
    let args: Args = Args::parse();
    
    let config: Config = Config::new(Some(args.source), Some(args.target), None, None);

    println!("Starting synchronization...");
    let changes: Vec<(String, file_scanner::FileState)> = compare_directories(&config.source_dir, &config.target_dir);

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
