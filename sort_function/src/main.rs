mod sort;

fn main() {
    let mut array: [u32; 5] = [1, 4, 3, 2, 6];
    sort::sort(&mut array);
    dbg!(array);
}
