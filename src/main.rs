extern crate chrono;
extern crate patchgl;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate time;

use patchgl::app::App;
use patchgl::traits::*;
use patchgl::window;
use ui::section::SectionMdl;

mod storage;
mod domain;
mod ui;

fn main() {
    window::start(768, 768, |window| {
        let mdl = SectionMdl { session: storage::load(), ..Default::default() };
        App::new(SectionMdl::update, SectionMdl::draw)
            .run("Jzero", mdl, window);
    });
}
