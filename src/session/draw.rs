use patchgl::flood::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::components::stepper::*;
use super::*;

impl Draw<SessionMsg> for SessionMdl {
    fn draw(&self) -> Flood<SessionMsg> {
        let palette = &Palette::default();
        if let Some(ref active_lesson) = self.session.active_lesson {
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
            let stepper: Flood<SessionMsg> = Stepper {
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

pub fn draw_perform(mdl: &SessionMdl, palette: &Palette, active_lesson: &Lesson) -> Flood<SessionMsg> {
    let title = "Say it aloud".into();
    let button_bar = ButtonBar {
        msg_wrap: SessionMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Check Answer".into(),
                intent: ButtonIntent::Call,
                click_msg: SessionMsg::ProceedToAnswer,
            }
        ],
    };
    let Question::Recall { ref english, .. } = active_lesson.question;
    Flood::Text(english.to_owned(), palette.light_background_text_primary, Placement::Start)
        + (Position::Bottom(Length::Full * 0.4), Flood::Text("?".into(), palette.primary, Placement::Start))
        + Padding::Uniform(Length::Cross * 0.25)
        + (Position::Top(Length::Spacing * 2), Flood::Text(title, palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

const GOT_THIS: &str = "Good (Revisit in 2d)";
const EASY: &str = "Easy (Revisit in 1w)";

pub fn draw_acquire(mdl: &SessionMdl, palette: &Palette, active_lesson: &Lesson) -> Flood<SessionMsg> {
    let Question::Recall { ref english, ref kana, .. } = active_lesson.question;
    let title = format!("{}", english);
    let button_bar = ButtonBar {
        msg_wrap: SessionMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Hard or Wrong".into(),
                intent: ButtonIntent::Call,
                click_msg: SessionMsg::ProceedToReview,
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: SessionMsg::GoodResult,
            },
            Button {
                id: 41,
                label: EASY.into(),
                intent: ButtonIntent::Provide,
                click_msg: SessionMsg::EasyResult,
            }
        ],
    };
    Flood::Text(kana.to_owned(), palette.primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.35)
        + (Position::Top(Length::Spacing * 2), Flood::Text(title, palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

pub fn draw_review(mdl: &SessionMdl, palette: &Palette, active_lesson: &Lesson) -> Flood<SessionMsg> {
    let title = "Say it again".into();
    let button_bar = ButtonBar {
        msg_wrap: SessionMsg::ButtonBarMsg,
        palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Next Question".into(),
                intent: ButtonIntent::Call,
                click_msg: SessionMsg::HardResult,
            },
            Button {
                id: 39,
                label: "Back".into(),
                intent: ButtonIntent::Provide,
                click_msg: SessionMsg::ProceedToAnswer,
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
        + (Position::Top(Length::Spacing * 2), Flood::Text(title, palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}
