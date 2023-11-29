fn main() {
    let numbers = String::from("0,1,2,3,4,5,6,7,8,9");
    let number_vec = parse_numbers(&numbers);
    dbg!(number_vec);
}

fn parse_numbers(s: &str) -> Vec<u32> {
    let arr: Vec<u32> = s
        .split(',')
        .map(|item| item.parse::<u32>().unwrap())
        .collect();
    arr
}
