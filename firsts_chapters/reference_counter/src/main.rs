mod counter;

fn main() {
    let array: [u32; 15] = [1,2,3,4,5,5,5,5,6,7,7,8,8,8,9];
    let target = 5;
    let count = counter::count_int(&array, target);

    println!("{target} appears {count} times.")
}
