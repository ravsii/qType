use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use strum::IntoEnumIterator;

use crate::{
    dict::{Dictionary, Language},
    event::Event,
};

use super::Screen;

#[derive(Clone, Debug)]
pub struct SelectDictScreen {
    dict: Rc<RefCell<Dictionary>>,
    languages: Vec<Language>,
    state: ListState,
}

impl SelectDictScreen {
    pub fn new(dict: &Rc<RefCell<Dictionary>>) -> Self {
        Self {
            dict: dict.clone(),
            languages: Language::iter().collect(),
            state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Event {
        match key.code {
            KeyCode::Up => match self.state.selected() {
                Some(0) => self.state.select_last(),
                Some(_) => self.state.select_previous(),
                None => {}
            },
            KeyCode::Down => {
                if let Some(i) = self.state.selected() {
                    if i == self.languages.len() - 1 {
                        self.state.select_first();
                    } else {
                        self.state.select_next()
                    }
                }
            }
            KeyCode::Esc => return Event::Switch(Screen::Typing),
            KeyCode::Enter => {
                self.change_dict();
                return Event::Switch(Screen::Typing);
            }
            _ => {}
        };

        Event::DoNothing
    }

    fn change_dict(&mut self) {
        if let Some(i) = self.state.selected() {
            let lang = self.languages.get(i).unwrap_or(&Language::English);

            // TODO: Handle err
            self.dict.borrow_mut().load(*lang).unwrap();
        }
    }

    pub fn actions() -> Vec<(&'static str, &'static str)> {
        vec![("Esc", "Back"), ("▲ ▼", "Move"), ("Enter", "Select")]
    }
}

impl Widget for &mut SelectDictScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Available dictionaties").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED);

        let list_items = self
            .languages
            .iter()
            .map(|item| ListItem::from(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(list_items)
            .block(block)
            .highlight_symbol("> ")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
