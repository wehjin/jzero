extern crate chrono;
extern crate patchgl;

use jzero::JzeroMdl;
use patchgl::app::App;
use traits::*;

mod jzero;
mod traits;

fn main() {
    window::start(768, 768, |window| {
        App::new(JzeroMdl::update, JzeroMdl::draw)
            .run("Jzero", JzeroMdl::default(), window);
    });
    use patchgl::window;
}
