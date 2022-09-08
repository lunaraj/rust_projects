use std::{io};
extern crate rand;
use rand::{Rng, random};
fn main() {
    let mut list = Vec::new();
    for i in 0..=99999{
        let letter = random_generator();
        list.push(letter)
    }
    let mut list_string = String::new();
    for i in 0..list.len() {
        list_string.push(list[i]);
    }
    for i in 0..list.len() {
        list_string.push(list[list.len()-i-1])
    }
    let mut list2 = Vec::new();
    for i in 0..list_string.len() {
        let letter = string_to_char(list_string.clone(), i);
        list2.push(letter);
    }
    let mut reverse = String::new();
    for i in 0..list2.len(){
        reverse.push(list2[list2.len()-i-1])
    }
    let true_or_false = reverse.trim() == list_string.trim();
    println!("{}", reverse);
    println!("{}", true_or_false);
}


fn string_to_char(input_string: String, num: usize) -> char{
    let byte = input_string.as_bytes()[num];
    let character = byte as char;
    return character
}
fn random_generator() -> char {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(97..=122);
    let num = num as u8;
    let character = num as char;
    return character
}