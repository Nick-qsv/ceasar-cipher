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

enum Mode {
    MainMenu,
    Encrypt,
    Decrypt,
}

struct State {}

fn main() {
    let alphabet: [char; 27] = [
        ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut pass_phrase_alphabet: Vec<char> = vec![];
    loop {
        println!("Welcome to Nick's Ceasar Cipher!  Please enter the password for your encryption, no spaces! Or leave empty and press ENTER to quit.");
        let pk = get_password();
        if pk.is_empty() {
            break;
        }
        while pass_phrase_alphabet.len() < 27 {
            for c in pk.chars() {
                if pass_phrase_alphabet.len() < 27 {
                    pass_phrase_alphabet.push(c);
                    // println!("{}", c);
                } else {
                    break;
                }
            }
        }
        // println!(
        //     "length is {}, this is the key: {:?}",
        //     pass_phrase_alphabet.len(),
        //     pass_phrase_alphabet
        // );

        //now we have the key and we need to encrypt the phrase
        println!("Thank you, your private key is ready.  Please enter the phrase you want to encrypt! Or leave empty and press ENTER to quit.");
        let m = get_message();
        if m.is_empty() {
            break;
        }
        let mut encrypted = String::new();
        for c in m.chars() {
            let index = alphabet.iter().position(|&r| r == c).unwrap();
            encrypted.push(pass_phrase_alphabet[index]);
        }
        println!("Your encrypted phrase is: {}", &encrypted);
        break;
    }
}
