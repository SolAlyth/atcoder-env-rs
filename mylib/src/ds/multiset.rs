use std::{collections::{BTreeMap, HashMap}, hash::Hash, ops::RangeBounds, ptr::eq as ptr_eq};

pub use {
    btree_multi_set::BTreeMultiSet,
    hash_multi_set::HashMultiSet
};

/// 長さ `len` のブロック `value` の `i` 番目であることを表す。
pub struct BlockItem<'a, V> {
    pub value: &'a V,
    pub len: usize,
    pub idx: usize,
}

mod btree_multi_set {
    use super::*;
    use std::collections::btree_map::Iter as BIter;
    
    // PartialOrd, Ord は要らない？
    
    /// Multi な `BTreeSet`
    /// 
    /// implemented: `FromIterator<V>`
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BTreeMultiSet<V: Ord + Clone> {
        inner: BTreeMap<V, usize>,
        len: usize
    }
    
    impl<V: Ord + Clone> BTreeMultiSet<V> {
        // implement for both BTree and Hash
        pub fn clear(&mut self) { self.inner.clear(); self.len = 0; }
        pub fn contains(&self, value: &V) -> bool { self.inner.contains_key(&value) }
        /// `value` を `n` 個追加する。新規追加するとき `V::clone` が発生する。
        pub fn insert(&mut self, value: &V, n: usize) { self.modify(value, |befn| befn+n); }
        pub fn is_empty(&self) -> bool { self.len == 0 }
        /// ブロック値の和を返す。`{a: 3, b: 4} => 7`
        pub fn len(&self) -> usize { self.len }
        /// `value` を `n` 個削除する。成功したら `true` を返す。
        /// 
        /// - `strict = true` のとき、`0..n` 個ならば削除せず、失敗とする。
        /// - `strict = false` のとき、`1..n` 個ならば全て削除し、成功とする。`0` 個のときは、失敗とする。
        pub fn remove(&mut self, value: &V, n: usize, strict: bool) -> bool {
            let mut ret = true;
            self.modify(value, |befn| { if strict && befn < n { ret = false; befn } else { befn.saturating_sub(n) } });
            ret
        }
        pub fn remove_block(&mut self, value: &V) -> Option<usize> {
            let mut ret = None;
            self.modify(value, |n| { if n != 0 { ret = Some(n); } 0 });
            ret
        }
        
        /// ブロック数を返す。`{a: 3, b: 4} => 2`
        pub fn len_blocks(&self) -> usize { self.inner.len() }
        /// ブロック `value` の値を返す。
        pub fn len_block(&self, value: &V) -> usize { *self.inner.get(value).unwrap_or(&0) }
        /// ブロックの値を取得・変更できる。新規追加するとき `value.clone()` が発生する。
        pub fn modify(&mut self, value: &V, f: impl FnOnce(usize) -> usize) {
            if let Some(n) = self.inner.get_mut(value) {
                self.len -= *n; *n = f(*n); self.len += *n;
                if *n == 0 { self.inner.remove(value); }
            } else {
                let n = f(0);
                if n != 0 { self.inner.insert(value.clone(), n); self.len += n; }
            }
        }
        
        
        // implement only for BTree
        pub fn iter(&self) -> Iter<V> { Iter::new(self) }
        pub fn iter_blocks(&self) -> impl Iterator<Item = (&V, usize)> + DoubleEndedIterator { self.inner.iter().map(|(v, &n)| (v, n)) }
        pub fn new() -> Self { Self { inner: BTreeMap::new(), len: 0 } }
        pub fn first(&self) -> Option<(&V, usize)> { self.inner.first_key_value().map(|v| (v.0, *v.1)) }
        pub fn last(&self) -> Option<(&V, usize)> { self.inner.last_key_value().map(|v| (v.0, *v.1)) }
        /// 先頭のブロックから一つ削除して取り出す。ただし、`V::clone()` が発生する。
        pub fn pop_first(&mut self) -> Option<V> {
            let Some(mut entry) = self.inner.first_entry() else { return None; };
            let (v, &n) = (entry.key().clone(), entry.get());
            if n == 1 { entry.remove(); } else { entry.insert(n-1); }
            self.len -= 1;
            Some(v)
        }
        /// 末尾のブロックから一つ削除して取り出す。ただし、`V::clone()` が発生する。
        pub fn pop_last(&mut self) -> Option<V> {
            let Some(mut entry) = self.inner.last_entry() else { return None; };
            let (v, &n) = (entry.key().clone(), entry.get());
            if n == 1 { entry.remove(); } else { entry.insert(n-1); }
            self.len -= 1;
            Some(v)
        }
        pub fn range_blocks(&self, range: impl RangeBounds<V>) -> impl Iterator<Item = (&V, usize)> + DoubleEndedIterator { self.inner.range(range).map(|(v, &n)| (v, n)) }
        
        pub fn pop_first_block(&mut self) -> Option<(V, usize)> { if let Some(v) = self.inner.pop_first() { self.len -= v.1; Some(v) } else { None } }
        pub fn pop_last_block(&mut self) -> Option<(V, usize)> { if let Some(v) = self.inner.pop_last() { self.len -= v.1; Some(v) } else { None } }
    }
    
