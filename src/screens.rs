use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub mod select_dict;
pub mod typing;

pub trait Screen {
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn handle_key(&mut self, key: KeyEvent) -> bool;
    fn custom_options(&self) -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}
