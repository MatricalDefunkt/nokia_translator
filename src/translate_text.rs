use std::io::{stdin, stdout, Write};

use crate::translator::{Translator, TranslatorMode};

pub fn translate_text() -> Translator {
    println!("Translate text to numbers");

    print!("Enter some text > ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut translator = Translator::new(String::from(input.trim()).to_lowercase());
    translator.set_mode(TranslatorMode::Text);
    translator.translate();

    translator
}
