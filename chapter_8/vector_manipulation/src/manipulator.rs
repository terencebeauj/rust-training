use std::collections::HashMap;

pub fn find_median(v: &mut Vec<i32>) -> i32 {
    v.sort();

    let length = v.len();

    if length == 1 {
        return v[0];
    }

    if length % 2 == 0 {
        let i = (length / 2) - 1;
        let j = length / 2;
        let median = (v[i] + v[j]) / 2;
        return median;
    } else {
        let i = ((length + 1) / 2) - 1;
        return v[i];
    }
}

pub fn find_mode(v: &Vec<i32>) -> Option<i32> {
    let mut dict = HashMap::new();
    for num in v {
        let count = dict.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max: Option<i32> = None;
    let mut i = i32::MIN;
    for (key, value) in dict {
        if value > i {
            i = value;
            max = Some(*key);
        }
    }
    max
}
