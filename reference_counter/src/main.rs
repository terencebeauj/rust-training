fn main() {
    let array: [u32; 15] = [1,2,3,4,5,5,5,5,6,7,7,8,8,8,9];
    let target = 5;
    let count = count_int(&array, target);

    println!("{target} appears {count} times.")
}

fn count_int(arr: &[u32], target: u32) -> u32 {
    let mut count = 0;
    for &i in arr {
        if i == target {
            count += 1;
        }
    }

    count
}
