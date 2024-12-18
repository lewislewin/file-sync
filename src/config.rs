use std::string::String;

pub struct Config {
    pub source_dir: String,
    pub target_dir: String,
    pub dry_run: bool,
    pub delete: bool,
}

impl Config {
    pub fn new(
        source: Option<String>,
        target: Option<String>,
        dry_run: Option<bool>,
        delete: Option<bool>,
    ) -> Self {
        Self {
            source_dir: source.unwrap_or_else(|| "./source".to_string()),
            target_dir: target.unwrap_or_else(|| "./target".to_string()),
            dry_run: dry_run.unwrap_or(false),
            delete: delete.unwrap_or(true),
        }
    }
}
