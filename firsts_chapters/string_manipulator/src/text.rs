pub struct Text {
    pub text: String,
}

impl Text {
    pub fn append(&mut self, s: &str) {
        self.text.push_str(s);
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn count_word(&self, word: &str) -> u32 {
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