use super::*;
use patchgl::flood::*;
use patchgl::material::components::stepper::*;

pub fn draw(mdl: &Mdl) -> Flood<Msg> {
    let active_content = match mdl.view_state {
        ViewState::Perform => draw_perform(&mdl),
        ViewState::Acquire => draw_acquire(&mdl),
        ViewState::Review => draw_review(&mdl),
    };
    let active_index = match mdl.view_state {
        ViewState::Perform => 0,
        ViewState::Acquire => 1,
        ViewState::Review => 2,
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
                label: "Show Answer".into(),
                intent: ButtonIntent::Call,
                click_msg: Msg::ViewAnswer,
            }
        ],
    };
    Flood::Text(mdl.card.english.to_owned(), mdl.palette.light_background_text_primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.4)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

const GOT_THIS: &str = "Got This (2d)";
const EASY: &str = "Too Easy (1w)";

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
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: Msg::RetestLater,
            },
            Button {
                id: 41,
                label: EASY.into(),
                intent: ButtonIntent::Provide,
                click_msg: Msg::RetestMuchLater,
            }
        ],
    };
    let english = Flood::Text(mdl.card.english.to_owned(), mdl.palette.light_background_text_primary, Placement::Start);
    Flood::Text(mdl.card.kana.to_owned(), mdl.palette.primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.35)
        + (Position::Top(Length::Spacing * 2), english)
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

fn draw_review(mdl: &Mdl) -> Flood<Msg> {
    let button_bar = ButtonBar {
        msg_wrap: Msg::ButtonBarMsg,
        palette: &mdl.palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Repeat (10m)".into(),
                intent: ButtonIntent::Call,
                click_msg: Msg::RetestSoon,
            },
            Button {
                id: 39,
                label: "Back".into(),
                intent: ButtonIntent::Provide,
                click_msg: Msg::ViewAnswer,
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: Msg::RetestLater,
            },
        ],
    };

    let fillin = mdl.card.kana.chars().fold(String::new(), |full, _next| {
        format!("{} {}", full, "—")
    });

    Flood::Text(mdl.card.english.to_owned(), mdl.palette.light_background_text_primary, Placement::Start)
        + (Position::Bottom(Length::Full * 0.3), Flood::Text(fillin, mdl.palette.primary, Placement::Start))
        + Padding::Uniform(Length::Cross * 0.25)
        + (Position::Top(Length::Spacing * 2), Flood::Text("Say it aloud".into(), mdl.palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}
