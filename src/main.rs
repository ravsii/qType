use app::App;
use std::io;

mod app;
mod dict;
mod event;
mod screens;
mod wpm;

fn main() -> Result<(), io::Error> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut typer = App::new()?;
    typer.run(terminal)?;

    ratatui::restore();

    Ok(())
}
