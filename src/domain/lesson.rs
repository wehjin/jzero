use super::*;

impl Lesson {
    pub fn new(question: Question) -> Self {
        Lesson { question, progress: LessonProgress::Test }
    }
}
