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
    use patchgl::material::components::button_bar::*;
    use patchgl::material::Palette;
    pub use self::update::*;
    pub use self::draw::*;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Mdl {
        button_bar_mdl: ButtonBarMdl,
        palette: Palette,
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

    mod update {
        use super::*;

        pub fn update(mdl: &mut Mdl, msg: Msg) {
            println!("Msg: {:?}", msg);
            match msg {
                Msg::ButtonBarMsg(msg) => update_button_bar(&mut mdl.button_bar_mdl, msg),
                Msg::View => {}
            }
        }

        #[derive(Clone, PartialEq, Debug)]
        pub enum Msg {
            ButtonBarMsg(ButtonBarMsg),
            View,
        }

        impl Default for Mdl {
            fn default() -> Self {
                let palette = Palette::default();
                let button_bar_mdl = ButtonBarMdl::default();
                let card = RecallCard {
                    english: "mouth".into(),
                    progressive: "kuchi".into(),
                    kana: "くち".into(),
                    kanji: Some("口".into()),
                };
                let view_state = ViewState::Performance;
                Mdl { palette, button_bar_mdl, card, view_state }
            }
        }
    }

    mod draw {
        use super::*;
        use patchgl::flood::*;

        pub fn draw(mdl: &Mdl) -> Flood<Msg> {
            let card = match mdl.view_state {
                ViewState::Performance => draw_performance(&mdl)
            };
            card + Flood::Color(mdl.palette.light_background_raised)
        }

        fn draw_performance(mdl: &Mdl) -> Flood<Msg> {
            let button_bar = ButtonBar {
                msg_wrap: Msg::ButtonBarMsg,
                palette: &mdl.palette,
                button_bar_mdl: &mdl.button_bar_mdl,
                buttons: vec![
                    Button {
                        id: 38,
                        label: "View".into(),
                        intent: ButtonIntent::Call,
                        click_msg: Msg::View,
                    }
                ],
            };
            Flood::Text(mdl.card.english.to_owned(), mdl.palette.light_background_text_primary, Placement::Start)
                + Padding::Uniform(Length::Cross * 0.4)
                + (Position::Bottom(Length::Spacing * 3), button_bar.into())
        }
    }
}