pub enum TranslatorMode {
    Text,
    Num,
    Default,
}

pub struct Translator {
    pub text: String,
    pub translated_text: String,
    pub mode: TranslatorMode,
}

impl Translator {
    pub fn new(text: String) -> Translator {
        Translator {
            text: text,
            translated_text: String::new(),
            mode: TranslatorMode::Default,
        }
    }

    pub fn translate(&mut self) {
        match self.mode {
            TranslatorMode::Text => self.translate_text(),
            TranslatorMode::Num => self.translate_num(),
            TranslatorMode::Default => panic!("Translator mode not set"),
        }
    }

    pub fn set_mode(&mut self, mode: TranslatorMode) {
        self.mode = mode;
    }

    fn translate_text(&mut self) -> () {
        for c in self.text.chars() {
            self.translated_text.extend(match c {
                'a' => "2 ".chars().into_iter(),
                'b' => "22 ".chars().into_iter(),
                'c' => "222 ".chars().into_iter(),
                'd' => "3 ".chars().into_iter(),
                'e' => "33 ".chars().into_iter(),
                'f' => "333 ".chars().into_iter(),
                'g' => "4 ".chars().into_iter(),
                'h' => "44 ".chars().into_iter(),
                'i' => "444 ".chars().into_iter(),
                'j' => "5 ".chars().into_iter(),
                'k' => "55 ".chars().into_iter(),
                'l' => "555 ".chars().into_iter(),
                'm' => "6 ".chars().into_iter(),
                'n' => "66 ".chars().into_iter(),
                'o' => "666 ".chars().into_iter(),
                'p' => "7 ".chars().into_iter(),
                'q' => "77 ".chars().into_iter(),
                'r' => "777 ".chars().into_iter(),
                's' => "7777 ".chars().into_iter(),
                't' => "8 ".chars().into_iter(),
                'u' => "88 ".chars().into_iter(),
                'v' => "888 ".chars().into_iter(),
                'w' => "9 ".chars().into_iter(),
                'x' => "99 ".chars().into_iter(),
                'y' => "999 ".chars().into_iter(),
                'z' => "9999 ".chars().into_iter(),
                ' ' => "0 ".chars().into_iter(),
                '\r' => "00 ".chars().into_iter(),
                '1' => "1111 ".chars().into_iter(),
                '2' => "2222 ".chars().into_iter(),
                '3' => "3333 ".chars().into_iter(),
                '4' => "4444 ".chars().into_iter(),
                '5' => "5555 ".chars().into_iter(),
                '6' => "6666 ".chars().into_iter(),
                '7' => "77777 ".chars().into_iter(),
                '8' => "8888 ".chars().into_iter(),
                '9' => "99999 ".chars().into_iter(),
                '0' => "000 ".chars().into_iter(),
                _ => panic!("Cannot translate char: {}", c),
            });
        }
        self.translated_text = self.translated_text.trim().to_string();
    }

    fn translate_num(&mut self) {
        let split_chars = self.text.split(" ");

        for chars in split_chars {
            match chars {
                "2" => self.translated_text.push('a'),
                "22" => self.translated_text.push('b'),
                "222" => self.translated_text.push('c'),
                "3" => self.translated_text.push('d'),
                "33" => self.translated_text.push('e'),
                "333" => self.translated_text.push('f'),
                "4" => self.translated_text.push('g'),
                "44" => self.translated_text.push('h'),
                "444" => self.translated_text.push('i'),
                "5" => self.translated_text.push('j'),
                "55" => self.translated_text.push('k'),
                "555" => self.translated_text.push('l'),
                "6" => self.translated_text.push('m'),
                "66" => self.translated_text.push('n'),
                "666" => self.translated_text.push('o'),
                "7" => self.translated_text.push('p'),
                "77" => self.translated_text.push('q'),
                "777" => self.translated_text.push('r'),
                "7777" => self.translated_text.push('s'),
                "8" => self.translated_text.push('t'),
                "88" => self.translated_text.push('u'),
                "888" => self.translated_text.push('v'),
                "9" => self.translated_text.push('w'),
                "99" => self.translated_text.push('x'),
                "999" => self.translated_text.push('y'),
                "9999" => self.translated_text.push('z'),
                "0" => self.translated_text.push(' '),
                "00" => self.translated_text.push('\r'),
                "1111" => self.translated_text.push('1'),
                "2222" => self.translated_text.push('2'),
                "3333" => self.translated_text.push('3'),
                "4444" => self.translated_text.push('4'),
                "5555" => self.translated_text.push('5'),
                "6666" => self.translated_text.push('6'),
                "77777" => self.translated_text.push('7'),
                "8888" => self.translated_text.push('8'),
                "99999" => self.translated_text.push('9'),
                "000" => self.translated_text.push('0'),
                _ => panic!("Cannot translate char: {}", chars),
            }
        }
        self.translated_text = self.translated_text.trim().to_string();
    }
}

#[test]
fn test_translate_text() {
    let mut translator = Translator::new("hello world".to_string());
    translator.set_mode(TranslatorMode::Text);
    translator.translate();
    assert_eq!(
        "44 33 555 555 666 0 9 666 777 555 3".to_string(),
        translator.translated_text
    );
}

#[test]
fn test_translate_num() {
    let mut translator = Translator::new("44 33 555 555 666 0 9 666 777 555 3".to_string());
    translator.set_mode(TranslatorMode::Num);
    translator.translate();
    assert_eq!("hello world".to_string(), translator.translated_text);
}