    impl<V: Ord + Clone> FromIterator<V> for BTreeMultiSet<V> {
        fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
            let mut res = Self::new();
            for v in iter { res.insert(&v, 1); }
            res
        }
    }
    
    pub enum Iter<'a, V> {
        Empty,
        Some {
            src: BIter<'a, V, usize>,
            f: (&'a V, &'a usize),
            b: (&'a V, &'a usize),
            fidx: usize,
            bidx: usize
        }
    }
    
    impl<'a, V: Ord + Clone> Iter<'a, V> {
        fn new(src: &'a BTreeMultiSet<V>) -> Self {
            if src.is_empty() { return Self::Empty; }
            let mut src = src.inner.iter();
            let f = src.next().unwrap();
            let b = src.next_back().unwrap_or(f);
            Self::Some { src, f, b, fidx: 0, bidx: *b.1 }
        }
    }
    
    impl<'a, V> Iterator for Iter<'a, V> {
        type Item = BlockItem<'a, V>;
        fn next(&mut self) -> Option<Self::Item> {
            let Self::Some { src, f, b, fidx, bidx } = self else { return None; };
            let res = BlockItem { value: f.0, len: *f.1, idx: *fidx };
            *fidx += 1;
            if ptr_eq(f.0, b.0) && fidx == bidx { *self = Self::Empty; return Some(res); }
            if fidx == f.1 { *f = src.next().unwrap_or(*b); *fidx = 0; }
            Some(res)
        }
    }
    
    impl<'a, V> DoubleEndedIterator for Iter<'a, V> {
        fn next_back(&mut self) -> Option<Self::Item> {
            let Self::Some { src, f, b, fidx, bidx } = self else { return None; };
            *bidx -= 1;
            let res = BlockItem { value: b.0, len: *b.1, idx: *bidx };
            if ptr_eq(f.0, b.0) && fidx == bidx { *self = Self::Empty; return Some(res); }
            if *bidx == 0 { *b = src.next().unwrap_or(*f); *bidx = *b.1; }
            Some(res)
        }
    }
}



mod hash_multi_set {
    use super::*;
    use std::collections::hash_map::Iter as HIter;
    
    /// Multi な `HashSet`
    /// 
    /// implemented: `FromIterator<V>`
    pub struct HashMultiSet<V: Clone + Hash + Eq> {
        inner: HashMap<V, usize>,
        len: usize
    }
    
    impl<V: Clone + Hash + Eq> HashMultiSet<V> {
        // implement for both BTree and Hash
        pub fn clear(&mut self) { self.inner.clear(); self.len = 0; }
        pub fn contains(&self, value: &V) -> bool { self.inner.contains_key(&value) }
        /// `value` を `n` 個追加する。新規追加するとき `V::clone` が発生する。
        pub fn insert(&mut self, value: &V, n: usize) { self.modify(value, |befn| befn+n); }
        pub fn is_empty(&self) -> bool { self.len == 0 }
        /// ブロック値の和を返す。`{a: 3, b: 4} => 7`
        pub fn len(&self) -> usize { self.len }
        /// `value` を `n` 個削除する。成功したら `true` を返す。
        /// 
        /// - `strict = true` のとき、`0..n` 個ならば削除せず、失敗とする。
        /// - `strict = false` のとき、`1..n` 個ならば全て削除し、成功とする。`0` 個のときは、失敗とする。
        pub fn remove(&mut self, value: &V, n: usize, strict: bool) -> bool {
            let mut ret = true;
            self.modify(value, |befn| { if strict && befn < n { ret = false; befn } else { befn.saturating_sub(n) } });
            ret
        }
        pub fn remove_block(&mut self, value: &V) -> Option<usize> {
            let mut ret = None;
            self.modify(value, |n| { if n != 0 { ret = Some(n); } 0 });
            ret
        }
        
        /// ブロック数を返す。`{a: 3, b: 4} => 2`
        pub fn len_blocks(&self) -> usize { self.inner.len() }
        /// ブロック `value` の値を返す。
        pub fn len_block(&self, value: &V) -> usize { *self.inner.get(value).unwrap_or(&0) }
        /// ブロックの値を取得・変更できる。新規追加するとき `value.clone()` が発生する。
        pub fn modify(&mut self, value: &V, f: impl FnOnce(usize) -> usize) {
            if let Some(n) = self.inner.get_mut(value) {
                self.len -= *n; *n = f(*n); self.len += *n;
                if *n == 0 { self.inner.remove(value); }
            } else {
                let n = f(0);
                if n != 0 { self.inner.insert(value.clone(), n); self.len += n; }
            }
        }
        
        
        // implement only for Hash
        pub fn iter(&self) -> Iter<V> { Iter::new(self) }
        pub fn iter_blocks(&self) -> impl Iterator<Item = (&V, usize)> { self.inner.iter().map(|(v, &n)| (v, n)) }
        pub fn new() -> Self { Self { inner: HashMap::new(), len: 0} }
    }
    
    impl<V: Clone + Hash + Eq> FromIterator<V> for HashMultiSet<V> {
        fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
            let mut res = Self::new();
            for v in iter { res.insert(&v, 1); }
            res
        }
    }
    
    pub enum Iter<'a, V> {
        Empty,
        Some {
            src: HIter<'a, V, usize>,
            f: (&'a V, &'a usize),
            fidx: usize,
        }
    }
    
    impl<'a, V: Clone + Hash + Eq> Iter<'a, V> {
        fn new(src: &'a HashMultiSet<V>) -> Self {
            if src.is_empty() { return Self::Empty; }
            let mut src = src.inner.iter();
            let f = src.next().unwrap();
            Self::Some { src, f, fidx: 0 }
        }
    }
    
    impl<'a, V> Iterator for Iter<'a, V> {
        type Item = BlockItem<'a, V>;
        fn next(&mut self) -> Option<Self::Item> {
            let Self::Some { src, f, fidx } = self else { return None; };
            let res = BlockItem { value: f.0, len: *f.1, idx: *fidx };
            *fidx += 1;
            if fidx == f.1 {
                if let Some(tmp) = src.next() { *f = tmp; *fidx = 0; } else { *self = Self::Empty; return Some(res); }
            }
            Some(res)
        }
    }
}
