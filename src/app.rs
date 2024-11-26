use std::{cell::RefCell, io, rc::Rc};

use crate::{
    dict::Dictionary,
    event::Event,
    screens::{select_dict::SelectDictScreen, typing::TypingScreen, Opts, Screen},
};
use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::*,
    text::{Line, Span},
    DefaultTerminal, Frame,
};

pub struct App {
    dict: Rc<RefCell<Dictionary>>,
    screen: Screen,
    typing_screen: TypingScreen,
    select_dict_screen: SelectDictScreen,
}

impl App {
    pub fn new(dict: Dictionary) -> Result<Self, std::io::Error> {
        // TODO: I don't know if this is the right way to do it, but it works for now.
        let dict = Rc::new(RefCell::new(dict));
        let typing_screen = TypingScreen::new(&dict);
        let select_dict_screen = SelectDictScreen::new(&dict);

        Ok(Self {
            dict,
            screen: Screen::Typing,
            typing_screen,
            select_dict_screen,
        })
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            match self.handle_key()? {
                Event::DoNothing => continue,
                Event::Quit => return Ok(()),
                Event::Switch(_) => todo!(),
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [screen_area, actions_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(frame.area());

        let mut bottom_bar_opts = vec![("<C-q>", "Quit")];
        match self.screen {
            Screen::Typing => {
                frame.render_widget(&self.typing_screen, screen_area);
                bottom_bar_opts.extend(TypingScreen::custom_options());
            }
            Screen::Dicts => {
                frame.render_widget(&mut self.select_dict_screen, screen_area);
                bottom_bar_opts.extend(SelectDictScreen::custom_options());
            }
        }

        let mut spans = bottom_bar_opts
            .iter()
            .flat_map(|(key, desc)| [Span::from(format!("{key} {desc}")), Span::from(" | ")])
            .collect::<Vec<Span>>();

        // excluding last "|"
        spans.pop();

        Line::from(spans)
            .centered()
            .render(actions_area, frame.buffer_mut());
    }

    fn handle_key(&mut self) -> io::Result<Event> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(Event::DoNothing);
            }

            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
                return Ok(Event::Quit);
            };

            match self.screen {
                Screen::Typing => Ok(self.typing_screen.handle_key(key)),
                Screen::Dicts => todo!(),
            }
        } else {
            Ok(Event::DoNothing)
        }
    }
}
