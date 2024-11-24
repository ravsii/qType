use itertools::Itertools;
use std::{borrow::BorrowMut, io::Stdout};

use crate::{
    dict::Dictionary,
    screens::{select_dict::SelectDictScreen, typing::TypingScreen, Screen},
};
use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    text::{Line, Span},
    widgets::Widget,
    Terminal,
};

pub struct App<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    dict: &'static Dictionary,
    current_screen: &'a dyn Screen,

    typing_screen: &'a Box<TypingScreen>,
    select_dict_screen: &'a Box<SelectDictScreen>,
}

impl<'a> App<'a> {
    pub fn new(dict: &'static Dictionary) -> Result<Self, std::io::Error> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let typing_screen = Box::new(TypingScreen::new(dict));
        let select_dict_screen = Box::new(SelectDictScreen::new(dict));

        let ts: &'a TypingScreen = Box::leak(typing_screen);
        let ds: &'a SelectDictScreen = Box::leak(select_dict_screen);

        Ok(Self {
            terminal,
            dict,
            current_screen: ts,
            typing_screen: ts,
            select_dict_screen: ds,
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            self.terminal.draw(|frame| {
                let [main, actions_bar] =
                    Layout::vertical([Constraint::Min(0), Constraint::Length(1)])
                        .areas(frame.area());

                self.current_screen.render(main, frame.buffer_mut());
                App::render_bottom_bar(
                    actions_bar,
                    frame.buffer_mut(),
                    self.current_screen.custom_options(),
                );
            })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s') {
                    self.current_screen = self.select_dict_screen;
                    continue;
                }

                if !self.current_screen.handle_key(key) {
                    self.current_screen = self.typing_screen;
                }
            }
        }
    }

    fn render_bottom_bar(area: Rect, buf: &mut Buffer, opts: Vec<(&str, &str)>) {
        let keys = [("<C-q>", "Quit")];
        let len = keys.len() + opts.len();

        let spans = keys
            .iter()
            .chain(opts.iter())
            .flat_map(|(key, desc)| [Span::from(format!("{key} {desc}")), Span::from(" | ")])
            .take(len * 2 - 1)
            .collect_vec();

        Line::from(spans).centered().render(area, buf);
    }

    pub fn stop(self) {
        ratatui::restore()
    }
}
