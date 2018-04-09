use ::traits::*;
use chrono::prelude::*;
use chrono::Duration;
use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;
use std::collections::HashMap;

mod draw;
mod update;

#[derive(Clone, PartialEq, Debug)]
pub enum JzeroMsg {
    ButtonBarMsg(ButtonBarMsg),
    ProceedToAnswer,
    ProceedToReview,
    HardResult,
    GoodResult,
    EasyResult,
}

#[derive(Clone, PartialEq, Debug)]
pub struct JzeroMdl {
    button_bar_mdl: ButtonBarMdl,
    questions: Vec<Question>,
    active_lesson: Option<Lesson>,
    lesson_results: HashMap<Question, LessonResult>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Question {
    Recall { english: String, progressive: String, kana: String, kanji: Option<String> }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LessonResult {
    Hard(DateTime<Utc>),
}

impl LessonResult {
    pub fn time(&self) -> DateTime<Utc> {
        match self {
            &LessonResult::Hard(time) => time.clone()
        }
    }

    pub fn rest_duration(&self) -> Duration {
        match self {
            &LessonResult::Hard(_) => Duration::minutes(0),
        }
    }

    pub fn due_time(&self) -> DateTime<Utc> {
        self.time() + self.rest_duration()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Lesson {
    question: Question,
    progress: LessonProgress,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LessonProgress {
    Perform,
    Acquire,
    Review,
}

impl Default for JzeroMdl {
    fn default() -> Self {
        JzeroMdl {
            button_bar_mdl: ButtonBarMdl::default(),
            questions: vec![
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
            ],
            active_lesson: Some(Lesson {
                question: Question::Recall {
                    english: "mouth".into(),
                    progressive: "kuchi".into(),
                    kana: "くち".into(),
                    kanji: Some("口".into()),
                },
                progress: LessonProgress::Perform,
            }),
            lesson_results: HashMap::new(),
        }
    }
}

impl Mdl<JzeroMsg> for JzeroMdl {}

