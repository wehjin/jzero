use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;
pub use self::update::*;
pub use self::draw::*;

#[derive(Clone, PartialEq, Debug)]
pub struct RecallCard {
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
pub struct Mdl {
    button_bar_mdl: ButtonBarMdl,
    palette: Palette,
    card: RecallCard,
    view_state: ViewState,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Msg {
    ButtonBarMsg(ButtonBarMsg),
    ViewAnswer,
    Review,
    RetestSoon,
    RetestLater,
    RetestMuchLater,
}

mod update;
mod draw;
