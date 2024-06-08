use std::ops::{RangeBounds, Bound::*};

/// 与えられた `RangeBounds` を等価な `left..right` に変換します。
/// 
/// # Panic
/// 
/// + `left <= right <= sup` でないとき。
pub fn as_range(range: impl RangeBounds<usize>, sup: usize) -> (usize, usize) {
    let l = match range.start_bound() {
        Included(&v) => v,
        Excluded(&v) => v+1,
        Unbounded => 0
    };
    
    let r = match range.end_bound() {
        Included(&v) => v+1,
        Excluded(&v) => v,
        Unbounded => sup
    };
    
    if !(l <= r && r <= sup) { panic!("valid: 0..{sup}\ninputed: {l}..{r}"); }
    (l, r)
}
