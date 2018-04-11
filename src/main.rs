extern crate chrono;
extern crate patchgl;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate time;

use patchgl::app::App;
use patchgl::flood::Flood;
use patchgl::traits::{Draw, Update};
use patchgl::window;
use ui::section_viewer::{SectionViewerMdl, SectionViewer, SectionViewerMsg};
use domain::Course;

mod storage;
mod domain;
mod ui;

fn main() {
    window::start(768, 768, |window| {
        let app_mdl = AppMdl {
            course: storage::load_course(),
            section_viewer_mdl: SectionViewerMdl::default(),
        };
        App::new(AppMdl::update, AppMdl::draw)
            .run("Jzero", app_mdl, window);
    });
}


#[derive(Clone, PartialEq, Debug)]
pub struct AppMdl {
    pub course: Course,
    pub section_viewer_mdl: SectionViewerMdl,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AppMsg {
    SectionViewerMsgWrap(SectionViewerMsg),
    Save,
}

impl Update<AppMsg> for AppMdl {
    fn update(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::SectionViewerMsgWrap(section_msg) => {
                self.section_viewer_mdl.update(section_msg);
            }
            AppMsg::Save => {
                if let Some(ref section) = self.section_viewer_mdl.section {
                    storage::save(section);
                }
            }
        }
    }
}

impl Draw<AppMsg> for AppMdl {
    fn draw(&self) -> Flood<AppMsg> {
        SectionViewer {
            msg_wrap: AppMsg::SectionViewerMsgWrap,
            mdl: &self.section_viewer_mdl,
            viewed_lesson_changed_msg: AppMsg::Save,
        }.into()
    }
}

