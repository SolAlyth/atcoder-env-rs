//! ナンバリング・座標圧縮

use std::{collections::HashMap, hash::Hash};



/// 見かけた順にナンバリングをする構造体。
/// 
/// 値は `impl Eq + Hash + Clone` が必要。
/// 
/// 全ての操作が `O(1)`
///
/// # Usage
///
/// ```
/// use crate::mylib::data_struct::compress::Numbering;
///
/// let mut numb = Numbering::new();
/// let values = [3, 1, 2, 3];
/// let mut keys = vec![];
/// 
/// for value in &values { keys.push(numb.entry(value)); }
/// assert_eq!(keys, [0, 1, 2, 0]);
/// 
/// let mut restore = vec![];
/// for &key in &keys { restore.push(*numb.restore(key)); }
/// assert_eq!(restore, values);
/// ```

pub struct Numbering<T: Eq + Hash + Clone> {
    map: HashMap<T, usize>, vec: Vec<T>
}

impl<T: Eq + Hash + Clone> Numbering<T> {
    pub fn new() -> Self { Numbering { map: HashMap::new(), vec: vec![] } }
    
    /// `value` を見かけた順に、`0` から自然数 `key` を返します。
    pub fn entry(&mut self, value: &T) -> usize {
        if !self.map.contains_key(value) {
            self.map.insert(value.clone(), self.vec.len());
            self.vec.push(value.clone());
        }
        self.map[value]
    }
    
    /// `key` に対応した、元の `value` を返します。
    pub fn restore(&self, key: usize) -> &T {
        &self.vec[key]
    }
}



/// 順序を保ちながらナンバリングする構造体。
///
/// いわゆる座標圧縮。
/// 
/// `entry`: `O(1)`, `calc`: `O(NlogN)`
///
/// # Usage
///
/// ```
/// use crate::mylib::data_struct::compress::{Compress, Compressed};
///
/// let mut comp = Compress::new();
/// let values = [5, 2, 4, 1, 3, 2];
/// for &value in &values { comp.entry(value); }
///
/// let comp: Compressed<_> = comp.calc();
/// 
/// let key = comp.key(&3); // 3 is placed in 2 of [1,2,3,4,5]
/// assert_eq!(key, 2);
/// assert_eq!(*comp.restore(key), 3);
/// ```

pub struct Compress<T: Ord> (Vec<T>);

impl<T: Ord> Compress<T> {
    pub fn new() -> Self { Compress(vec![]) }
    
    pub fn entry(&mut self, value: T) {
        self.0.push(value);
    }
    
    pub fn calc(mut self) -> Compressed<T> {
        self.0.sort_unstable();
        self.0.dedup();
        Compressed(self.0)
    }
}


/// [`Compress`] が計算されたことを表す構造体。
pub struct Compressed<T: Ord> (Vec<T>);

impl<T: Ord> Compressed<T> {
    pub fn key(&self, value: &T) -> usize {
        self.0.binary_search(value).unwrap()
    }
    
    pub fn restore(&self, key: usize) -> &T {
        &self.0[key]
    }
}
