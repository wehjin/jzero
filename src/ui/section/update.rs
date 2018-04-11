use super::*;

impl Update<SectionMsg> for SectionMdl {
    fn update(&mut self, msg: SectionMsg) {
        println!("Msg: {:?}\n  Mdl: {:?}", msg, self);
        match msg {
            SectionMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            SectionMsg::ProceedToAnswer => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Learn;
                }
            }
            SectionMsg::ProceedToReview => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Review;
                }
            }
            SectionMsg::HardResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Hard(now));
                self.session.start_next_lesson(now);

                use storage;
                storage::save(&self.session);
            }
            SectionMsg::GoodResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Good(now));
                self.session.start_next_lesson(now);

                use storage;
                storage::save(&self.session);
            }
            SectionMsg::EasyResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Easy(now));
                self.session.start_next_lesson(now);

                use storage;
                storage::save(&self.session);
            }
        }
    }
}
