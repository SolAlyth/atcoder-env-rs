// use crate::mylib::util::func::as_range;
use std::{ops::RangeBounds, collections::BTreeSet};

/// 半開区間を set で管理するやつ。
/// 
/// 半開区間なので `usize::MAX` は入れられないことに注意。
pub struct RangeSet {
    set: BTreeSet<(usize, usize)>
}

impl RangeSet {
    pub fn new() -> Self { Self { set: BTreeSet::new() } }
    
    pub fn insert(&mut self, value: usize) {
        let (mut l, mut r) = (value, value+1);
        
        let mut iter = self.set.range(..=(value+1, usize::MAX)).rev();
        
        
    }
}
