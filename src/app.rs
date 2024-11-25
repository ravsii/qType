use std::{cell::RefCell, rc::Rc};

use crate::{
    dict::Dictionary,
    screens::{typing::TypingScreen, Screen},
};
use ratatui::DefaultTerminal;

pub struct App {
    dict: Rc<RefCell<Dictionary>>,
    screen: Screen,
    // typing_screen: TypingScreen,
    // select_dict_screen: SelectDictScreen,
}

impl App {
    pub fn new(dict: Dictionary) -> Result<Self, std::io::Error> {
        let dict = Rc::new(RefCell::new(dict));
        let typing_screen = TypingScreen::new(&dict);
        // let select_dict_screen = SelectDictScreen::new(dict);

        Ok(Self {
            dict,
            screen: Screen::Typing(typing_screen),
            // typing_screen,
            // select_dict_screen,
        })
    }

    pub fn run(&self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&self.screen, frame.area()))?;

            // if let event::Event::Key(key) = event::read()? {
            //     if key.kind != KeyEventKind::Press {
            //         continue;
            //     }
            //
            //     match self.screen {
            //         Screen::Typing(typing_screen) => {
            //             typing_screen.handle_key(key);
            //         }
            //     }
            //
            //     if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
            //         return Ok(());
            //     }
            //     if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s') {
            //         self.current_screen = self.select_dict_screen;
            //         continue;
            //     }
            // }
        }
    }
}
