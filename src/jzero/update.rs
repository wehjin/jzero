use super::*;
use rand::{Rng, thread_rng};

impl Update<JzeroMsg> for JzeroMdl {
    fn update(&mut self, msg: JzeroMsg) {
        println!("Msg: {:?}\n  Mdl: {:?}", msg, self);
        match msg {
            JzeroMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            JzeroMsg::ProceedToAnswer => {
                if let &mut Some(ref mut lesson) = &mut self.active_lesson {
                    lesson.progress = LessonProgress::Acquire;
                }
            }
            JzeroMsg::ProceedToReview => {
                if let &mut Some(ref mut lesson) = &mut self.active_lesson {
                    lesson.progress = LessonProgress::Review;
                }
            }
            JzeroMsg::HardResult => {
                let now = Utc::now();
                self.save_active_lesson_result(LessonResult::Hard(now));

                let mut candidates = self.questions.iter()
                    .filter(|question| now >= self.due_time(question))
                    .collect::<Vec<_>>();

                if let Some(ref active_lesson) = self.active_lesson {
                    if let Some(index) = candidates.iter().position(|it| it.clone() == &active_lesson.question) {
                        candidates.remove(index);
                    }
                }

                candidates.sort_by_key(|it| self.due_time(it));

                let next_lesson = if candidates.is_empty() {
                    self.active_lesson.clone()
                } else {
                    let mut rng = thread_rng();
                    let index = rng.gen_range(0, candidates.len());
                    let question = candidates[index].clone();
                    Some(Lesson { question, progress: LessonProgress::Perform })
                };

                self.active_lesson = next_lesson;
            }
            JzeroMsg::GoodResult => {}
            JzeroMsg::EasyResult => {}
        }
    }
}

impl JzeroMdl {
    fn due_time(&self, question: &Question) -> DateTime<Utc> {
        match self.lesson_results.get(question) {
            Some(ref result) => result.due_time(),
            None => Utc::now() - Duration::days(1),
        }
    }

    fn save_active_lesson_result(&mut self, result: LessonResult) {
        if let Some(ref lesson) = self.active_lesson {
            let question = lesson.question.clone();
            let results = &mut self.lesson_results;
            results.insert(question, result);
        }
    }
}
