/// Potential 付き Union Find (union by size, path compression)
pub struct UnionFind<Op: UnionFindOp> {
    par: Vec<usize>,
    size: Vec<usize>,
    next: Vec<usize>,
    diff: Vec<Op::T>
}

impl<Op: UnionFindOp> Clone for UnionFind<Op> {
    fn clone(&self) -> Self {
        Self { par: self.par.clone(), size: self.size.clone(), next: self.next.clone(), diff: self.diff.clone() }
    }
}



/// 可換群をなすポテンシャルが乗る。
pub trait UnionFindOp {
    type T: Clone + PartialEq + std::fmt::Debug;
    fn e() -> Self::T;
    fn add(a: &Self::T, b: &Self::T) -> Self::T;
    fn inv(a: &Self::T) -> Self::T;
    
    #[doc(hidden)]
    fn sub(a: &Self::T, b: &Self::T) -> Self::T { Self::add(a, &Self::inv(b)) }
}

pub struct Nop;
impl UnionFindOp for Nop { type T = (); fn e() {} fn add(_: &(), _: &()) {} fn inv(_: &()) { } }

impl UnionFind<Nop> {
    pub fn new_nop(len: usize) -> Self { Self::new(len) }
}



impl<Op: UnionFindOp> UnionFind<Op> {
    pub fn new(len: usize) -> Self {
        UnionFind { par: (0..len).collect(), size: vec![1; len], next: (0..len).collect(), diff: vec![Op::e(); len] }
    }
    
    pub fn extend(&mut self, len: usize) {
        self.par.extend(self.size.len()..len);
        self.next.extend(self.size.len()..len);
        self.size.resize(len, 1);
        self.diff.resize(len, Op::e());
    }
    
    pub fn len(&self) -> usize { self.par.len() }
    
    pub fn leader_and_size(&mut self, mut i: usize) -> (usize, usize) {
        let mut v = vec![];
        while self.par[i] != i { v.push(i); i = self.par[i]; }
        for j in (1..v.len()).rev() {
            self.par[v[j-1]] = i;
            self.diff[v[j-1]] = Op::add(&self.diff[v[j]], &self.diff[v[j-1]]);
        }
        (i, self.size[i])
    }
    
    pub fn is_same(&mut self, i: usize, j: usize) -> bool {
        self.leader_and_size(i).0 == self.leader_and_size(j).0
    }
    
    /// `potential[i] - potential[j]` を返す。
    pub fn diff(&mut self, i: usize, j: usize) -> Option<Op::T> {
        if self.is_same(i, j) { Some(Op::sub(&self.diff[i], &self.diff[j])) } else { None }
    }
    
    /// `potential[i] - potential[j] = w` となるよう辺を追加する。
    pub fn merge(&mut self, i: usize, j: usize, mut w: Op::T) -> Option<bool> {
        let ((mut u, us), (mut v, vs)) = (self.leader_and_size(i), self.leader_and_size(j));
        // assert!([self.par[i], self.par[j], self.par[u], self.par[v]] == [u, v, u, v]);
        // p(i) = p(u) + diff[i], p(j) = p(v) + diff[j]
        // p(i) - p(j) = w => p(u) - p(v) = w - di + dj
        w = Op::sub(&Op::add(&w, &self.diff[j]), &self.diff[i]);
        if u == v { return if w == Op::e() { Some(false) } else { None }; }
        if us > vs { (u, v) = (v, u); w = Op::inv(&w); }
        self.par[u] = v;
        self.diff[u] = w;
        self.size[v] = us + vs;
        self.next.swap(u, v);
        Some(true)
    }
    
    pub fn group(&mut self, i: usize) -> Vec<usize> {
        let (mut res, mut j) = (vec![i], self.next[i]);
        while j != i { res.push(j); j = self.next[j]; }
        res
    }
    
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = crate::nest![void; self.len()];
        for i in 0..self.len() { res[self.leader_and_size(i).0].push(i); }
        res.retain(|v| v.len() != 0);
        res
    }
}

impl<Op: UnionFindOp> std::fmt::Debug for UnionFind<Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::mylib::util::func::join;
        let mut uf = self.clone();
        let g = uf.groups().into_iter().map(|s| {
            join(s.into_iter().map(|i| format!("{i}({:?})", uf.diff[i]).trim_end_matches("(())").into())).unwrap()
        });
        write!(f, "[{}]", join(g.into_iter().map(|s| format!("{{{s}}}"))).unwrap_or(String::new()))
    }
}
