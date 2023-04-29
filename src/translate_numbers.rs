use std::io::{stdin, stdout, Write};

use crate::translator::{Translator, TranslatorMode};

pub fn translate_numbers() -> Translator {
    println!("Translate numbers to text");

    print!("Enter some numbers > ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut translator = Translator::new(String::from(input.trim()).to_lowercase());
    translator.set_mode(TranslatorMode::Num);
    translator.translate();

    translator
}
