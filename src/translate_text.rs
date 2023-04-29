use std::io::{stdin, stdout, Write};

use crate::translator::Translator;

pub fn translate_text() -> Translator {
    println!("Translate text to numbers");

    print!("Enter some text > ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut translator =
        crate::translator::Translator::new(String::from(input.trim()).to_lowercase());
    translator.set_mode(crate::translator::TranslatorMode::Text);
    translator.translate();

    translator
}

#[cfg(tests)]
mod Tests {
    #[test]
    fn test_translate_text() {
        let mut translator = crate::translator::Translator::new(String::from("hello"));
        translator.set_mode(crate::translator::TranslatorMode::Text);
        translator.translate();
        assert_eq!(translator.translated_text, "44 33 555 555 666");
    }
}
