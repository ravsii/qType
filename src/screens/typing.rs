use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use layout::Flex;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use strum::{EnumIter, IntoEnumIterator};

use crate::{dict::Dictionary, event::Event, wpm::WpmCounter};

use super::Screen;

#[derive(Clone, Copy, Debug, Default)]
enum UserInput {
    #[default]
    Pass,
    Miss(char),
    Hit(char),
}

#[derive(Debug, Default, EnumIter, Clone, Copy)]
enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
    VeryHard,
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::VeryHard => "Very hard",
        }
    }
}

#[derive(Debug, Default)]
pub struct TypingScreen {
    current_str: String,
    next_str: String,
    user_string: Vec<char>,
    current_pos: i32,
    // last holds user's last input to apply changes on redraw.
    last: UserInput,

    show_diff_popup: bool,
    curr_diff: Difficulty,
    difficulties: Vec<Difficulty>,
    diff_state: ListState,

    dict: Rc<RefCell<Dictionary>>,
    wpm: WpmCounter,
}

impl TypingScreen {
    pub fn new(dict: &Rc<RefCell<Dictionary>>) -> Self {
        let mut typing_screen = Self {
            dict: dict.clone(),
            difficulties: Difficulty::iter().collect(),
            diff_state: ListState::default().with_selected(Some(0)),
            ..Default::default()
        };

        typing_screen.randomize_input();
        typing_screen
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Event {
        match self.show_diff_popup {
            true => self.handle_popup_key(key),
            false => self.handle_input_key(key),
        }
    }

    fn handle_popup_key(&mut self, key: KeyEvent) -> Event {
        match key.code {
            KeyCode::Up => match self.diff_state.selected() {
                Some(0) => self.diff_state.select_last(),
                Some(_) => self.diff_state.select_previous(),
                None => {}
            },
            KeyCode::Down => {
                if let Some(i) = self.diff_state.selected() {
                    if i == self.difficulties.len() - 1 {
                        self.diff_state.select_first();
                    } else {
                        self.diff_state.select_next()
                    }
                }
            }
            KeyCode::Esc => return Event::Switch(Screen::Typing),
            KeyCode::Enter => {
                self.curr_diff = match self.difficulties.get(self.diff_state.selected().unwrap()) {
                    Some(d) => *d,
                    None => Difficulty::Easy,
                };
                self.show_diff_popup = false;
                self.reset_input();
            }
            _ => {}
        };

        Event::DoNothing
    }

    fn handle_input_key(&mut self, key: KeyEvent) -> Event {
        if self.current_pos == 1 {
            self.wpm.start();
        }

        match key.code {
            KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
                return Event::Switch(Screen::Dicts);
            }
            KeyCode::Char('f') if key.modifiers == KeyModifiers::CONTROL => {
                self.show_diff_popup = true
            }
            KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
                self.randomize_input();
                self.last = UserInput::Pass;
                return Event::DoNothing;
            }
            KeyCode::Char(c) => {
                let next_char = self.current_str.chars().nth(self.current_pos as usize);
                match next_char {
                    Some(nc) if nc == c => self.last = UserInput::Hit(c),
                    _ => self.last = UserInput::Miss(c),
                };
            }
            _ => self.last = UserInput::Pass,
        }

        if let UserInput::Hit(c) = self.last {
            self.user_string.push(c);
            self.current_pos += 1;
            self.wpm.tick_char(&c);
        }

        // we respect utf-8
        if self.current_pos as usize >= self.current_str.chars().count() {
            self.randomize_input();
        }

        Event::DoNothing
    }

    pub fn randomize_input(&mut self) {
        let batch_size = self.curr_diff.batch_size();
        let amount = 5;

        let mut exclude = vec![];
        self.current_str = if self.current_str.is_empty() {
            exclude = self.dict.borrow().random_words(batch_size, amount);
            exclude.join(" ")
        } else {
            self.next_str.to_owned()
        };

        self.next_str = self
            .dict
            .borrow()
            .random_words_exclude(batch_size, amount, &exclude)
            .join(" ");
        self.user_string.clear();
        self.current_pos = 0;
    }

    pub fn reset_input(&mut self) {
        self.current_str = "".to_owned();
        self.next_str = "".to_owned();
        self.randomize_input();
    }

    pub fn actions() -> Vec<(&'static str, &'static str)> {
        vec![
            ("<C-r>", "New random words"),
            ("<C-d>", "Select Dict"),
            ("<C-f>", "Difficulty"),
        ]
    }
}

impl Widget for &mut TypingScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

        let [wpm_area, words_area, input_area, miss_area] = layout.areas(area);

        if self.wpm.is_started() {
            let current = self.wpm.current_wpm();
            Paragraph::new(format!(
                "{:.1} cpm | {:.1} wpm",
                current.chars_per_min, current.words_per_min
            ))
            .centered()
            .gray()
            .render(wpm_area, buf);
        }

        Paragraph::new(self.current_str.clone())
            .centered()
            .white()
            .render(words_area, buf);

        let user_str: String = self.user_string.iter().collect();
        Paragraph::new(user_str)
            .centered()
            .green()
            .render(input_area, buf);

        if let UserInput::Miss(c) = self.last {
            Paragraph::new(c.to_string())
                .centered()
                .red()
                .render(miss_area, buf);
        }

        if self.show_diff_popup {
            let block = Block::bordered()
                .title("Difficulties")
                .borders(Borders::ALL);
            let area = popup_area_exact(area, 10, 40);
            let [block_area, actions_area] =
                Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

            let list_items = self
                .difficulties
                .iter()
                .map(|item| ListItem::from(item.as_str()))
                .collect::<Vec<ListItem>>();

            let list = List::new(list_items)
                .block(block)
                .highlight_symbol("> ")
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

            Clear.render(area, buf);
            StatefulWidget::render(list, block_area, buf, &mut self.diff_state);
            Line::from("controls").centered().render(actions_area, buf);
        }
    }
}

impl Difficulty {
    fn batch_size(&self) -> usize {
        match self {
            Difficulty::Easy => 1_000,
            Difficulty::Medium => 3_000,
            Difficulty::Hard => 5_000,
            Difficulty::VeryHard => 10_000,
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area_percent(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area_exact(area: Rect, x: u16, y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Max(x)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Max(y)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
