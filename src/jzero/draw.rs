use patchgl::flood::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::components::stepper::*;
use super::*;

impl Draw<JzeroMsg> for JzeroMdl {
    fn draw(&self) -> Flood<JzeroMsg> {
        let palette = &Palette::default();
        if let Some(ref active_lesson) = self.active_lesson {
            let active_content = match active_lesson.progress {
                LessonProgress::Perform => draw_perform(self, palette, active_lesson),
                LessonProgress::Acquire => draw_acquire(self, palette, active_lesson),
                LessonProgress::Review => draw_review(self, palette, active_lesson),
            };
            let active_index = match active_lesson.progress {
                LessonProgress::Perform => 0,
                LessonProgress::Acquire => 1,
                LessonProgress::Review => 2,
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
        } else {
            Flood::Text("Take a break".into(), palette.primary, Placement::Center)
                + Padding::Uniform(Length::Cross * 0.4)
        }
    }
}

pub fn draw_perform(mdl: &JzeroMdl, palette: &Palette, active_lesson: &Lesson) -> Flood<JzeroMsg> {
    let button_bar = ButtonBar {
        msg_wrap: JzeroMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Reveal".into(),
                intent: ButtonIntent::Call,
                click_msg: JzeroMsg::ProceedToAnswer,
            }
        ],
    };
    let Question::Recall { ref english, .. } = active_lesson.question;
    Flood::Text(english.to_owned(), palette.light_background_text_primary, Placement::Start)
        + (Position::Bottom(Length::Full * 0.4), Flood::Text("?".into(), palette.primary, Placement::Start))
        + Padding::Uniform(Length::Cross * 0.25)
        + (Position::Top(Length::Spacing * 2), Flood::Text("Say it aloud".into(), palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

const GOT_THIS: &str = "Good (Revisit in 2d)";
const EASY: &str = "Easy (Revisit in 1w)";

pub fn draw_acquire(mdl: &JzeroMdl, palette: &Palette, active_lesson: &Lesson) -> Flood<JzeroMsg> {
    let button_bar = ButtonBar {
        msg_wrap: JzeroMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Review".into(),
                intent: ButtonIntent::Call,
                click_msg: JzeroMsg::ProceedToReview,
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::GoodResult,
            },
            Button {
                id: 41,
                label: EASY.into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::EasyResult,
            }
        ],
    };
    let Question::Recall { ref english, ref kana, .. } = active_lesson.question;
    let english = Flood::Text(english.to_owned(), palette.light_background_text_primary, Placement::Start);
    Flood::Text(kana.to_owned(), palette.primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.35)
        + (Position::Top(Length::Spacing * 2), english)
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

pub fn draw_review(mdl: &JzeroMdl, palette: &Palette, active_lesson: &Lesson) -> Flood<JzeroMsg> {
    let button_bar = ButtonBar {
        msg_wrap: JzeroMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Hard (Repeat)".into(),
                intent: ButtonIntent::Call,
                click_msg: JzeroMsg::HardResult,
            },
            Button {
                id: 39,
                label: "Back".into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::ProceedToAnswer,
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: JzeroMsg::GoodResult,
            },
        ],
    };

    let Question::Recall { ref english, ref kana, .. } = active_lesson.question;
    let prompt = kana.chars().fold(String::new(), |full, _next| {
        format!("{} {}", full, "—")
    });

    Flood::Text(english.to_owned(), palette.light_background_text_primary, Placement::Start)
        + (Position::Bottom(Length::Full * 0.3), Flood::Text(prompt, palette.primary, Placement::Start))
        + Padding::Uniform(Length::Cross * 0.25)
        + (Position::Top(Length::Spacing * 2), Flood::Text("Say it aloud".into(), palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}
