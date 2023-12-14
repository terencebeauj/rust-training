mod text;

use crate::text::Text;

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