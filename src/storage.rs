use domain::*;
use serde_yaml;
use std::collections::HashMap;
use std::path::PathBuf;
use self::readwrite::*;

pub fn save(session: &Section) {
    use std::thread;
    let session = session.clone();
    thread::spawn(move || {
        save_session(&session);
    });
}

fn save_session(session: &Section) {
    if let Some(ref folder) = home_folder() {
        if let Ok(string) = serde_yaml::to_string(&session.lesson_results) {
            write_string(folder, LESSON_RESULTS_FILE, &string);
        }
        if let Ok(string) = serde_yaml::to_string(&session.questions) {
            write_string(folder, QUESTIONS_FILE, &string);
        }
    }
}

const QUESTIONS_FILE: &str = "questions.yaml";
const LESSON_RESULTS_FILE: &str = "lesson_results.yaml";

pub fn load_course() -> Course {
    Course {
        name: "Jzero".into(),
        sections: vec![
            load_section("Vocabulary Group A", vocabulary_group_a_questions(), vocab_a_folder()),
            load_section("Vocabulary Group B", vocabulary_group_b_questions(), vocab_b_folder()),
        ],
        active_section: None,
    }
}

fn load_section(name: &str, questions: Vec<Question>, folder: Option<PathBuf>) -> Section {
    let mut section = Section::new(name, questions);
    if let Some(ref folder) = folder {
        if let Some(ref string) = read_string(folder, LESSON_RESULTS_FILE) {
            section.lesson_results = serde_yaml::from_str(string).unwrap_or(HashMap::new())
        }
    }
    section
}

mod readwrite {
    use std::fs;
    use std::fs::File;
    use std::path::{PathBuf, Path};
    use std::env;

    pub fn vocab_a_folder() -> Option<PathBuf> {
        home_folder().map(|it| it.join("vocab_a"))
    }

    pub fn vocab_b_folder() -> Option<PathBuf> {
        home_folder().map(|it| it.join("vocab_b"))
    }

    pub fn home_folder() -> Option<PathBuf> {
        env::home_dir().map(|it| it.join(".jzero"))
    }

    pub fn read_string(folder: &PathBuf, file: &str) -> Option<String> {
        use std::io::Read;
        let path = folder.join(Path::new(file));
        File::open(path).ok().and_then(|mut f| {
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).ok().map(|_| { buffer })
        })
    }

    pub fn write_string(folder: &PathBuf, file: &str, string: &String) {
        use std::io::Write;
        fs::create_dir_all(folder).expect("Unable to create folder");
        let path = folder.join(Path::new(file));
        let mut f = File::create(path).expect("Unable to create file");
        f.write_all(string.as_bytes()).expect("Unable to write string");
    }
}

