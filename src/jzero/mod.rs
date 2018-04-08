use ::traits::*;
use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;

mod draw;

#[derive(Clone, PartialEq, Debug)]
pub struct Topic {
    pub english: String,
    pub progressive: String,
    pub kana: String,
    pub kanji: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ViewState {
    Perform,
    Acquire,
    Review,
}

#[derive(Clone, PartialEq, Debug)]
pub enum JzeroMsg {
    ButtonBarMsg(ButtonBarMsg),
    ViewAnswer,
    Review,
    RetestSoon,
    RetestLater,
    RetestMuchLater,
}

#[derive(Clone, PartialEq, Debug)]
pub struct JzeroMdl {
    button_bar_mdl: ButtonBarMdl,
    active_topic: Topic,
    view_state: ViewState,
}

impl Mdl<JzeroMsg> for JzeroMdl {}

impl Update<JzeroMsg> for JzeroMdl {
    fn update(&mut self, msg: JzeroMsg) {
        println!("Msg: {:?}\n  Mdl: {:?}", msg, self);
        match msg {
            JzeroMsg::ButtonBarMsg(msg) => update_button_bar(&mut self.button_bar_mdl, msg),
            JzeroMsg::ViewAnswer => {
                self.view_state = ViewState::Acquire;
            }
            JzeroMsg::Review => {
                self.view_state = ViewState::Review;
            }
            JzeroMsg::RetestSoon => {}
            JzeroMsg::RetestLater => {}
            JzeroMsg::RetestMuchLater => {}
        }
    }
}

impl Default for JzeroMdl {
    fn default() -> Self {
        let button_bar_mdl = ButtonBarMdl::default();
        let active_topic = Topic {
            english: "mouth".into(),
            progressive: "kuchi".into(),
            kana: "くち".into(),
            kanji: Some("口".into()),
        };
        let view_state = ViewState::Perform;
        JzeroMdl { button_bar_mdl, active_topic, view_state }
    }
}
