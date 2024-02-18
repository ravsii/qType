use std::io::{self, Write};

use console::{Key, Style, Term};

pub struct InputResult {
    pub c: char,
    pub typ: InputResultType,
}

pub enum InputResultType {
    Miss,
    Hit,
}

pub struct TermHandler {
    t: Term,
}

pub fn new() -> io::Result<TermHandler> {
    let term = Term::stdout();
    term.hide_cursor()?;
    Ok(TermHandler { t: term })
}

impl TermHandler {
    pub fn clear(&self) -> io::Result<()> {
        self.t.clear_screen()
    }

    pub fn write_target(&mut self, s: &str) -> io::Result<()> {
        let target_style: console::Style = Style::new().on_cyan().black();
        self.write_style(s, target_style)
    }

    pub fn write_user_input(&mut self, s: &str) -> io::Result<()> {
        let user_style: console::Style = Style::new().green();
        self.write_style(s, user_style)
    }

    pub fn write_user_miss(&mut self, s: &str) -> io::Result<()> {
        let miss_style: console::Style = console::Style::new().red();
        self.write_style(s, miss_style)
    }

    fn write_style(&mut self, s: &str, style: console::Style) -> io::Result<()> {
        self.t.write_fmt(format_args!("{}\n", style.apply_to(s)))
    }

    pub fn get_input_char(&self) -> char {
        loop {
            match self.t.read_key() {
                Ok(key) => {
                    if let Key::Char(c) = key {
                        return c;
                    }
                }
                Err(_) => todo!(),
            };
        }
    }
}
