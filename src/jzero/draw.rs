use super::*;
use patchgl::flood::*;
use patchgl::material::components::stepper::*;

pub fn draw(mdl: &Mdl) -> Flood<Msg> {
    let active_content = match mdl.view_state {
        ViewState::Perform => draw_perform(&mdl),
        ViewState::Acquire => draw_acquire(&mdl),
    };
    let active_index = match mdl.view_state {
        ViewState::Perform => 0,
        ViewState::Acquire => 1,
    };
    let stepper: Flood<Msg> = Stepper {
        palette: &mdl.palette,
        id: vec![15],
        active_index,
        active_content,
        steps: vec![
            Step { label: "Recall" },
            Step { label: "Remember" },
            Step { label: "Review" },
        ],
    }.into();

    stepper + Padding::Uniform(Length::Spacing) + Flood::Color(mdl.palette.light_background)
}

fn draw_perform(mdl: &Mdl) -> Flood<Msg> {
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

fn draw_acquire(mdl: &Mdl) -> Flood<Msg> {
    let button_bar = ButtonBar {
        msg_wrap: Msg::ButtonBarMsg,
        palette: &mdl.palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Review".into(),
                intent: ButtonIntent::Call,
                click_msg: Msg::Review,
            }
        ],
    };
    let english = Flood::Text(mdl.card.english.to_owned(), mdl.palette.light_background_text_primary, Placement::Start);
    Flood::Text(mdl.card.kana.to_owned(), mdl.palette.primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.35)
        + (Position::Top(Length::Spacing * 2), english)
        + Padding::Uniform(Length::Spacing * 3/2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}
