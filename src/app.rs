use itertools::Itertools;
use std::io::Stdout;

use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::Stylize,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Terminal,
};

use crate::{dict::Dictionary, wpm::WpmCounter};

enum UserInput {
    Pass,
    Miss(char),
    Hit(char),
}

pub struct App {
    dict: Dictionary,
    terminal: Terminal<CrosstermBackend<Stdout>>,

    current_string: String,
    user_string: Vec<char>,
    current_pos: i32,

    wpm: WpmCounter,
}

impl App {
    pub fn new(dict: Dictionary) -> Result<Self, std::io::Error> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let current_string = dict.random_words(5).join(" ");
        Ok(Self {
            terminal,
            dict,
            current_string,
            user_string: vec![],
            wpm: WpmCounter::new(),
            current_pos: 0,
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut last: UserInput = UserInput::Pass;

        loop {
            if self.current_pos == 1 {
                self.wpm.start();
            }

            if let UserInput::Hit(c) = last {
                self.user_string.push(c);
                self.current_pos += 1;
                self.wpm.tick_char(&c);
            }

            if self.current_pos as usize >= self.current_string.len() {
                self.randomize_input();
            }

            let user_input_joined: String = self.user_string.iter().collect();
            self.terminal.draw(|frame| {
                let layout = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ]);

                let [wpm_area, words, input_area, miss_area, _, bottom_bar] =
                    layout.areas(frame.area());

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

                let input = Paragraph::new(user_input_joined).centered().green();
                frame.render_widget(input, input_area);

                if let UserInput::Miss(c) = last {
                    let input = Paragraph::new(c.to_string()).centered().red();
                    frame.render_widget(input, miss_area);
                }

                App::render_bottom_bar(bottom_bar, frame.buffer_mut());
            })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('r') {
                    self.randomize_input();
                    last = UserInput::Pass;
                    continue;
                }

                if let KeyCode::Char(c) = key.code {
                    let next_char = self.current_string.chars().nth(self.current_pos as usize);
                    match next_char {
                        Some(nc) if nc == c => last = UserInput::Hit(c),
                        _ => last = UserInput::Miss(c),
                    };
                }
            }
        }
    }

    fn randomize_input(&mut self) {
        self.current_string = self.dict.random_words(5).join(" ");
        self.user_string.clear();
        self.current_pos = 0;
    }

    fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
        let keys = [("<C-q>", "Quit"), ("<C-r>", "New random words")];
        let spans = keys
            .iter()
            .flat_map(|(key, desc)| [Span::from(format!("{key} {desc}")), Span::from(" | ")])
            .take(keys.len() * 2 - 1)
            .collect_vec();

        Line::from(spans).centered().render(area, buf);
    }

    pub fn stop(self) {
        ratatui::restore()
    }
}
