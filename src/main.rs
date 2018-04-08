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

mod jzero {
    pub fn update(mdl: &mut Mdl, msg: Msg) {}

    #[derive(Clone, PartialEq, Debug)]
    pub struct Mdl {}

    impl Default for Mdl {
        fn default() -> Self {
            Mdl {}
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Msg {}

    pub fn draw(model: &Mdl) -> Flood<Msg> {
        let palette = Palette::default();
        use patchgl::material::Palette;
        Flood::Color(palette.light_background)
    }

    use patchgl::flood::*;
}