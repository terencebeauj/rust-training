mod swap;

pub fn sort(array: &mut [u32]) {
    loop {
        let mut swapped = false;

        for i in 0..(array.len() - 1) {
            if array[i] > array[i + 1] {
                swapped = swap::swap(array, i, i + 1);
            }
        }
        if !swapped {
            break;
        }
    }
}
