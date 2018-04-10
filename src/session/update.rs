use super::*;

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
                self.session.finish_active_lesson_with_result(LessonResult::Hard(now));
                self.session.start_next_lesson(now);

                use storage;
                storage::save(&self.session);
            }
            SessionMsg::GoodResult => {}
            SessionMsg::EasyResult => {}
        }
    }
}
