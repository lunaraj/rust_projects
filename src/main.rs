use std::{io};
use rand::Rng; // 0.8.0
fn main() {
    if password_analyzer() == true{
        command_executer()
    }
    else{
        quit()
    }
   
}
fn password_input() -> String { // gets password from user
    let mut password = String::new();
    println!("type in the password");
    io::stdin().read_line(&mut password).expect("failed to read line");
    return password
}
fn password_analyzer() -> bool { // decides whether password is correct
    let password = password_input();
    if password.trim() == "mcflurry"{
        println!("password accepted");
        return true
    }
    else{
        println!("password declined");
        return false
    }
}
fn input_function() -> String { // gets input command from user
    let mut input = String::new();
    println!();
    println!();
    println!();
    println!("type a command");
    io::stdin().read_line(&mut input).expect("failed to read line");
    return input
}
fn command_executer() { // executes command written
    let command = input_function();
    if command.trim() == "arden" {
        arden()
    }
    if command.trim() == "quit"{
        quit()
    }
    if command.trim() == "moo"{
        moo()
    }
    if command.trim() == "guessing game" {
        guessing_game()
    }
    if command.trim() == "jack" {
        jack()
    }
    else {
        println!("invalid command");
        command_executer()
    }
}
fn jack() {
    println!("he is racist");
    command_executer()
}
fn guessing_game() {
    println!("Guess a number between 1 and 10");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    let actual = rand::thread_rng().gen_range(0..10);
    if guess.trim() == actual.to_string() {
        println!("your guess {}was correct!", guess);
    }
    else {
        println!("your guess {}was incorrect. The correct answer was {}.", guess, actual)
    }
    command_executer()
}
fn quit() { // command to terminate program
    std::process::abort()
}
fn moo() {
    println!(r" 
     \'.____.'/
    __'-.  .-'__                         .--.
    '_i:'oo':i_'---...____...----i'''-.-'.-'\\
      /._  _.\       :       /   '._   ;/    ;'-._     Drink your milk
     (  o  o  )       '-.__.'       '. '.     '-.'     
      '-.__.-' _.--.                 '-.:              
       : '-'  /     ;   _..--,  /       ;
       :      '-._.-'  ;     ; :       :
        :  `      .'    '-._.' :      /
         \  :    /    ____....--\    :
          '._\  :'''''    '.     !.   :
           : |: :           'www'| \ '|
           | || |              : |  | :
           | || |             .' !  | |
          .' !| |            /__I   | |
         /__I.' !                  .' !
            /__I                  /__I  
    ");
    let num = rand::thread_rng().gen_range(0..10);
    if num == 7 {
        dairy_king()
    }
    else {
        command_executer()
    }
    
}
fn dairy_king() {
    println!("you have summoned the dairy king");
    command_executer()
}
fn arden() { // arden command
    let arr = ["arden", "is", "a", "cutie"];
    let mut num = 0;
    let mut count = 0;
    println!();
    loop {
        if count == 5{
            break;
        }
        if num == 4 {
            num = 0;
            count += 1;
            println!();
        }
        println!("{}", arr[num]);
        num += 1;
    }
    command_executer()
}

