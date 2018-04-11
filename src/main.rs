extern crate chrono;
extern crate patchgl;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate time;

use std::collections::HashMap;
use patchgl::app::App;
use patchgl::flood::Flood;
use patchgl::traits::{Draw, Update};
use patchgl::window;
use patchgl::button;
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
            button_mdls: HashMap::new(),
        };
        App::new(AppMdl::update, AppMdl::draw)
            .run("Jzero", app_mdl, window);
    });
}


#[derive(Clone, PartialEq, Debug)]
pub struct AppMdl {
    pub course: Course,
    pub section_viewer_mdl: SectionViewerMdl,
    pub button_mdls: HashMap<u64, button::Model>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AppMsg {
    SectionViewerMsgWrap(SectionViewerMsg),
    Save,
    ButtonMsgWrap(u64, button::Msg),
    PickSection(usize),
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
            AppMsg::ButtonMsgWrap(id, button_msg) => {
                let button_mdls = &mut self.button_mdls;
                if button_mdls.contains_key(&id) {
                    let button_mdl = button_mdls.get_mut(&id).unwrap();
                    button::update(button_mdl, button_msg);
                } else {
                    let mut button_mdl = button::Model::default();
                    button::update(&mut button_mdl, button_msg);
                    button_mdls.insert(id, button_mdl);
                }
            }
            AppMsg::PickSection(index) => {
                if let Some(ref section) = self.section_viewer_mdl.section {
                    if let Some(index) = self.course.active_section {
                        self.course.sections[index] = section.clone();
                    }
                }
                self.course.active_section = Some(index);
                self.section_viewer_mdl.update(SectionViewerMsg::SetSection(self.course.sections[index].clone()));
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

        let seed_id: u64 = 50;
        let picker_panel = {
            let picker_panel_items = {
                let items = self.course.sections.iter().enumerate()
                    .map(|(i, section)| {
                        let id = seed_id + i as u64;
                        let button = button::Button {
                            id,
                            kind: button::Kind::PlainFlat(section.name.to_owned()),
                            model: self.button_mdls.get(&id)
                                .map(|it| { it.clone() })
                                .unwrap_or(button::Model::default()),
                            click_msg: AppMsg::PickSection(i),
                            palette,
                        };
                        button::flood(move |button_msg| AppMsg::ButtonMsgWrap(id, button_msg), button)
                    })
                    .rev()
                    .collect::<Vec<_>>();
                items.into_iter().fold(Flood::Color(palette.transparent), |panel, item| {
                    panel + (Position::Top(Length::Spacing * 4), item)
                })
            };

            use patchgl::Color;
            picker_panel_items + Flood::Color(Color::custom_white(0.4))
        };

        section_viewer + Padding::Behind(Length::CardApproach) + (Position::Left(Length::Full * 0.25), picker_panel)
    }
}

