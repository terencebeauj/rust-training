mod finder;

fn main() {
    let s = String::from("Hello world!");
    let word = finder::find_first_word(&s);

    println!("The first word in {s} is: {word}");
}
