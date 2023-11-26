fn main() {
    let mut s = String::from("Hello,");
    let s2 = String::from("world!");

    concatenate(&mut s, s2);

    println!("{}", s);
}

fn concatenate(s1: &mut String, s2: String) {
    s1.push_str(&s2);
}
