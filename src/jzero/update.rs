use super::*;

pub fn update(mdl: &mut Mdl, msg: Msg) {
    println!("Msg: {:?}\n  Mdl: {:?}", msg, mdl);
    match msg {
        Msg::ButtonBarMsg(msg) => update_button_bar(&mut mdl.button_bar_mdl, msg),
        Msg::ViewAnswer => {
            mdl.view_state = ViewState::Acquire;
        }
        Msg::Review => {
            mdl.view_state = ViewState::Review;
        }
        Msg::RetestSoon => {}
        Msg::RetestLater => {}
        Msg::RetestMuchLater => {}
    }
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
        let view_state = ViewState::Perform;
        Mdl { palette, button_bar_mdl, card, view_state }
    }
}
