use ::domain::*;
use ::traits::*;
use chrono::prelude::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;

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
    button_bar_mdl: ButtonBarMdl,
    session: Session,
}

impl Default for SessionMdl {
    fn default() -> Self {
        SessionMdl {
            button_bar_mdl: ButtonBarMdl::default(),
            session: Session::default(),
        }
    }
}

impl Mdl<SessionMsg> for SessionMdl {}

