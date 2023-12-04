mod parser;

fn main() {
    let numbers = String::from("0,1,2,3,4,5,6,7,8,9");
    let number_vec = parser::parse_numbers(&numbers);
    dbg!(number_vec);
}
