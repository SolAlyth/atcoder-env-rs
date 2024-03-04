use std::collections::{BTreeMap, HashMap};

pub struct BTreeMultiSet<T: Ord + Clone> {
    inner: BTreeMap<T, usize>
}

impl<T: Ord + Clone> BTreeMultiSet<T> {
    pub fn new() -> Self {
        BTreeMultiSet { inner: BTreeMap::new() }
    }
    
    pub fn clear(&mut self) { self.inner.clear(); }
    
    pub fn contains(&self, key: &T) -> bool { self.inner.contains_key(&key) }
    
    pub fn first(&self) -> Option<&T> { self.inner.first_key_value().map(|v| v.0) }
    
    pub fn last(&self) -> Option<&T> { self.inner.last_key_value().map(|v| v.0) }
    
    pub fn pop_first(&mut self) -> Option<T> {
        let Some(entry) = self.inner.first_entry() else { return None; };
    }
}
