use std::fs::{read_to_string, write};

fn main() {
    let word_file = read_to_string("words_all.txt").unwrap();
    let words: String = word_file
        .split("\n")
        .filter_map(|f| {
            let letters_only = f.chars().all(|x| x.is_alphabetic());
            let val = f.to_lowercase().to_string();

            if letters_only && val.len() == 5 {
                Some(val + "\n")
            } else {
                None
            }
        })
        .collect::<String>()
        .trim()
        .to_string();

    write("words_5ltr.txt", words).unwrap();
}
