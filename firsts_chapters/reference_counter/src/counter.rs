pub fn count_int(arr: &[u32], target: u32) -> u32 {
    let mut count = 0;
    for &i in arr {
        if i == target {
            count += 1;
        }
    }

    count
}