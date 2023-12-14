mod manipulator;

use crate::manipulator::find_median;
use crate::manipulator::find_mode;

fn main() {
    let mut v: Vec<i32> = vec![1, 54, 36, 84, 100, 2, 13, 36, 2];
    let mut v2: Vec<i32> = vec![10, 20, 30, 40, 50, 50, 60, 70, 80, 90, 100];
    let mut v3: Vec<i32> = vec![-1];

    let median1 = find_median(&mut v);
    let median2 = find_median(&mut v2);
    let median3 = find_median(&mut v3);

    println!("Median for v is: {}", median1);
    println!("Median for v2 is {}", median2);
    println!("Median for v3 is {}", median3);

    let mode: Option<i32> = find_mode(&v);
    let mode2: Option<i32> = find_mode(&v2);
    let mode3: Option<i32> = find_mode(&v3);

    match mode {
        Some(i) => println!("Mode is: {}", i),
        None => println!("There is no mode in the vector"),
    }

    match mode2 {
        Some(i) => println!("Mode 2 is: {}", i),
        None => println!("There is no mode in the vector"),
    }

    match mode3 {
        Some(i) => println!("Mode 3 is: {}", i),
        None => println!("There is no mode in the vector"),
    }
}
