use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};
use select_dict::SelectDictScreen;
use typing::TypingScreen;

pub mod select_dict;
pub mod typing;

pub enum Screen {
    Typing,
    Dicts,
}

pub trait Opts {
    fn custom_options() -> Vec<(&'static str, &'static str)>;
}

impl Screen {
    pub fn render_screen<T>(&self, area: Rect, buf: &mut Buffer, screen: &Screen) {}
}
