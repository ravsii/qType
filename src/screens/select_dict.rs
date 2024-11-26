use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};
use strum::IntoEnumIterator;

use crate::dict::{Dictionary, Language};

use super::{Opts, Screen};

#[derive(Clone, Copy, Debug)]
enum UserInput {
    Pass,
    Miss(char),
    Hit(char),
}

#[derive(Clone, Debug)]
pub struct SelectDictScreen {
    dict: Rc<RefCell<Dictionary>>,
    items: Vec<Language>,
    state: ListState,
}

impl SelectDictScreen {
    pub fn new(dict: &Rc<RefCell<Dictionary>>) -> Self {
        Self {
            dict: dict.clone(),
            items: Language::iter().collect(),
            state: ListState::default(),
        }
    }
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Esc => return true,
            _ => {}
        };

        false
    }
}

impl Widget for &mut SelectDictScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list_items = self
            .items
            .iter()
            .map(|item| ListItem::from(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(list_items).highlight_symbol(">");

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl Opts for SelectDictScreen {
    fn custom_options() -> Vec<(&'static str, &'static str)> {
        vec![("Esc", "Back"), ("▲ ▼", "Move"), ("Enter", "Select")]
    }
}
