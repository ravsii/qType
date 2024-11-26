use crate::dict::Dictionary;
use app::App;
use dict::Language;
use std::io;

mod app;
mod dict;
mod event;
mod screens;
mod wpm;

fn main() -> Result<(), io::Error> {
    let mut dict = Dictionary::new();
    dict.load(Language::English).expect("dict loaded");

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut typer = App::new(dict)?;
    typer.run(terminal)?;

    ratatui::restore();

    Ok(())
}
