use patchgl::material::components::button_bar::*;
use patchgl::material::Palette;
pub use self::update::*;
pub use self::draw::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Mdl {
    button_bar_mdl: ButtonBarMdl,
    palette: Palette,
    card: RecallCard,
    view_state: ViewState,
}


#[derive(Clone, PartialEq, Debug)]
pub struct RecallCard {
    pub english: String,
    pub progressive: String,
    pub kana: String,
    pub kanji: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ViewState {
    Performance,
}

mod update;
mod draw;
