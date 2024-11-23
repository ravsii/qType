use std::io;

mod typer;
mod words;
mod wpm;

fn main() -> Result<(), io::Error> {
    let mut dict = words::Dictionary::new();
    dict.load_dict_file("./dict/en.txt").expect("dict loaded");

    let mut typer = typer::Typer::new(dict)?;
    typer.run()?;
    typer.stop();

    Ok(())
}
