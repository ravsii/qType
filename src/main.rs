mod dictionary;
mod typer;

fn main() {
    let mut dict = dictionary::new();
    dict.load_dict_file("./dict/en.txt").expect("dict loaded");

    let mut term = typer::new(dict);
    term.run()
}
