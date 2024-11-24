use itertools::Itertools;
use std::io::Stdout;

use crate::{dict::Dictionary, screens::typing::TypingScreen, wpm::WpmCounter};
use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    text::{Line, Span},
    widgets::Widget,
    Frame, Terminal,
};

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    typing_screen: TypingScreen,
}

impl App {
    pub fn new(dict: Dictionary) -> Result<Self, std::io::Error> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let typing_screen = TypingScreen::new(dict);

        Ok(Self {
            terminal,
            typing_screen,
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            self.terminal.draw(|frame| {
                let [main, actions_bar] =
                    Layout::vertical([Constraint::Min(0), Constraint::Length(1)])
                        .areas(frame.area());

                self.typing_screen.draw(main, frame);
                App::render_bottom_bar(actions_bar, frame.buffer_mut());
            })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                self.typing_screen.handle_key(key);
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {}

    fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
        let keys = vec![("<C-q>", "Quit")];

        let spans = keys
            .iter()
            .chain(&TypingScreen::custom_options())
            .flat_map(|(key, desc)| [Span::from(format!("{key} {desc}")), Span::from(" | ")])
            .take(keys.len() * 2 - 1)
            .collect_vec();

        Line::from(spans).centered().render(area, buf);
    }

    pub fn stop(self) {
        ratatui::restore()
    }
}
