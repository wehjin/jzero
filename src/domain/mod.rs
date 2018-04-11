use chrono::prelude::{DateTime, Utc};
use std::collections::HashMap;

mod lesson;
mod lesson_result;
mod section;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Section {
    pub questions: Vec<Question>,
    pub active_lesson: Option<Lesson>,
    pub lesson_results: HashMap<Question, LessonResult>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Question {
    Recall { english: String, progressive: String, kana: String, kanji: Option<String> }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LessonResult {
    Hard(DateTime<Utc>),
    Good(DateTime<Utc>),
    Easy(DateTime<Utc>),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub question: Question,
    pub progress: LessonProgress,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LessonProgress {
    Test,
    Learn,
    Review,
}

pub fn default_questions() -> Vec<Question> {
    vec![
        Question::Recall {
            english: "mouth".into(),
            progressive: "kuchi".into(),
            kana: "くち".into(),
            kanji: Some("口".into()),
        },
        Question::Recall {
            english: "eye".into(),
            progressive: "me".into(),
            kana: "め".into(),
            kanji: Some("目".into()),
        },
        Question::Recall {
            english: "ear".into(),
            progressive: "mimi".into(),
            kana: "みみ".into(),
            kanji: Some("耳".into()),
        },
        Question::Recall {
            english: "nose".into(),
            progressive: "hana".into(),
            kana: "はな".into(),
            kanji: Some("鼻".into()),
        },
        Question::Recall {
            english: "face".into(),
            progressive: "kao".into(),
            kana: "かお".into(),
            kanji: Some("顔".into()),
        },
        Question::Recall {
            english: "hand".into(),
            progressive: "te".into(),
            kana: "て".into(),
            kanji: Some("手".into()),
        },
        Question::Recall {
            english: "foot, leg".into(),
            progressive: "ashi".into(),
            kana: "あし".into(),
            kanji: Some("足".into()),
        },
        Question::Recall {
            english: "finger".into(),
            progressive: "yubi".into(),
            kana: "ゆび".into(),
            kanji: Some("指".into()),
        },
        Question::Recall {
            english: "head".into(),
            progressive: "atama".into(),
            kana: "あたま".into(),
            kanji: Some("頭".into()),
        },
        Question::Recall {
            english: "tooth, teeth".into(),
            progressive: "ha".into(),
            kana: "は".into(),
            kanji: Some("歯".into()),
        },
    ]
}