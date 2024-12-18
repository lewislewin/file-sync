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
    /// Source directory (positional or --source)
    #[arg(short = 's', long)]
    source: Option<String>,

    /// Target directory (positional or --target)
    #[arg(short = 't', long)]
    target: Option<String>,

    /// Dry-run mode (default: false)
    #[arg(short = 'r', long, default_value_t = false)]
    dry_run: bool,

    /// Delete extra files in the target directory (default: true)
    #[arg(short = 'x', long, default_value_t = true)]
    delete: bool,

    /// Positional source directory
    #[arg(index = 1)]
    positional_source: Option<String>,

    /// Positional target directory
    #[arg(index = 2)]
    positional_target: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let source = args
        .positional_source
        .or(args.source)
        .expect("Source directory must be specified either positionally or with --source");
    let target = args
        .positional_target
        .or(args.target)
        .expect("Target directory must be specified either positionally or with --target");

    // Create the configuration
    let config = Config::new(Some(source), Some(target), Some(args.dry_run), Some(args.delete));

    println!("Starting synchronization...");
    let changes: Vec<(String, file_scanner::FileState)> =
        compare_directories(&config.source_dir, &config.target_dir);

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
