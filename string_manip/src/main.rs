fn main() {
    let s = String::from("Hello world!");
    let word = find_first_word(&s);

    println!("The first word in {s} is: {word}");
}

fn find_first_word(my_string: &String) -> &str {
    let string_bytes = my_string.as_bytes();

    for (i, &x) in string_bytes.iter().enumerate() {
        if x == b' ' {
            return &my_string[..i];
        }
    }

    &my_string[..]
}
