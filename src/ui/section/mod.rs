use chrono::prelude::*;
use rand::{Rng, thread_rng};
use patchgl::material::components::button_bar::*;
use patchgl::flood::VersionCounter;
use patchgl::traits::*;
use domain::{LessonProgress, LessonResult, Lesson, Section};

mod draw;

pub struct SectionElement<'a, MsgT, F>
    where F: Fn(SectionMsg) -> MsgT + Send + Sync + 'static,
{
    pub section_msg_wrap: F,
    pub section_mdl: &'a SectionMdl,
    pub change_msg: MsgT,
}


#[derive(Clone, PartialEq, Debug)]
pub struct SectionMdl {
    pub id: u64,
    pub section: Section,
    pub button_bar_mdl: ButtonBarMdl,
    pub change_version_counter: VersionCounter,
}

impl Default for SectionMdl {
    fn default() -> Self {
        SectionMdl {
            id: thread_rng().gen(),
            section: Section::vocabulary_group_a(),
            button_bar_mdl: ButtonBarMdl::default(),
            change_version_counter: VersionCounter::enabled_after_bump(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SectionMsg {
    ButtonBarMsg(ButtonBarMsg),
    ProceedToAnswer,
    ProceedToReview,
    HardResult,
    GoodResult,
    EasyResult,
}

impl Update<SectionMsg> for SectionMdl {
    fn update(&mut self, msg: SectionMsg) {
        match msg {
            SectionMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            SectionMsg::ProceedToAnswer => {
                if let &mut Some(ref mut lesson) = &mut self.section.active_lesson {
                    lesson.progress = LessonProgress::Learn;
                }
            }
            SectionMsg::ProceedToReview => {
                if let &mut Some(ref mut lesson) = &mut self.section.active_lesson {
                    lesson.progress = LessonProgress::Review;
                }
            }
            SectionMsg::HardResult => {
                let now = Utc::now();
                self.section.finish_active_lesson_with_result(LessonResult::Hard(now));
                self.section.start_next_lesson(now);
                self.change_version_counter.bump();
            }
            SectionMsg::GoodResult => {
                let now = Utc::now();
                self.section.finish_active_lesson_with_result(LessonResult::Good(now));
                self.section.start_next_lesson(now);
                self.change_version_counter.bump();
            }
            SectionMsg::EasyResult => {
                let now = Utc::now();
                self.section.finish_active_lesson_with_result(LessonResult::Easy(now));
                self.section.start_next_lesson(now);
                self.change_version_counter.bump();
            }
        }
    }
}
