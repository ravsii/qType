mod app;
mod dict;
mod wpm;
use crate::dict::Dictionary;
use app::App;
use dict::Language;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut dict = Dictionary::new();
    dict.load(Language::English).expect("dict loaded");

    let mut typer = App::new(dict)?;
    typer.run()?;
    typer.stop();

    Ok(())
}
