mod dict;
mod term;

fn main() -> ! {
    let mut all_words = dict::random_words(2).join(" ");

    let mut term = term::new().expect("valid term");
    let mut written = String::new();
    let mut current_pos = 0;
    let mut miss_char: Option<char> = None;

    loop {
        term.clear();
        term.write_target(&all_words);
        term.write_user_input(written.as_str());
        if let Some(pc) = miss_char {
            term.write_user_miss(format!("miss: {}", pc).as_str());
            miss_char = None;
        }
        // term.move_cursor_to(written.len(), 1);

        let input_char = term.get_input_char();

        let pos_char = all_words.chars().nth(current_pos);
        if let Some(cur_target_char) = pos_char {
            if input_char == cur_target_char {
                current_pos += 1;
                written.push(input_char);

                if written.len() == all_words.len() {
                    all_words = dict::random_words(1).join(" ");
                    current_pos = 0;
                    written.clear();
                }
            } else {
                miss_char = Some(input_char);
            }
        }
    }
}
