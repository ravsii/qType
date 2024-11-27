use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{dict::Dictionary, event::Event, wpm::WpmCounter};

use super::Screen;

#[derive(Clone, Copy, Debug)]
enum UserInput {
    Pass,
    Miss(char),
    Hit(char),
}

#[derive(Clone, Debug)]
pub struct TypingScreen {
    current_string: String,
    user_string: Vec<char>,
    current_pos: i32,
    // last holds user's last input to apply changes on redraw.
    last: UserInput,

    dict: Rc<RefCell<Dictionary>>,
    wpm: WpmCounter,
}

impl TypingScreen {
    pub fn new(dict: &Rc<RefCell<Dictionary>>) -> Self {
        Self {
            current_pos: 0,
            dict: dict.clone(),
            current_string: dict.borrow().random_words(5).join(" "),
            last: UserInput::Pass,
            user_string: vec![],
            wpm: WpmCounter::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Event {
        if self.current_pos == 1 {
            self.wpm.start();
        }

        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('d') {
            return Event::Switch(Screen::Dicts);
        }

        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('r') {
            self.randomize_input();
            self.last = UserInput::Pass;
            return Event::DoNothing;
        }

        if let KeyCode::Char(c) = key.code {
            let next_char = self.current_string.chars().nth(self.current_pos as usize);
            match next_char {
                Some(nc) if nc == c => self.last = UserInput::Hit(c),
                _ => self.last = UserInput::Miss(c),
            };
        };

        if let UserInput::Hit(c) = self.last {
            self.user_string.push(c);
            self.current_pos += 1;
            self.wpm.tick_char(&c);
        }

        // we respect utf-8
        if self.current_pos as usize >= self.current_string.chars().count() {
            self.randomize_input();
        }

        Event::DoNothing
    }

    pub fn randomize_input(&mut self) {
        self.current_string = self.dict.borrow().random_words(5).join(" ");
        self.user_string.clear();
        self.current_pos = 0;
    }

    pub fn actions() -> Vec<(&'static str, &'static str)> {
        vec![("<C-r>", "New random words"), ("<C-d>", "Select Dict")]
    }
}

impl Widget for &TypingScreen {
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

        Paragraph::new(self.current_string.to_owned())
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
    }
}
