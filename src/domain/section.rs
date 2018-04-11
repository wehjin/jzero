use super::*;

impl Section {
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

impl Default for Section {
    fn default() -> Self {
        Section {
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
