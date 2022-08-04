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
    let alphabet: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut pass_phrase_alphabet: Vec<char> = vec![];

    println!("Welcome to Nick's Ceasar Cipher!  Please enter the password for your encryption, no spaces! Or leave empty and press ENTER to quit.");
    let pk = get_password();

    while pass_phrase_alphabet.len() < 26 {
        for c in pk.chars() {
            if pass_phrase_alphabet.len() < 26 {
                pass_phrase_alphabet.push(c);
                println!("{}", c);
            } else {
                break;
            }
        }
    }
    print!(
        "length is {}, this is the key: {:?}",
        pass_phrase_alphabet.len(),
        pass_phrase_alphabet
    );
    //now we have the key and we need to encrypt the phrase
    println!("Thank you, your private key is ready.  Please enter the phrase you want to encrypt! Or leave empty and press ENTER to quit.");
    let m = get_message();
    let mut encrypted = String::new();
    for c in m.chars() {
        let index = alphabet.iter().position(|&r| r == c).unwrap();
        encrypted.push(pass_phrase_alphabet[index]);
    }
    println!("Your encrypted phrase is: {}", &encrypted);
    //loop through every char of the string
    //take the character and loop through the alphabet and the index of that
    //push the index of the pass_phrase_alphabet onto a new string
    //return a new string
    //loop through
    //then we have to return it

    //need some way to see if it's empty
    //then iter through the string
    //oh when you iter through the string thats going to return an option
    //jeffrey, two loops.  for x 0..25{ for i 0..string.len(){
    //vector.push(str[i])
    //}}
}
