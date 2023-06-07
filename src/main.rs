use console::Term;

mod dict;

fn main() {
    let word = dict::random_word();
    println!("type: {}", word);

    let mut chars = word.chars().peekable();

    let term = Term::stdout();
    loop {
        match term.read_key() {
            Ok(key) => match key {
                console::Key::Char(c) => {
                    if chars.peek().is_none() {
                        return;
                    }

                    if &c == chars.peek().unwrap() {
                        let c = chars.next().unwrap();
                        println!("{}", c)
                    }
                }
                _ => todo!(),
            },
            Err(err) => panic!("{err}"),
        };
    }
}
