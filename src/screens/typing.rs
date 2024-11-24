use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{dict::Dictionary, wpm::WpmCounter};

enum UserInput {
    Pass,
    Miss(char),
    Hit(char),
}

pub struct TypingScreen {
    current_string: String,
    user_string: Vec<char>,
    current_pos: i32,
    // last holds user's last input to apply changes on redraw.
    last: UserInput,

    dict: Dictionary,
    wpm: WpmCounter,
}

impl TypingScreen {
    pub fn new(dict: Dictionary) -> Self {
        Self {
            current_pos: 0,
            current_string: dict.random_words(5).join(" "),
            dict,
            last: UserInput::Pass,
            user_string: vec![],
            wpm: WpmCounter::new(),
        }
    }

    pub fn draw(&mut self, area: Rect, frame: &mut Frame) {
        if self.current_pos == 1 {
            self.wpm.start();
        }

        if let UserInput::Hit(c) = self.last {
            self.user_string.push(c);
            self.current_pos += 1;
            self.wpm.tick_char(&c);
        }

        if self.current_pos as usize >= self.current_string.len() {
            self.randomize_input();
        }

        let layout = Layout::vertical([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

        let [wpm_area, words, input_area, miss_area] = layout.areas(area);

        if self.wpm.is_started() {
            let current = self.wpm.current_wpm();
            let wpm_widget = Paragraph::new(format!(
                "{:.1} cpm | {:.1} wpm",
                current.chars_per_min, current.words_per_min
            ))
            .centered()
            .gray();

            frame.render_widget(wpm_widget, wpm_area);
        }

        let greeting = Paragraph::new(self.current_string.to_owned())
            .centered()
            .white();
        frame.render_widget(greeting, words);

        let user_str: String = self.user_string.iter().collect();
        let input = Paragraph::new(user_str).centered().green();
        frame.render_widget(input, input_area);

        if let UserInput::Miss(c) = self.last {
            let input = Paragraph::new(c.to_string()).centered().red();
            frame.render_widget(input, miss_area);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('r') {
            self.randomize_input();
            self.last = UserInput::Pass;
            return;
        }

        if let KeyCode::Char(c) = key.code {
            let next_char = self.current_string.chars().nth(self.current_pos as usize);
            match next_char {
                Some(nc) if nc == c => self.last = UserInput::Hit(c),
                _ => self.last = UserInput::Miss(c),
            };
        }
    }

    fn randomize_input(&mut self) {
        self.current_string = self.dict.random_words(5).join(" ");
        self.user_string.clear();
        self.current_pos = 0;
    }

    pub fn custom_options() -> Vec<(&'static str, &'static str)> {
        return vec![("<C-r>", "New random words")];
    }
}
