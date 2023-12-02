struct Text {
    text: String,
}

impl Text {
    fn append(&mut self, s: &str) {
        self.text.push_str(s);
    }

    fn clear(&mut self) {
        self.text.clear();
    }

    fn count_word(&self, word: &str) -> u32 {
        let mut counter: u32 = 0;
        let words: Vec<&str> = self.text.split([' ', '.', ',', ';', '?', '!']).collect();
        for item in words {
            if item == word {
                counter += 1;
            }
        }

        counter
    }
}

fn main() {
    let sentence = "hello";
    let mut text_struct = Text {
        text: String::from(sentence),
    };

    text_struct.append(" there, my name is Terence and my profession is software developer");
    dbg!(&text_struct.text);

    let count = text_struct.count_word("is");
    dbg!(&count);

    text_struct.clear();
}

// TODO: Implement the searching substring method