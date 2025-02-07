pub fn binary_search(low: usize, high: usize) -> Option<usize> {
    if 1 < high.wrapping_sub(low) { Some(low.wrapping_add(high)/2) } else { None }
}

pub(crate) fn join(s: impl Iterator<Item = String>) -> Option<String> {
    let mut res = s.into_iter().fold(String::new(), |mut acc, e| { acc += &e; acc += ", "; acc });
    if res.is_empty() { return None; }
    res.truncate(res.len() - 2);
    Some(res)
}
