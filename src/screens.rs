use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};
use typing::TypingScreen;

pub mod select_dict;
pub mod typing;

pub enum Screen {
    Typing(TypingScreen),
    // Dicts(SelectDictScreen),
}

impl Widget for &Screen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [screen_area, actions_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

        let mut bottom_bar_opts = vec![("<C-q>", "Quit")];
        match self {
            Screen::Typing(typing_screen) => {
                typing_screen.render(screen_area, buf);
                bottom_bar_opts.extend(typing_screen.custom_options());
            } // Screen::Dicts(select_dict_screen) => todo!(),
        }

        let mut spans = bottom_bar_opts
            .iter()
            .flat_map(|(key, desc)| [Span::from(format!("{key} {desc}")), Span::from(" | ")])
            .collect::<Vec<Span>>();

        spans.pop();

        Line::from(spans).centered().render(actions_area, buf);
    }
}

// pub trait Screen {
//     fn render(&mut self, area: Rect, buf: &mut Buffer);
//     fn handle_key(&mut self, key: KeyEvent) -> bool;
//     fn custom_options(&self) -> Vec<(&'static str, &'static str)> {
//         vec![]
//     }
// }
