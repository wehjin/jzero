use std::sync::Arc;
use patchgl::flood::{Flood, Length, Signal, Version, Padding, Position, Placement, Sensor};
use patchgl::material::components::button_bar::ButtonBar;
use patchgl::material::components::stepper::*;
use patchgl::material::Palette;
use super::*;
use domain::{LessonProgress, Question};

impl<'a, MsgT, F> Into<Flood<MsgT>> for SectionViewer<'a, MsgT, F> where
    F: Fn(SectionViewerMsg) -> MsgT + Send + Sync + 'static,
    MsgT: Clone + Send + Sync + 'static,
{
    fn into(self) -> Flood<MsgT> {
        let palette = &Palette::default();
        let section_msg_wrap = Arc::new(self.section_viewer_msg_wrap);
        let section_mdl = self.section_viewer_mdl;
        let flood = if let Some(ref active_lesson) = section_mdl.section.active_lesson {
            let active_content = match active_lesson.progress {
                LessonProgress::Start => draw_start(&section_mdl, palette, active_lesson, section_msg_wrap.clone()),
                LessonProgress::Learn => draw_learn(&section_mdl, palette, active_lesson, section_msg_wrap.clone()),
                LessonProgress::Review => draw_review(&section_mdl, palette, active_lesson, section_msg_wrap.clone()),
            };
            let active_index = match active_lesson.progress {
                LessonProgress::Start => 0,
                LessonProgress::Learn => 1,
                LessonProgress::Review => 2,
            };
            let stepper: Flood<MsgT> = Stepper {
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
            Flood::Text("Lessons are napping".into(), palette.primary, Placement::Center)
                + Padding::Dual(Length::Full * 0.2, Length::Full * 0.45)
                + Flood::Color(palette.light_background)
        };
        flood + Sensor::Signal(Signal {
            id: self.section_viewer_mdl.id,
            version: Version {
                value: self.viewed_lesson_changed_msg.clone(),
                counter: section_mdl.change_version_counter,
            },
        })
    }
}

fn draw_start<MsgT, F>(section_mdl: &SectionViewerMdl, palette: &Palette, active_lesson: &Lesson, section_msg_wrap: Arc<F>) -> Flood<MsgT>
    where
        F: Fn(SectionViewerMsg) -> MsgT + Send + Sync + 'static,
        MsgT: Clone + Send + Sync + 'static,
{
    let title = "Say it aloud".into();
    let button_bar = ButtonBar {
        msg_wrap: {
            let section_msg_wrap = section_msg_wrap.clone();
            move |bar_msg: ButtonBarMsg| {
                let section_msg = SectionViewerMsg::ButtonBarMsg(bar_msg);
                section_msg_wrap(section_msg)
            }
        },
        palette,
        button_bar_mdl: &section_mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Check Answer".into(),
                intent: ButtonIntent::Call,
                click_msg: section_msg_wrap(SectionViewerMsg::ProceedToAnswer),
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

fn draw_learn<MsgT, F>(section_mdl: &SectionViewerMdl, palette: &Palette, active_lesson: &Lesson, section_msg_wrap: Arc<F>) -> Flood<MsgT>
    where
        F: Fn(SectionViewerMsg) -> MsgT + Send + Sync + 'static,
        MsgT: Clone + Send + Sync + 'static,
{
    let Question::Recall { ref english, ref kana, .. } = active_lesson.question;
    let title = format!("{}", english);
    let button_bar = ButtonBar {
        msg_wrap: {
            let section_msg_wrap = section_msg_wrap.clone();
            move |bar_msg: ButtonBarMsg| {
                let section_msg = SectionViewerMsg::ButtonBarMsg(bar_msg);
                section_msg_wrap(section_msg)
            }
        },
        palette,
        button_bar_mdl: &section_mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Hard (Or Wrong)".into(),
                intent: ButtonIntent::Call,
                click_msg: (section_msg_wrap)(SectionViewerMsg::ProceedToReview),
            },
            Button {
                id: 40,
                label: GOT_THIS.into(),
                intent: ButtonIntent::Provide,
                click_msg: (section_msg_wrap)(SectionViewerMsg::GoodResult),
            },
            Button {
                id: 41,
                label: EASY.into(),
                intent: ButtonIntent::Provide,
                click_msg: (section_msg_wrap)(SectionViewerMsg::EasyResult),
            }
        ],
    };
    Flood::Text(kana.to_owned(), palette.primary, Placement::Start)
        + Padding::Uniform(Length::Cross * 0.35)
        + (Position::Top(Length::Spacing * 2), Flood::Text(title, palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

pub fn draw_review<MsgT, F>(section_mdl: &SectionViewerMdl, palette: &Palette, active_lesson: &Lesson, section_msg_wrap: Arc<F>) -> Flood<MsgT>
    where
        F: Fn(SectionViewerMsg) -> MsgT + Send + Sync + 'static,
        MsgT: Clone + Send + Sync + 'static,
{
    let title = "Review".into();
    let button_bar = ButtonBar {
        msg_wrap: {
            let section_msg_wrap = section_msg_wrap.clone();
            move |bar_msg: ButtonBarMsg| {
                let section_msg = SectionViewerMsg::ButtonBarMsg(bar_msg);
                section_msg_wrap(section_msg)
            }
        },
        palette,
        button_bar_mdl: &section_mdl.button_bar_mdl,
        buttons: vec![
            Button {
                id: 38,
                label: "Next Question".into(),
                intent: ButtonIntent::Call,
                click_msg: (section_msg_wrap)(SectionViewerMsg::HardResult),
            },
            Button {
                id: 39,
                label: "Back".into(),
                intent: ButtonIntent::Provide,
                click_msg: (section_msg_wrap)(SectionViewerMsg::ProceedToAnswer),
            },
        ],
    };

    let Question::Recall { ref english, ref kana, .. } = active_lesson.question;
    let prompt = kana.chars().fold(String::new(), |full, _next| {
        format!("{} {}", full, "—")
    });
    Flood::Text(english.to_owned(), palette.light_background_text_primary, Placement::Start)
        + (Position::Bottom(Length::Full * 0.2), Flood::Text("Say it aloud".into(), palette.primary, Placement::Start))
        + (Position::Bottom(Length::Full * 0.3), Flood::Text(prompt, palette.primary, Placement::Start))
        + Padding::Uniform(Length::Cross * 0.25)
        + (Position::Top(Length::Spacing * 2), Flood::Text(title, palette.light_background_text_primary, Placement::Start))
        + Padding::Uniform(Length::Spacing * 3 / 2)
        + (Position::Bottom(Length::Spacing * 3), button_bar.into())
}

const GOT_THIS: &str = "Good (Revisit in 2d)";
const EASY: &str = "Easy (Revisit in 5d)";
