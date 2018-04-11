use super::*;

impl Update<AppMsg> for AppMdl {
    fn update(&mut self, msg: AppMsg) {
        println!("Msg: {:?}\n  Mdl: {:?}", msg, self);
        match msg {
            AppMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            AppMsg::ProceedToAnswer => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Learn;
                }
            }
            AppMsg::ProceedToReview => {
                if let &mut Some(ref mut lesson) = &mut self.session.active_lesson {
                    lesson.progress = LessonProgress::Review;
                }
            }
            AppMsg::HardResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Hard(now));
                self.session.start_next_lesson(now);
                self.save_version_counter.bump();
            }
            AppMsg::GoodResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Good(now));
                self.session.start_next_lesson(now);
                self.save_version_counter.bump();
            }
            AppMsg::EasyResult => {
                let now = Utc::now();
                self.session.finish_active_lesson_with_result(LessonResult::Easy(now));
                self.session.start_next_lesson(now);
                self.save_version_counter.bump();
            }
            AppMsg::Save => {
                use storage;
                storage::save(&self.session);
            }
        }
    }
}
