pub struct Config {
    pub source_dir: String,
    pub target_dir: String,
    pub dry_run: bool,
    pub delete: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            source_dir: "./source".to_string(),
            target_dir: "./target".to_string(),
            dry_run: false,
            delete: true,
        }
    }
}
