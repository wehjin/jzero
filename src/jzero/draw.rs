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
