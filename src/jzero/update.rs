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
