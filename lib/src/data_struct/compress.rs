use std::{collections::HashMap, hash::Hash, ops::Index};

/// # Usage
///
/// ```
/// let mut numbering = Numbering::new();
///
/// let unique_index = numbering[&key];
/// let unique_index = numbering.entry(&key);
///
/// let key = numbering[&unique_index]
/// ```
pub struct Numbering<T: Eq + Hash + Clone> {
    map: HashMap<T, usize>, vec: Vec<T>
}

impl<T: Eq + Hash + Clone> Numbering<T> {
    pub fn new() -> Self { Numbering { map: HashMap::new(), vec: vec![] } }
    
    /// `key` に対応した、被らない整数を返します。
    ///
    /// `self.entry(&key)` は `self[&key]` と等価です。
    ///
    /// # Memo
    /// 不必要に `key.clone()` を呼び出しているので、下のコードは却下されました。
    /// ```
    /// *self.map.entry(key.clone()).or_insert_with(|| { self.vec.push(key.clone()); self.vec.len()-1 })
    /// ```
    pub fn entry(&mut self, key: &T) -> usize {
        if self.map.contains_key(key) {
            self.map.insert(key.clone(), self.vec.len());
            self.vec.push(key.clone());
        }
        self.map[key]
    }
    
    /// `index` に対応した、元の `key` を返します。
    ///
    /// `self.get(index) は `self[index]` と等価です。
    pub fn get(&self, index: usize) -> &T {
        &self.vec[index]
    }
}

impl<T: Eq + Hash + Clone> Index<usize> for Numbering<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.get(index) }
}

impl<T: Eq + Hash + Clone> Index<&T> for Numbering<T> {
    type Output = usize;
    fn index(&self, key: &T) -> &Self::Output { &self.map[key] }
}




pub struct Compress<T: PartialOrd + Clone> {
    vec: Vec<T>, sorted: bool
}

impl<T: PartialOrd + Clone> Compress<T> {
    pub fn new() -> Self { Compress { vec: vec![], sorted: false } }
    
    pub fn insert(&mut self, key: &T) {
        assert!(!self.sorted, "fn Compress.insert は !Compress.sorted であるときでしか呼び出せません。");
        self.vec.push(key.clone());
    }
    
    pub fn calc(&mut self) {
        assert!(!self.sorted, "fn Compress.calc は一回しか呼び出せません。");
        self.sorted = true;
        
        self.vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        self.vec.dedup();
    }
    
    pub fn get(&self, key: &T) -> usize {
        assert!(self.sorted, "fn Compress.get は Compress.sorted であるときでしか呼び出せません。fn Compress.calc を呼び出してください。");
        self.vec.binary_search_by(|v| v.partial_cmp(key).unwrap()).unwrap()
    }
}

impl<T: PartialOrd + Clone> Index<usize> for Compress<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.vec[index] }
}
