mod concatener;

fn main() {
    let mut s = String::from("Hello,");
    let s2 = String::from("world!");

    concatener::concatenate(&mut s, s2);

    println!("{}", s);
}
