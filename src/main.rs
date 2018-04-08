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
    pub fn update(_mdl: &mut Mdl, _msg: Msg) {}

    #[derive(Clone, PartialEq, Debug)]
    pub struct Mdl {
        card: RecallCard,
        view_state: ViewState,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct RecallCard {
        pub english: String,
        pub progressive: String,
        pub kana: String,
        pub kanji: Option<String>,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum ViewState {
        Performance,
    }

    impl Default for Mdl {
        fn default() -> Self {
            let card = RecallCard {
                english: "mouth".into(),
                progressive: "kuchi".into(),
                kana: "くち".into(),
                kanji: Some("口".into()),
            };
            let view_state = ViewState::Performance;
            Mdl { card, view_state }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Msg {}

    pub fn draw(mdl: &Mdl) -> Flood<Msg> {
        let palette = Palette::default();
        let card = match mdl.view_state {
            ViewState::Performance => draw_performance(&palette, &mdl.card)
        };
        card + Flood::Color(palette.light_background)
    }

    use patchgl::material::Palette;
    use patchgl::flood::*;

    fn draw_performance(palette: &Palette, card: &RecallCard) -> Flood<Msg> {
        Flood::Text(card.english.to_owned(), palette.light_background_text_primary, Placement::Start)
            + Padding::Uniform(Length::Cross * 0.4)
    }
}