extern crate patchgl;

fn main() {
    window::start(640, 400, |window| {
        use patchgl::app::App;
        use self::jzero::*;

        App::new(update, draw)
            .run("Jzero", Mdl::default(), window);
    });
    use patchgl::window;
}

mod jzero;