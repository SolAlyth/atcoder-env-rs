//! 座標圧縮

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
    
    /// 圧縮された `key` を返します。
    pub fn key(&self, value: &T) -> usize { self.0.binary_search(value).unwrap() }
    
    /// 圧縮前の `value` を復元します。
    pub fn restore(&self, key: usize) -> &T { &self.0[key] }
    
    /// ソートされた `value` の列を返します。
    pub fn inner(&self) -> &Vec<T> { &self.0 }
}
