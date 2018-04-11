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
    window::start(1024, 768, |window| {
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
        use patchgl::material::Palette;
        use patchgl::flood::*;

        let palette = &Palette::default();

        let section_viewer: Flood<AppMsg> = SectionViewer {
            msg_wrap: AppMsg::SectionViewerMsgWrap,
            mdl: &self.section_viewer_mdl,
            viewed_lesson_changed_msg: AppMsg::Save,
        }.into();

        let picker_panel = {
            let picker_panel_items = {
                let items = self.course.sections.iter().map(|section| {
                    Flood::Text(section.name.to_owned(), palette.dark_background_text_primary, Placement::Start)
                        + Padding::Uniform(Length::Spacing)
                }).rev().enumerate().collect::<Vec<_>>();

                items.into_iter().fold(Flood::Color(palette.transparent), |panel, (_i, item)| {
                    panel + (Position::Top(Length::Spacing * 4), item)
                })
            };
            picker_panel_items + Flood::Color(palette.dark_background)
        };

        section_viewer + Padding::Behind(Length::CardApproach) + (Position::Left(Length::Full * 0.25), picker_panel)
    }
}

