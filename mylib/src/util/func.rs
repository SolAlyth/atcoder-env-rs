pub fn binary_search(low: usize, high: usize) -> Option<usize> {
    if 1 < high.wrapping_sub(low) { Some(low.wrapping_add(high)/2) } else { None }
}
