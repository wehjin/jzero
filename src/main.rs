extern crate rand;
extern crate chrono;
extern crate patchgl;

use session::SessionMdl;
use patchgl::app::App;
use traits::*;
use patchgl::window;

mod session;
mod traits;

fn main() {
    window::start(768, 768, |window| {
        App::new(SessionMdl::update, SessionMdl::draw)
            .run("Jzero", SessionMdl::default(), window);
    });
}
