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

use crate::words;

pub struct InputResult {
    pub c: char,
    pub typ: InputResultType,
}

pub enum InputResultType {
    Miss,
    Hit,
}

pub struct Typer {
    dict: words::Dictionary,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Typer {
    pub fn new(dict: words::Dictionary) -> Result<Self, std::io::Error> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        Ok(Self { terminal, dict })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let words = self.dict.random_words(5);
        let mut user_str: Vec<char> = vec![];
        // let mut written = String::new();
        // let mut current_pos = 0;
        // let mut miss_char: Option<char> = None;
        //
        // loop {
        //     self.ui.test(words.join(" "));
        //     self.handle_key().expect("key handled");
        // }
        loop {
            let vec_str = words.join(" ");
            let user_input_joined: String = user_str.iter().collect();
            self.terminal.draw(|frame| {
                let layout = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ]);

                let [words, input_area, _, bottom_bar] = layout.areas(frame.area());

                let greeting = Paragraph::new(vec_str).white();
                let input = Paragraph::new(user_input_joined).green();
                frame.render_widget(greeting, words);
                frame.render_widget(input, input_area);
                Typer::render_bottom_bar(bottom_bar, frame.buffer_mut());
            })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                match key.code {
                    KeyCode::Char(c) => user_str.push(c),
                    _ => {}
                }
            }
        }

        // loop {
        //     term.write_user_input(written.as_str());
        //     if let Some(pc) = miss_char {
        //         term.write_user_miss(format!("miss: {}", pc).as_str());
        //         miss_char = None;
        //     }
        //     // term.move_cursor_to(written.len(), 1);

        //     let input_char = term.get_input_char();

        //     let pos_char = all_words.chars().nth(current_pos);
        //     if let Some(cur_target_char) = pos_char {
        //         if input_char == cur_target_char {
        //             current_pos += 1;
        //             written.push(input_char);

        //             if written.len() == all_words.len() {
        //                 all_words = dict::random_words(1).join(" ");
        //                 current_pos = 0;
        //                 written.clear();
        //             }
        //         } else {
        //             miss_char = Some(input_char);
        //         }
        //     }
        // }
    }

    fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
        let keys = [
            ("H/←", "Left"),
            ("L/→", "Right"),
            ("K/↑", "Up"),
            ("J/↓", "Down"),
            ("D/Del", "Destroy"),
            ("Q/Esc", "Quit"),
        ];
        let spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::from(format!(" {key} "));
                let desc = Span::from(format!(" {desc} "));
                [key, desc]
            })
            .collect_vec();

        Line::from(spans).centered().render(area, buf);
    }

    pub fn stop(self) {
        ratatui::restore()
    }

    // pub fn handle_key(&mut self) -> io::Result<bool> {
    //     if let Event::Key(key) = event::read()? {
    //         if key.kind == event::KeyEventKind::Release {
    //             return Ok(true);
    //         }
    //
    //         if let KeyCode::Char(c) = key.code {
    //             self.ui.test(c.to_string())
    //         }
    //     }
    //
    //     Ok(true)
    // }

    // pub fn clear(&self) -> io::Result<()> {
    //     self.t.clear_screen()
    // }

    // pub fn write_target(&mut self, s: &str) -> io::Result<()> {
    //     let target_style: console::Style = Style::new().on_cyan().black();
    //     self.write_style(s, target_style)
    // }

    // pub fn write_user_input(&mut self, s: &str) -> io::Result<()> {
    //     let user_style: console::Style = Style::new().green();
    //     self.write_style(s, user_style)
    // }

    // pub fn write_user_miss(&mut self, s: &str) -> io::Result<()> {
    //     let miss_style: console::Style = console::Style::new().red();
    //     self.write_style(s, miss_style)
    // }

    // fn write_style(&mut self, s: &str, style: console::Style) -> io::Result<()> {
    //     self.t.write_fmt(format_args!("{}\n", style.apply_to(s)))
    // }

    // pub fn get_input_char(&self) -> char {
    //     loop {
    //         match self.t.read_key() {
    //             Ok(key) => {
    //                 if let Key::Char(c) = key {
    //                     return c;
    //                 }
    //             }
    //             Err(_) => todo!(),
    //         };
    //     }
    // }
}
