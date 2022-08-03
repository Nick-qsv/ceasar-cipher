#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::stdin;

fn get_message() -> String {
    let mut message = String::new();
    stdin().read_line(&mut message).expect("error");
    message.trim().to_lowercase()
}

fn get_password() -> String {
    let mut password = String::new();
    stdin().read_line(&mut password).expect("error");
    password.trim().to_lowercase()
}

enum State {
    MainMenu,
    Encrypt,
    Decrypt,
}

fn main() {
    let alphabet = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let mut pass_phrase_alphabet: Vec<&str> = vec![];
    loop {
        println!("Welcome to Nick's Ceasar Cipher!  Please enter the word you want to encrypt, no spaces! Or leave empty and press ENTER to quit.");
        let message = get_message().to_lowercase();
        //need some way to see if it's empty
        //then iter through the string
        //oh when you iter through the string thats going to return an option
        //jeffrey, two
    }
    let password = get_password();

    println!("Hello, world!");
}
