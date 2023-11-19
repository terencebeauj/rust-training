fn main() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let sentences: [&str; 12] = [
        "a partridge in a pear tree!",
        "Two turtle doves,\nAnd ",
        "Three French hens,\n",
        "Four calling birds,\n",
        "Five golden rings,\n",
        "Six geese a-laying,\n",
        "Seven swans a-swimming,\n",
        "Eight maids a-milking,\n",
        "Nine ladies dancing,\n",
        "Ten lords a-leaping,\n",
        "Eleven pipers piping,\n",
        "Twelve drummers drumming,\n",
    ];

    for i in 0..days.len() {
        let day = days[i];
        let mut lyrics = format!("On the {day} of Christmas\nmy true love sent to me\n");

        let array = &sentences[0..=i];

        for &elem in array.iter().rev() {
            lyrics.push_str(elem);
        }

        println!("{lyrics}\n");
    }
}
