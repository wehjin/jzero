use chrono::prelude::*;
use domain::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;
use patchgl::traits::*;
use patchgl::flood::VersionCounter;

mod draw;
mod update;

#[derive(Clone, PartialEq, Debug)]
pub struct AppMdl {
    pub id: u64,
    pub session: Section,
    pub button_bar_mdl: ButtonBarMdl,
    pub save_version_counter: VersionCounter,
}

impl Default for AppMdl {
    fn default() -> Self {
        AppMdl {
            id: {
                use rand::{Rng, thread_rng};
                let mut rng = thread_rng();
                rng.gen_range(1, u64::max_value())
            },
            session: Section::vocabulary_group_a(),
            button_bar_mdl: ButtonBarMdl::default(),
            save_version_counter: VersionCounter::enabled_after_bump(),
        }
    }
}

impl Mdl<AppMsg> for AppMdl {}

#[derive(Clone, PartialEq, Debug)]
pub enum AppMsg {
    ButtonBarMsg(ButtonBarMsg),
    ProceedToAnswer,
    ProceedToReview,
    HardResult,
    GoodResult,
    EasyResult,
    Save,
}
