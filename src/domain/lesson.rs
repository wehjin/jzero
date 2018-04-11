use super::*;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub question: Question,
    pub progress: LessonProgress,
}

impl Lesson {
    pub fn new(question: Question) -> Self {
        Lesson { question, progress: LessonProgress::Start }
    }
}
