//! 平方分割実装の Set

use num::integer::Roots;
use std::fmt::Debug;

/// 平方分割で実装された Set
/// 
/// # Complexity
/// 
/// space `O(N)`
/// 
/// `len`: `O(1)`  
/// `insert`, `remove`, `nth`, `position` などほとんどの操作: `O(√N)`
/// 
/// # Guarantee
/// 
/// `buckets: Vec<Vec<T>>` は空にはならない。何も要素を持っていないとき `[[]]` になる。
/// 
/// `bucket` は、何も要素を持っていないときを除き、空にならない。
/// 
/// # Reference
/// 
/// [tatyam さんの記事 (Qiita)](https://qiita.com/tatyam/items/492c70ac4c955c055602)  
/// [Blue-Suger さんの GitHub リポジトリ](https://github.com/Blue-Sugar/ac_library-rust/blob/main/src/data_structure/sorted_set.rs)
pub struct SqrtSet<T: Ord> {
    buckets: Vec<Vec<T>>,
    _len: usize
}

impl<T: Ord> SqrtSet<T> {
    const BUCKET_RATIO: usize = 16;
    const SPLIT_RATIO: usize = 24;
    
    /// 空の Set を返す。
    pub fn new() -> Self {
        SqrtSet { buckets: vec![vec![]], _len: 0 }
    }
    
    /// 要素数を返す。
    pub fn len(&self) -> usize { self._len }
    
    /// 第一返り値は、`value` が入るべきバケットのインデックスを返す。  
    /// 第二返り値は、そのバケット内で `binary_search(value)` した結果を返す。  
    /// 特に、`Ok(idx)` ならば `buckets[bidx][idx] == value` となる。
    fn raw_position(&self, value: &T) -> (usize, Result<usize, usize>) {
        if self._len == 0 { return (0, Err(0)); }
        let bidx = self.buckets[..].partition_point(|v| v[0].le(&value)).saturating_sub(1);
        (bidx, self.buckets[bidx].binary_search(&value))
    }
    
    /// `buckets[bidx]` を適切に分割・削除する。
    fn check(&mut self, bidx: usize) {
        let blen = self.buckets[bidx].len();
        
        // split
        if self.buckets.len() * Self::SPLIT_RATIO < blen {
            let tmp = self.buckets[bidx].drain(blen/2..).collect();
            self.buckets.insert(bidx+1, tmp);
        }
        
        // delete
        if blen == 0 && self._len != 0 { self.buckets.remove(bidx); }
    }
    
    /// `value` を Set に追加する。新たに `value` を追加したかどうかを表す `bool` 値を返す。
    pub fn insert(&mut self, value: T) -> bool {
        let (bidx, Err(idx)) = self.raw_position(&value) else { return false; };
        self.buckets[bidx].insert(idx, value); self._len += 1;
        self.check(bidx); true
    }
    
    /// `value` を Set から削除する。`value` を削除できたかどうかを表す `bool` 値を返す。
    pub fn remove(&mut self, value: &T) -> bool {
        let (bidx, Ok(idx)) = self.raw_position(value) else { return false; };
        self.buckets[bidx].remove(idx); self._len -= 1;
        self.check(bidx); true
    }
    
    /// `value` が Set に存在するかどうかを表す `bool` 値を返す。
    pub fn contains(&self, value: &T) -> bool {
        self.raw_position(value).1.is_ok()
    }
    
    /// Set の `idx` 番目に小さい値の参照を返す。存在しない場合は `None` を返す。
    pub fn nth(&self, mut idx: usize) -> Option<&T> {
        if self._len <= idx { return None; }
        for v in &self.buckets {
            if v.len() <= idx { idx -= v.len(); } else { return Some(&v[idx]); }
        }
        unreachable!()
    }
    
    /// Set の `idx` 番目に大きい値の参照を返す。存在しない場合は `None` を返す。
    pub fn nth_back(&self, idx: usize) -> Option<&T> {
        if self._len <= idx { None } else { self.nth(self._len - idx - 1) }
    }
    
    /// Set 内の `value` より小さい値の個数を返す。`value` が存在するならば `Ok(usize)` で、そうでないならば `Err(usize)` で返す。  
    /// 特に、`Ok(i)` ならば `set[i-1] < set[i] == value < set[i+1]` であり、
    /// `Err(i)` ならば `set[i-1] < value < set[i]` である。
    pub fn position(&self, value: &T) -> Result<usize, usize> {
        let (bidx, idx) = self.raw_position(value);
        let offset: usize = (0..bidx).map(|i| self.buckets[i].len()).sum();
        match idx { Ok(v) => Ok(v+offset), Err(v) => Err(v+offset) }
    }
    
    /// 先頭の要素の参照を返す。Set が空ならば `None` を返す。
    pub fn first(&self) -> Option<&T> {
        (self._len != 0).then(|| &self.buckets[0][0])
    }
    
    /// 末尾の要素の参照を返す。Set が空ならば `None` を返す。
    pub fn last(&self) -> Option<&T> {
        (self._len != 0).then(|| self.buckets.last().unwrap().last().unwrap())
    }
    
    pub fn iter(&self) -> Iter<T> { self.into() }
    
    pub fn into_vec(self) -> Vec<T> {
        let mut out = Vec::with_capacity(self._len);
        for v in self.buckets { out.extend(v); }
        out
    }
}

impl<T: Ord + Clone> From<Vec<T>> for SqrtSet<T> {
    fn from(mut value: Vec<T>) -> Self {
        if value.is_empty() { return SqrtSet::new(); }
        value.sort(); value.dedup();
        let len = value.len();
        let bucket_len = (len * Self::BUCKET_RATIO).sqrt();
        let buckets = value.chunks(bucket_len).map(|sl| sl.to_vec()).collect();
        SqrtSet { buckets, _len: len }
    }
}

impl<T: Ord + Clone, const N: usize> From<[T; N]> for SqrtSet<T> {
    fn from(value: [T; N]) -> Self { SqrtSet::from(value.to_vec()) }
}

impl<T: Ord + Debug> Debug for SqrtSet<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.buckets) } }

impl<'a, T: Ord> IntoIterator for &'a SqrtSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.into() }
}


#[derive(Clone)]
pub struct Iter<'a, T: Ord> { set: &'a SqrtSet<T>, l: (usize, usize), r: (usize, usize), remain: usize }

impl<'a, T: Ord> From<&'a SqrtSet<T>> for Iter<'a, T> {
    fn from(value: &'a SqrtSet<T>) -> Self {
        Iter { set: value, l: (0, 0), r: (value.buckets.len(), 0), remain: value._len }
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remain == 0 { return None; }
        let out = &self.set.buckets[self.l.0][self.l.1];
        self.l.1 += 1;
        if self.l.1 == self.set.buckets[self.l.0].len() { self.l = (self.l.0+1, 0); }
        self.remain -= 1;
        Some(out)
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remain, Some(self.remain))
    }
}

impl<'a, T: Ord> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remain == 0 { return None; }
        if self.r.1 == 0 { self.r = (self.r.0-1, self.set.buckets[self.r.0-1].len()); }
        self.r.1 -= 1;
        self.remain -= 1;
        Some(&self.set.buckets[self.r.0][self.r.1])
    }
}

impl<'a, T: Ord> ExactSizeIterator for Iter<'a, T> {}
