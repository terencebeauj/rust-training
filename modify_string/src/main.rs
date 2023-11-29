fn main() {
    let mut s = String::from("Hello, my name is ");
    modify_string(&mut s);

    println!("New string: {s}");
}

fn modify_string(s: &mut String) {
    let length = s.len();

    let i1 = 5;
    let i2 = 6;
    let i3 = 7;

    if i1 <= length {
        s.insert(i1, 'J');
    }

    if i2 <= length {
        s.insert(i2, 'Z');
    }

    if i3 <= length {
        s.insert(i3, 'T');
    }
}
