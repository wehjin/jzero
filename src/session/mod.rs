use chrono::prelude::*;
use domain::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;
use patchgl::traits::*;

mod draw;
mod update;

#[derive(Clone, PartialEq, Debug)]
pub enum SessionMsg {
    ButtonBarMsg(ButtonBarMsg),
    ProceedToAnswer,
    ProceedToReview,
    HardResult,
    GoodResult,
    EasyResult,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SessionMdl {
    pub button_bar_mdl: ButtonBarMdl,
    pub session: Section,
}

impl Default for SessionMdl {
    fn default() -> Self {
        SessionMdl {
            button_bar_mdl: ButtonBarMdl::default(),
            session: Section::vocabulary_group_a(),
        }
    }
}

impl Mdl<SessionMsg> for SessionMdl {}

