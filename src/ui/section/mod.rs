use chrono::prelude::*;
use domain::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;
use patchgl::traits::*;

mod draw;
mod update;

#[derive(Clone, PartialEq, Debug)]
pub struct SectionMdl {
    pub button_bar_mdl: ButtonBarMdl,
    pub session: Section,
}

impl Default for SectionMdl {
    fn default() -> Self {
        SectionMdl {
            button_bar_mdl: ButtonBarMdl::default(),
            session: Section::vocabulary_group_a(),
        }
    }
}

impl Mdl<SectionMsg> for SectionMdl {}

#[derive(Clone, PartialEq, Debug)]
pub enum SectionMsg {
    ButtonBarMsg(ButtonBarMsg),
    ProceedToAnswer,
    ProceedToReview,
    HardResult,
    GoodResult,
    EasyResult,
}
