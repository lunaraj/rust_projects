use std::{vec, io, string};
// LONGEST SUBSTRING OF ALPHABETICAL LETTERS
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input cannot be read");
    let input = input.trim();
    let mut recent = 'a';
    let mut word = String::new();
    let mut word2 = String::new();
    for i in 0..input.len() {
        let current_letter = string_to_char(input, i);
        let word_copy = word.clone();
        if current_letter >= recent {
            word.push(current_letter);
            println!("{}", word);
        }
        else {
            if word.len() >= word2.len() {
                word2 = word_copy;
            }
            word.clear();
            word.push(current_letter);
        }
        recent = current_letter
    }
    println!("{}!!!", word2)
}

fn string_to_char(stringy: &str, number: usize) -> char{
    let byte: u8 = stringy.as_bytes()[number];
    let charry: char = byte as char;
    return charry
}
