#[derive(Clone, Debug)]
pub struct InversibleVec {
    kv: Vec<Option<usize>>,
    vk: Vec<Option<usize>>
}

impl InversibleVec {
    pub fn new(len: usize) -> Self {
        InversibleVec { kv: vec![None; len], vk: vec![None; len] }
    }
    
    pub fn set(&mut self, key: usize, value: usize) {
        assert!(self.kv[key].is_none() && self.vk[value].is_none());
        self.kv[key] = Some(value); self.vk[value] = Some(key);
    }
    
    pub fn remove_by_key(&mut self, key: usize) -> bool {
        let Some(value) = self.kv[key] else { return false; };
        self.kv[key] = None; self.vk[value] = None;
        true
    }
    
    pub fn remove_by_value(&mut self, value: usize) -> bool {
        let Some(key) = self.vk[value] else { return false; };
        self.kv[key] = None; self.vk[value] = None;
        true
    }
    
    pub fn value(&self, key: usize) -> Option<usize> { self.kv[key] }
    pub fn key(&self, value: usize) -> Option<usize> { self.vk[value] }
    
    pub fn contains_value(&self, key: usize) -> bool { self.kv[key].is_some() }
    pub fn contains_key(&self, value: usize) -> bool { self.vk[value].is_some() }
    
    pub fn get_inner_kv(&self) -> &Vec<Option<usize>> { &self.kv }
    pub fn get_inner_vk(&self) -> &Vec<Option<usize>> { &self.vk }
}
