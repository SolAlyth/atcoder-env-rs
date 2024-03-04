//! ナンバリング・座標圧縮

use std::{collections::HashMap, hash::Hash};



/// 見かけた順にナンバリングをする構造体。
/// 
/// 値は `impl Eq + Hash + Clone` が必要。
pub struct Numbering<T: Eq + Hash + Clone> {
    map: HashMap<T, usize>, vec: Vec<T>
}

impl<T: Eq + Hash + Clone> Numbering<T> {
    pub fn new() -> Self { Numbering { map: HashMap::new(), vec: vec![] } }
    
    /// `value` を見かけた順に、`0` から自然数 `key` を返す。
    pub fn entry(&mut self, value: &T) -> usize {
        if !self.map.contains_key(value) {
            self.map.insert(value.clone(), self.vec.len());
            self.vec.push(value.clone());
        }
        self.map[value]
    }
    
    /// `key` に対応した、元の `value` を返す。
    pub fn restore(&self, key: usize) -> &T {
        &self.vec[key]
    }
}



/// 座標圧縮、つまり順序を保ちながらナンバリングする構造体。
/// 
/// `calc`: `O(NlogN)`
pub struct Compress<T: Ord> (Vec<T>);

impl<T: Ord> Compress<T> {
    pub fn new() -> Self { Compress(vec![]) }
    pub fn entry(&mut self, value: T) { self.0.push(value); }
    pub fn calc(mut self) -> Compressed<T> { self.0.sort_unstable(); self.0.dedup(); Compressed(self.0) }
}


/// [`Compress`] が計算されたことを表す構造体。
pub struct Compressed<T: Ord> (Vec<T>);

impl<T: Ord> Compressed<T> {
    pub fn len(&self) -> usize { self.0.len() }
    
    pub fn key(&self, value: &T) -> usize {
        self.0.binary_search(value).unwrap()
    }
    
    pub fn restore(&self, key: usize) -> &T {
        &self.0[key]
    }
    
    pub fn inner(&self) -> &Vec<T> { &self.0 }
}
