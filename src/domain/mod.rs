use chrono::prelude::{DateTime, Utc};
use std::collections::HashMap;

mod lesson_result;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Session {
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

impl Lesson {
    pub fn new(question: Question) -> Self {
        Lesson { question, progress: LessonProgress::Test }
    }
}

impl Session {
    pub fn finish_active_lesson_with_result(&mut self, result: LessonResult) {
        if let Some(lesson) = self.active_lesson.clone() {
            let results = &mut self.lesson_results;
            results.insert(lesson.question, result);
        }
    }

    pub fn start_next_lesson(&mut self, now: DateTime<Utc>) {
        let candidates = self.find_active_questions(now);
        let next_lesson = if candidates.is_empty() {
            match self.active_lesson {
                Some(ref lesson) if self.wake_time_of_question(&lesson.question) <= now => Some(Lesson::new(lesson.question.clone())),
                _ => None,
            }
        } else {
            use rand::{Rng, thread_rng};
            let mut rng = thread_rng();
            let index = rng.gen_range(0, candidates.len());
            let question = candidates[index].clone();
            Some(Lesson { question, progress: LessonProgress::Test })
        };
        self.active_lesson = next_lesson;
    }

    fn find_active_questions(&self, time: DateTime<Utc>) -> Vec<Question> {
        let mut candidates = self.questions.iter()
            .filter(|question| time >= self.wake_time_of_question(question))
            .map(|it| it.clone())
            .collect::<Vec<_>>();

        if let Some(ref active_lesson) = self.active_lesson {
            if let Some(index) = candidates.iter().position(|it| it == &active_lesson.question) {
                candidates.remove(index);
            }
        }

        candidates.sort_by_key(|it| self.wake_time_of_question(it));
        candidates
    }

    fn wake_time_of_question(&self, question: &Question) -> DateTime<Utc> {
        use time::Duration;
        match self.lesson_results.get(question) {
            Some(ref lesson_result) => lesson_result.due_time(),
            None => Utc::now() - Duration::days(100),
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Session {
            questions: default_questions(),
            active_lesson: Some(Lesson {
                question: Question::Recall {
                    english: "mouth".into(),
                    progressive: "kuchi".into(),
                    kana: "くち".into(),
                    kanji: Some("口".into()),
                },
                progress: LessonProgress::Test,
            }),
            lesson_results: HashMap::new(),
        }
    }
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