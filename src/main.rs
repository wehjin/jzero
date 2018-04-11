extern crate chrono;
extern crate patchgl;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate time;

use patchgl::app::App;
use patchgl::flood::Flood;
use patchgl::traits::{Draw, Mdl, Update};
use patchgl::window;
use ui::section::{SectionMdl, SectionElement, SectionMsg};

mod storage;
mod domain;
mod ui;

fn main() {
    window::start(768, 768, |window| {
        let mdl = AppMdl { section_mdl: SectionMdl { section: storage::load(), ..Default::default() } };
        App::new(AppMdl::update, AppMdl::draw)
            .run("Jzero", mdl, window);
    });
}


#[derive(Clone, PartialEq, Debug)]
pub struct AppMdl {
    pub section_mdl: SectionMdl,
}

impl Default for AppMdl {
    fn default() -> Self {
        AppMdl { section_mdl: SectionMdl::default() }
    }
}

impl Update<AppMsg> for AppMdl {
    fn update(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::SectionMsgWrap(section_msg) => {
                self.section_mdl.update(section_msg);
            }
            AppMsg::Save => {
                storage::save(&self.section_mdl.section);
            }
        }
    }
}

impl Draw<AppMsg> for AppMdl {
    fn draw(&self) -> Flood<AppMsg> {
        SectionElement {
            section_msg_wrap: AppMsg::SectionMsgWrap,
            section_mdl: &self.section_mdl,
            change_msg: AppMsg::Save,
        }.into()
    }
}

impl Mdl<AppMsg> for AppMdl {}

#[derive(Clone, PartialEq, Debug)]
pub enum AppMsg {
    SectionMsgWrap(SectionMsg),
    Save,
}
