mod translate_numbers;
mod translate_text;
mod translator;

use std::io::{stdin, stdout, Write};

use crate::translate_numbers::translate_numbers;
use crate::translate_text::translate_text;

fn main() {
    loop {
        println!("1) Translate text to numbers\n2) Translate numbers to text\n3) Exit");
        print!("Enter your choice > ");
        stdout().flush().unwrap();

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();

        let result = match choice.trim() {
            "1" => translate_text(),
            "2" => translate_numbers(),
            "3" => break,
            _ => panic!("Invalid choice"),
        };

        println!("Translation: {}", result.translated_text);
    }
}
