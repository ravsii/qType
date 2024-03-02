use std::io::{self, stdout, Stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

/// init creates a new ratatui on top of crossterm object, clears it and
/// returns a struct to work with.
pub fn init() -> io::Result<UI> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(UI {
        terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
    })
}

impl UI {
    pub fn test(&mut self, words: String) {
        self.terminal
            .draw(|frame| {
                let area = frame.size();
                frame.render_widget(Paragraph::new(words).white().on_blue(), area);
            })
            .expect("test");
    }

    pub fn exit(&self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
