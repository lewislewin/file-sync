use std::fs::OpenOptions;
use std::io::Write;

pub fn log_action(action: &str) {
    let mut file: std::fs::File = OpenOptions::new()
        .append(true)
        .create(true)
        .open("sync.log")
        .unwrap();

    writeln!(file, "{}", action).unwrap();
}
