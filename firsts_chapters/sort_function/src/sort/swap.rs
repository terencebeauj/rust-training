pub fn swap(array: &mut [u32], i: usize, j: usize) -> bool {
    let temp = array[i];
    array[i] = array[j];
    array[j] = temp;
    true
}