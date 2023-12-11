use std::mem::swap;


#[derive(Clone, Copy)]
pub enum Item { Leader(usize), Child(usize) }

pub struct UnionFindCore<T, U> {
    data: Vec<Item>, pub value: Vec<T>,
    cmpf: fn(&mut [T], usize, usize, usize),
    mgf: fn(&mut [T], usize, usize, usize, usize, U) -> bool,
    invf: fn(&mut U)
}

impl<T, U> UnionFindCore<T, U> {
    fn compress(&mut self, child: usize, parent_new: usize) {
        let Item::Child(parent_old) = self.data[child] else { assert_eq!(child, parent_new); return; };
        
        // parent > child のとき
        if parent_old == parent_new { return; }
        
        // parent_new > ... > parent_old > child のとき
        // parent_new > parent_old > child にしてから処理
        self.compress(parent_old, parent_new);
        self.data[child] = Item::Child(parent_new);
        (self.cmpf)(&mut self.value, child, parent_old, parent_new);
    }
    
    fn leader_and_size(&mut self, u: usize) -> (usize, usize) {
        match self.data[u] {
            Item::Leader(size) => { (u, size) }
            Item::Child(par) => {
                let (leader, size) = self.leader_and_size(par);
                self.compress(u, leader);
                (leader, size)
            }
        }
    }
    
    pub fn merge(&mut self, mut child: usize, mut parent: usize, mut arg: U) -> Result<bool, ()> {
        if self.size(parent) < self.size(child) { swap(&mut parent, &mut child); (self.invf)(&mut arg); }
        
        let ((cl, cs), (pl, ps)) = (self.leader_and_size(child), self.leader_and_size(parent));
        let res = (self.mgf)(&mut self.value, child, cl, parent, pl, arg);
        if res {
            if cl != pl {
                self.data[pl] = Item::Leader(ps+cs);
                self.data[cl] = Item::Child(pl);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(())
        }
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
    pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
    pub fn len(&self) -> usize { self.data.len() }
    
    pub fn group(&mut self, u: usize) -> Vec<usize> { (0..self.len()).filter(|&v| self.is_same(u, v)).collect() }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut out = vec![vec![]; self.len()];
        for u in 0..self.len() { out[self.leader(u)].push(u); }
        out.dedup(); out
    }
}

pub struct UnionFind {
    inner: UnionFindCore<(), ()>
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            inner: UnionFindCore {
                data: vec![Item::Leader(1); size], value: vec![(); size],
                cmpf: |_: &mut _, _, _, _| {},
                mgf: |_: &mut _, _, _, _, _, _| true,
                invf: |_: &mut _| {}
            }
        }
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.inner.leader(u) }
    pub fn size(&mut self, u: usize) -> usize { self.inner.size(u) }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.inner.is_same(u, v) }
    pub fn len(&self) -> usize { self.inner.len() }
    pub fn group(&mut self, u: usize) -> Vec<usize> { self.inner.group(u) }
    pub fn groups(&mut self) -> Vec<Vec<usize>> { self.inner.groups() }
    
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        self.inner.merge(u, v, ()).unwrap()
    }
    
}



type WeightType = i128;

pub struct WeightedUnionFind {
    inner: UnionFindCore<WeightType, WeightType>
}

impl WeightedUnionFind {
    pub fn new(size: usize) -> Self {
        WeightedUnionFind {
            inner: UnionFindCore {
                data: vec![Item::Leader(1); size], value: vec![0; size],
                cmpf: |value: &mut [WeightType], child, parent_old, _| { value[child] += value[parent_old]; },
                mgf: |value: &mut [WeightType], c, cl, p, pl, arg| {
                    if cl != pl { value[cl] = arg + value[p] - value[c]; true } else { value[p] - value[c] == arg }
                },
                invf: |arg: &mut _| { *arg *= -1; }
            }
        }
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.inner.leader(u) }
    pub fn size(&mut self, u: usize) -> usize { self.inner.size(u) }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.inner.is_same(u, v) }
    pub fn len(&self) -> usize { self.inner.len() }
    pub fn group(&mut self, u: usize) -> Vec<usize> { self.inner.group(u) }
    pub fn groups(&mut self) -> Vec<Vec<usize>> { self.inner.groups() }
    
    pub fn merge(&mut self, child: usize, parent: usize, arg: WeightType) -> Result<bool, ()> { self.inner.merge(child, parent, arg) }
    pub fn dist(&mut self, child: usize, parent: usize) -> Option<WeightType> { if self.is_same(child, parent) { Some(self.inner.value[child] - self.inner.value[parent]) } else { None } }
}
