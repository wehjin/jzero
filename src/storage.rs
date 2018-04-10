use domain::*;
use serde_yaml;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn save(session: &Session) {
    use std::thread;
    let session = session.clone();
    thread::spawn(move || {
        save_session(&session);
    });
}

fn save_session(session: &Session) {
    if let Some(path_buf) = env::home_dir() {
        let folder = &path_buf.join(Path::new(".jzero"));
        fs::create_dir_all(folder).expect("Unable to create folder");
        if let Ok(string) = serde_yaml::to_string(&session.lesson_results) {
            save_value(folder, "lesson_results.yaml", &string);
        }
        if let Ok(string) = serde_yaml::to_string(&session.questions) {
            save_value(folder, "questions.yaml", &string);
        }
    }
}


fn save_value(folder: &PathBuf, file: &str, string: &String) {
    let path = folder.join(Path::new(file));
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(string.as_bytes()).expect("Unable to write questions");
}
