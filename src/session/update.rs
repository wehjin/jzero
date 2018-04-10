use rand::{Rng, thread_rng};
use super::*;
use time::Duration;

impl Update<SessionMsg> for SessionMdl {
    fn update(&mut self, msg: SessionMsg) {
        println!("Msg: {:?}\n  Mdl: {:?}", msg, self);
        match msg {
            SessionMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            SessionMsg::ProceedToAnswer => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Acquire;
                }
            }
            SessionMsg::ProceedToReview => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Review;
                }
            }
            SessionMsg::HardResult => {
                let now = Utc::now();
                self.record_lesson_result_for_active_lesson(LessonResult::Hard(now));
                self.update_active_lesson(now);

                use storage;
                storage::save(&self.session);
            }
            SessionMsg::GoodResult => {}
            SessionMsg::EasyResult => {}
        }
    }
}


impl SessionMdl {
    fn due_time(&self, question: &Question) -> DateTime<Utc> {
        match self.session.lesson_results.get(question) {
            Some(ref result) => result.due_time(),
            None => Utc::now() - Duration::days(1),
        }
    }

    fn record_lesson_result_for_active_lesson(&mut self, result: LessonResult) {
        if let Some(ref lesson) = self.session.active_lesson {
            let question = lesson.question.clone();
            let results = &mut self.session.lesson_results;
            results.insert(question, result);
        }
    }

    fn find_active_questions(&self, now: DateTime<Utc>) -> Vec<Question> {
        let mut candidates = self.session.questions.iter()
            .filter(|question| now >= self.due_time(question))
            .map(|it| it.clone())
            .collect::<Vec<_>>();

        if let Some(ref active_lesson) = self.session.active_lesson {
            if let Some(index) = candidates.iter().position(|it| it == &active_lesson.question) {
                candidates.remove(index);
            }
        }

        candidates.sort_by_key(|it| self.due_time(it));
        candidates
    }

    fn update_active_lesson(&mut self, now: DateTime<Utc>) {
        let candidates = self.find_active_questions(now);
        let next_lesson = if candidates.is_empty() {
            self.session.active_lesson.clone()
        } else {
            let mut rng = thread_rng();
            let index = rng.gen_range(0, candidates.len());
            let question = candidates[index].clone();
            Some(Lesson { question, progress: LessonProgress::Perform })
        };
        let session = &mut self.session;
        session.active_lesson = next_lesson;
    }
}
