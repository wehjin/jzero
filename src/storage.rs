use domain::*;
use serde_yaml;
use std::collections::HashMap;
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
    if let Some(ref folder) = home_folder() {
        if let Ok(string) = serde_yaml::to_string(&session.lesson_results) {
            save_string(folder, LESSON_RESULTS_FILE, &string);
        }
        if let Ok(string) = serde_yaml::to_string(&session.questions) {
            save_string(folder, QUESTIONS_FILE, &string);
        }
    }
}

fn home_folder() -> Option<PathBuf> {
    if let Some(path_buf) = env::home_dir() {
        let folder = path_buf.join(Path::new(".jzero"));
        fs::create_dir_all(&folder).expect("Unable to create folder");
        Some(folder)
    } else {
        None
    }
}

const QUESTIONS_FILE: &str = "questions.yaml";
const LESSON_RESULTS_FILE: &str = "lesson_results.yaml";

fn save_string(folder: &PathBuf, file: &str, string: &String) {
    let path = folder.join(Path::new(file));
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(string.as_bytes()).expect("Unable to write questions");
}


pub fn load() -> Session {
    let mut session = Session::default();
    session.active_lesson = None;
    if let Some(ref folder) = home_folder() {
        if let Some(ref string) = read_string(folder, QUESTIONS_FILE) {
            session.questions = serde_yaml::from_str(string).unwrap_or(default_questions())
        }
        if let Some(ref string) = read_string(folder, LESSON_RESULTS_FILE) {
            session.lesson_results = serde_yaml::from_str(string).unwrap_or(HashMap::new())
        }
    }

    use chrono::prelude::*;
    session.start_next_lesson(Utc::now());
    session
}

fn read_string(folder: &PathBuf, file: &str) -> Option<String> {
    use std::io::Read;
    let path = folder.join(Path::new(file));
    File::open(path).ok().and_then(|mut f| {
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).ok().map(|_| { buffer })
    })
}
