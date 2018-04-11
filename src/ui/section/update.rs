use super::*;

impl Update<SessionMsg> for SessionMdl {
    fn update(&mut self, msg: SessionMsg) {
        println!("Msg: {:?}\n  Mdl: {:?}", msg, self);
        match msg {
            SessionMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            SessionMsg::ProceedToAnswer => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Learn;
                }
            }
            SessionMsg::ProceedToReview => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Review;
                }
            }
            SessionMsg::HardResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Hard(now));
                self.session.start_next_lesson(now);

                use storage;
                storage::save(&self.session);
            }
            SessionMsg::GoodResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Good(now));
                self.session.start_next_lesson(now);

                use storage;
                storage::save(&self.session);
            }
            SessionMsg::EasyResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Easy(now));
                self.session.start_next_lesson(now);

                use storage;
                storage::save(&self.session);
            }
        }
    }
}
