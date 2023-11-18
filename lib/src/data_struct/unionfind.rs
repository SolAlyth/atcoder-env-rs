use {
    itertools::Itertools,
    std::mem::swap
};


#[derive(Clone, Copy)]
pub(crate) enum Item {
    Parent(usize), Size(usize)
}

/// ふつうの UnionFind
///
/// # Reference
///
/// - [nekolib by rsk0315](https://rsk0315.github.io/library-rs/nekolib/ds/struct.UnionFind.html)
pub struct UnionFind {
    pub(crate) data: Vec<Item>
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self { data: vec![Item::Size(1); size] }
    }
    
    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.parent_and_size(u).0 == self.parent_and_size(v).0
    }
    
    fn parent_and_size(&mut self, i: usize) -> (usize, usize) {
        match self.data[i] {
            Item::Size(size) => { (i, size) }
            Item::Parent(par) => {
                let (parpar, pars) = self.parent_and_size(par);
                if par != parpar {
                    self.data[i] = Item::Parent(parpar);
                }
                (parpar, pars)
            }
        }
    }
    
    pub(crate) fn parent_and_size_with(&mut self, i: usize, mergef: &mut impl FnMut(usize, usize)) -> (usize, usize) {
        match self.data[i] {
            Item::Size(size) => { (i, size) }
            Item::Parent(par) => {
                let (parpar, pars) = self.parent_and_size_with(par, mergef);
                if par != parpar {
                    self.data[i] = Item::Parent(parpar);
                    mergef(i, par);
                }
                (parpar, pars)
            }
        }
    }
    
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let (u, us) = self.parent_and_size(u);
        let (v, vs) = self.parent_and_size(v);
        if u == v { return false; }
        let (child, par) = if us <= vs { (u, v) } else { (v, u) };
        self.data[par] = Item::Size(us + vs);
        self.data[child] = Item::Parent(par);
        true
    }
    
    pub fn size(&mut self, i: usize) -> usize { self.parent_and_size(i).1 }
    
    /// u が属する集合を返す
    ///
    /// 計算量: O(n)
    pub fn subset(&mut self, i: usize) -> Vec<usize> {
        (0..self.data.len()).filter(|&v| self.is_same(i, v)).collect_vec()
    }
    
    /// 集合族を返す
    ///
    /// 計算量: O(n)
    pub fn partition(&mut self) -> Vec<Vec<usize>> {
        let mut out = crate::nest!(void; self.data.len());
        for i in 0..self.data.len() {
            out[self.parent_and_size(i).0].push(i);
        }
        out.retain(|v| v.len() != 0);
        out
    }
}



type WeightType = i128;

/// 重み付き UnionFind
///
/// 集合 `G` に対して `Weight(i) = ` $`\sum_{p \leq i}`$ `diff(p)` が定義できる。
pub struct WeightedUnionFind {
    uf: UnionFind,
    /// weight\[i] = 0 (if i is parent) \
    /// weight\[i] = weight\[parent\[i]] + diff\[i] (otherwise)
    diff: Vec<WeightType>
}

impl WeightedUnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            uf: UnionFind::new(size),
            diff: vec![0; size]
        }
    }
    
    fn parent_and_size(&mut self, i: usize) -> (usize, usize) {
        let mut weight_merge = |child, par| {
            // parpar - par - child になっていたものを
            // parpar - {par, child} にする
            self.diff[child] += self.diff[par];
        };
        self.uf.parent_and_size_with(i, &mut weight_merge)
    }
    
    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.parent_and_size(u).0 == self.parent_and_size(v).0
    }
    
    pub fn merge(&mut self, u: usize, v: usize, mut weight: WeightType) -> Result<bool, ()> {
        let (mut child, childs) = self.parent_and_size(u);
        let (mut par, pars) = self.parent_and_size(v);
        
        if child == par { return if self.diff[v] - self.diff[u] == weight { Ok(false) } else { Err(()) } }
        
        // child(u_par) -> u -> v -> par(v_par) の重み
        weight = -self.diff[u] + weight + self.diff[v];
        
        if childs > pars {
            swap(&mut child, &mut par);
            weight *= -1;
        }
        
        self.uf.data[child] = Item::Parent(par);
        self.uf.data[par] = Item::Size(childs + pars);
        self.diff[child] = weight;
        
        Ok(true)
    }
    
    pub fn dist(&mut self, u: usize, v: usize) -> Result<WeightType, ()> {
        if self.is_same(u, v) {
            Ok(self.diff[u] - self.diff[v])
        } else {
            Err(())
        }
    }
    
    pub fn size(&mut self, i: usize) -> usize {
        self.parent_and_size(i).1
    }
    
    /// u が属する集合を返す
    ///
    /// 計算量: O(n)
    pub fn subset(&mut self, i: usize) -> Vec<usize> {
        self.uf.subset(i)
    }
    
    /// 集合族を返す
    ///
    /// 計算量: O(n)
    pub fn partition(&mut self) -> Vec<Vec<usize>> {
        self.uf.partition()
    }
}


/* pub struct WeightedUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<i128>
}

impl WeightedUnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect_vec(),
            rank: vec![0; size],
            weight: vec![0; size]
        }
    }
    
    fn root(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            idx
        } else {
            let root = self.root(self.parent[idx]);
            self.weight[idx] += self.weight[self.parent[idx]];
            self.parent[idx] = root;
            root
        }
    }
    
    fn weight(&mut self, idx: usize) -> i128 {
        self.root(idx);
        self.weight[idx]
    }
    
    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
    
    pub fn merge(&mut self, mut u: usize, mut v: usize, mut weight: i128) -> bool {
        weight += self.weight(u);
        weight -= self.weight(v);
        u = self.parent[u]; v = self.parent[v];
        if u == v { return false; }
        if self.rank[u] < self.rank[v] { swap(&mut u, &mut v); weight *= -1; }
        if self.rank[u] == self.rank[v] { self.rank[u] += 1; }
        self.parent[v] = u;
        self.weight[v] = weight;
        true
    }
    
    pub fn diff(&mut self, u: usize, v: usize) -> i128 {
        assert!(self.is_same(u, v));
        self.weight(v) - self.weight(u)
    }
} */



/* pub struct UnionFind(WeightedUnionFind);

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind(WeightedUnionFind::new(size))
    }
    
    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.0.is_same(u, v)
    }
    
    pub fn merge(&mut self, u: usize, v: usize) {
        self.0.merge(u, v, 0);
    }
} */
