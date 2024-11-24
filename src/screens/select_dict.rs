use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};
use strum::IntoEnumIterator;

use crate::dict::{Dictionary, Language};

use super::Screen;

#[derive(Clone, Copy, Debug)]
enum UserInput {
    Pass,
    Miss(char),
    Hit(char),
}

#[derive(Clone, Debug)]
pub struct SelectDictScreen {
    dict: &'static Dictionary,
    items: Vec<Language>,
    state: ListState,
}

impl SelectDictScreen {
    pub fn new(dict: &'static Dictionary) -> Self {
        Self {
            dict,
            items: Language::iter().collect(),
            state: ListState::default(),
        }
    }
}

impl Screen for SelectDictScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let list_items = self
            .items
            .iter()
            .map(|item| ListItem::from(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(list_items).highlight_symbol(">");

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Esc => return true,
            _ => {}
        };

        false
    }

    fn custom_options(&self) -> Vec<(&'static str, &'static str)> {
        vec![("Esc", "Back"), ("▲ ▼", "Move"), ("Enter", "Select")]
    }
}
