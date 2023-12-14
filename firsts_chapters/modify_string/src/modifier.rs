pub fn modify_string(s: &mut String) {
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