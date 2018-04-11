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
    pub msg_wrap: F,
    pub mdl: &'a SectionViewerMdl,
    pub viewed_lesson_changed_msg: MsgT,
}


#[derive(Clone, PartialEq, Debug)]
pub struct SectionViewerMdl {
    pub id: u64,
    pub section: Option<Section>,
    pub button_bar_mdl: ButtonBarMdl,
    pub change_version_counter: VersionCounter,
}

impl Default for SectionViewerMdl {
    fn default() -> Self {
        SectionViewerMdl {
            id: thread_rng().gen(),
            section: None,
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
                if let Some(Section { active_lesson: Some(ref mut lesson), .. }) = self.section {
                    lesson.progress = LessonProgress::Learn;
                }
            }
            SectionViewerMsg::ProceedToReview => {
                if let Some(Section { active_lesson: Some(ref mut lesson), .. }) = self.section {
                    lesson.progress = LessonProgress::Review;
                }
            }
            SectionViewerMsg::HardResult => {
                self.finsh_and_restart_active_lesson(LessonResult::Hard(Utc::now()))
            }
            SectionViewerMsg::GoodResult => {
                self.finsh_and_restart_active_lesson(LessonResult::Good(Utc::now()))
            }
            SectionViewerMsg::EasyResult => {
                self.finsh_and_restart_active_lesson(LessonResult::Easy(Utc::now()))
            }
        }
    }
}

impl SectionViewerMdl {
    fn finsh_and_restart_active_lesson(&mut self, result: LessonResult) -> () {
        if let Some(ref mut section) = self.section {
            section.finish_active_lesson_with_result(result);
            section.start_next_lesson(Utc::now());
            self.change_version_counter.bump();
        }
    }
}
