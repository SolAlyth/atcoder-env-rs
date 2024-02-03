//! ナンバリング・座標圧縮

use std::{collections::HashMap, hash::Hash};



/// 見かけた順にナンバリングをする構造体。
///
/// # Usage
///
/// ```
/// use lib::data_struct::compress::Numbering;
///
/// let mut numb = Numbering::new();
/// let key = 10;
///
/// let unique_value = numb.entry(&key);
/// assert!(unique_value == numb.entry(&key));
///
/// assert!(key == *numb.get(unique_value));
/// ```

pub struct Numbering<T: Eq + Hash + Clone> {
    map: HashMap<T, usize>, vec: Vec<T>
}

impl<T: Eq + Hash + Clone> Numbering<T> {
    pub fn new() -> Self { Numbering { map: HashMap::new(), vec: vec![] } }
    
    /// `key` を見かけた順に対応した自然数を返します。
    pub fn entry(&mut self, key: &T) -> usize {
        if !self.map.contains_key(key) {
            self.map.insert(key.clone(), self.vec.len());
            self.vec.push(key.clone());
        }
        self.map[key]
    }
    
    /// `index` に対応した、元の `key` を返します。
    pub fn get(&self, index: usize) -> &T {
        &self.vec[index]
    }
}



/// 順序を保ちながらナンバリングする構造体。
///
/// いわゆる座標圧縮。
///
/// # Usage
///
/// ```
/// use lib::data_struct::compress::{Compress, Compressed};
///
/// let mut comp = Compress::new();
/// comp.insert_with([0, 1, 3, 3, 5].into_iter());
///
/// let comp: Compressed<_> = comp.calc();
/// assert!(comp.entry(&3) == 2);
/// assert!(*comp.get(2) == 3);
/// ```

pub struct Compress<T: Ord> (Vec<T>);

impl<T: Ord> Compress<T> {
    pub fn new() -> Self { Compress(vec![]) }
    
    pub fn insert(&mut self, key: T) {
        self.0.push(key);
    }
    
    pub fn insert_with(&mut self, iter: impl Iterator<Item = T>) {
        for key in iter { self.insert(key); }
    }
    
    pub fn calc(mut self) -> Compressed<T> {
        self.0.sort_unstable();
        self.0.dedup();
        Compressed(self.0)
    }
}


/// [`Compress`] が計算されたことを表す構造体。
///
/// # Usage
///
/// [`Compress`] 参照
pub struct Compressed<T: Ord> (Vec<T>);

impl<T: Ord> Compressed<T> {
    pub fn entry(&self, key: &T) -> usize {
        self.0.binary_search(key).unwrap()
    }
    
    pub fn get(&self, idx: usize) -> &T {
        &self.0[idx]
    }
}
