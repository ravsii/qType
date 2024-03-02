use crossterm::event::{self, Event, KeyCode};

use crate::dictionary::Dictionary;
use std::io;
mod ui;

pub struct InputResult {
    pub c: char,
    pub typ: InputResultType,
}

pub enum InputResultType {
    Miss,
    Hit,
}

pub struct Typer {
    dict: Dictionary,
    ui: ui::UI,
}

pub fn new(dictionary: Dictionary) -> Typer {
    let ui = ui::init().expect("initialized interface");
    Typer {
        ui,
        dict: dictionary,
    }
}

impl Typer {
    /// run runs an infinite loop of the app.
    pub fn run(&mut self) {
        let words = self.dict.random_words(5);
        // let mut written = String::new();
        // let mut current_pos = 0;
        // let mut miss_char: Option<char> = None;
        //
        loop {
            self.ui.test(words.join(" "));
            self.handle_key().expect("key handled");
        }

        // loop {
        //     term.clear();
        //     term.write_target(&all_words);
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

        self.ui.exit().expect("successful close")
    }

    pub fn handle_key(&mut self) -> io::Result<bool> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(true);
            }

            if let KeyCode::Char(c) = key.code {
                self.ui.test(c.to_string())
            }
        }

        Ok(true)
    }

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
