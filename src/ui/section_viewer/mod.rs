use chrono::prelude::*;
use rand::{Rng, thread_rng};
use patchgl::material::components::button_bar::*;
use patchgl::flood::VersionCounter;
use patchgl::traits::*;
use domain::{LessonProgress, LessonResult, Lesson, Section};

mod draw;

pub struct SectionViewer<'a, MsgT, F>
    where F: Fn(SectionViewerMsg) -> MsgT + Send + Sync + 'static,
{
    pub section_viewer_msg_wrap: F,
    pub section_viewer_mdl: &'a SectionViewerMdl,
    pub viewed_lesson_changed_msg: MsgT,
}


#[derive(Clone, PartialEq, Debug)]
pub struct SectionViewerMdl {
    pub id: u64,
    pub section: Section,
    pub button_bar_mdl: ButtonBarMdl,
    pub change_version_counter: VersionCounter,
}

impl Default for SectionViewerMdl {
    fn default() -> Self {
        SectionViewerMdl {
            id: thread_rng().gen(),
            section: Section::vocabulary_group_a(),
            button_bar_mdl: ButtonBarMdl::default(),
            change_version_counter: VersionCounter::enabled_after_bump(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SectionViewerMsg {
    ButtonBarMsg(ButtonBarMsg),
    ProceedToAnswer,
    ProceedToReview,
    HardResult,
    GoodResult,
    EasyResult,
}

impl Update<SectionViewerMsg> for SectionViewerMdl {
    fn update(&mut self, msg: SectionViewerMsg) {
        match msg {
            SectionViewerMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            SectionViewerMsg::ProceedToAnswer => {
                if let &mut Some(ref mut lesson) = &mut self.section.active_lesson {
                    lesson.progress = LessonProgress::Learn;
                }
            }
            SectionViewerMsg::ProceedToReview => {
                if let &mut Some(ref mut lesson) = &mut self.section.active_lesson {
                    lesson.progress = LessonProgress::Review;
                }
            }
            SectionViewerMsg::HardResult => {
                let now = Utc::now();
                self.section.finish_active_lesson_with_result(LessonResult::Hard(now));
                self.section.start_next_lesson(now);
                self.change_version_counter.bump();
            }
            SectionViewerMsg::GoodResult => {
                let now = Utc::now();
                self.section.finish_active_lesson_with_result(LessonResult::Good(now));
                self.section.start_next_lesson(now);
                self.change_version_counter.bump();
            }
            SectionViewerMsg::EasyResult => {
                let now = Utc::now();
                self.section.finish_active_lesson_with_result(LessonResult::Easy(now));
                self.section.start_next_lesson(now);
                self.change_version_counter.bump();
            }
        }
    }
}
