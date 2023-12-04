pub fn parse_numbers(s: &str) -> Vec<u32> {
    let arr: Vec<u32> = s
        .split(',')
        .map(|item| item.parse::<u32>().unwrap())
        .collect();
    arr
}