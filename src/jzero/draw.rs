use patchgl::flood::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::components::stepper::*;
use super::*;

impl Draw<JzeroMsg> for JzeroMdl {
    fn draw(&self) -> Flood<JzeroMsg> {
        let palette = &Palette::default();
        let active_content = match self.view_state {
            ViewState::Perform => draw_perform(self, palette),
            ViewState::Acquire => draw_acquire(self, palette),
            ViewState::Review => draw_review(self, palette),
        };
        let active_index = match self.view_state {
            ViewState::Perform => 0,
            ViewState::Acquire => 1,
            ViewState::Review => 2,
        };
        let stepper: Flood<JzeroMsg> = Stepper {
            palette,
            id: vec![15],
            active_index,
            active_content,
            steps: vec![
                Step { label: "Recall" },
                Step { label: "Remember" },
                Step { label: "Review" },
            ],
        }.into();

        stepper + Padding::Uniform(Length::Spacing) + Flood::Color(palette.light_background)
    }
}

pub fn draw_perform(mdl: &JzeroMdl, palette: &Palette) -> Flood<JzeroMsg> {
    let button_bar = ButtonBar {
        msg_wrap: JzeroMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Reveal".into(),
                intent: ButtonIntent::Call,
                click_msg: JzeroMsg::ViewAnswer,
            }
        ],
    };
    Flood::Text(mdl.active_topic.english.to_owned(), palette.light_background_text_primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.4)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

const GOT_THIS: &str = "Got This (2d)";
const EASY: &str = "Too Easy (1w)";

pub fn draw_acquire(mdl: &JzeroMdl, palette: &Palette) -> Flood<JzeroMsg> {
    let button_bar = ButtonBar {
        msg_wrap: JzeroMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Next".into(),
                intent: ButtonIntent::Call,
                click_msg: JzeroMsg::Review,
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::RetestLater,
            },
            Button {
                id: 41,
                label: EASY.into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::RetestMuchLater,
            }
        ],
    };
    let english = Flood::Text(mdl.active_topic.english.to_owned(), palette.light_background_text_primary, Placement::Start);
    Flood::Text(mdl.active_topic.kana.to_owned(), palette.primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.35)
        + (Position::Top(Length::Spacing * 2), english)
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

pub fn draw_review(mdl: &JzeroMdl, palette: &Palette) -> Flood<JzeroMsg> {
    let button_bar = ButtonBar {
        msg_wrap: JzeroMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Continue".into(),
                intent: ButtonIntent::Call,
                click_msg: JzeroMsg::RetestSoon,
            },
            Button {
                id: 39,
                label: "Back".into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::ViewAnswer,
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::RetestLater,
            },
        ],
    };

    let fillin = mdl.active_topic.kana.chars().fold(String::new(), |full, _next| {
        format!("{} {}", full, "—")
    });

    Flood::Text(mdl.active_topic.english.to_owned(), palette.light_background_text_primary, Placement::Start)
        + (Position::Bottom(Length::Full * 0.3), Flood::Text(fillin, palette.primary, Placement::Start))
        + Padding::Uniform(Length::Cross * 0.25)
        + (Position::Top(Length::Spacing * 2), Flood::Text("Say it aloud".into(), palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}
