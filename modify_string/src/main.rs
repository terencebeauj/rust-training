mod modifier;

use modifier::modify_string;

fn main() {
    let mut s = String::from("Hello, my name is ");
    modify_string(&mut s);

    println!("New string: {s}");
}
