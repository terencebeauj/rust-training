fn main() {
    let mut array: [u32; 5] = [1, 4, 3, 2, 6];
    sort(&mut array);
    dbg!(array);
}

fn sort(array: &mut [u32]) {
    loop {
        let mut swapped = false;

        for i in 0..(array.len() - 1) {
            if array[i] > array[i + 1] {
                swapped = swap(array, i, i + 1);
            }
        }
        if !swapped {
            break;
        }
    }
}

fn swap(array: &mut [u32], i: usize, j: usize) -> bool {
    let temp = array[i];
    array[i] = array[j];
    array[j] = temp;
    true
}
