pub use crate::mylib::abstracts::{Group, Nop};

/// Potential 付き Union Find (union by size, path compression)
/// 
/// Potential として可換群が乗る。
/// 
/// # 搭載機能
/// 
/// - `Clone`, `Debug`
/// 
/// # 例題
/// 
/// - https://atcoder.jp/contests/abc328/tasks/abc328_f
#[derive(Clone)]
pub struct UnionFind<Op: Group> {
    par: Vec<usize>,
    size: Vec<usize>,
    diff: Vec<Op::T>
}



impl UnionFind<Nop> {
    pub fn new_nop(len: usize) -> Self { Self::new(len) }
}

impl<Op: Group> UnionFind<Op> {
    pub fn new(len: usize) -> Self {
        UnionFind { par: (0..len).collect(), size: vec![1; len], diff: vec![Op::e(); len] }
    }
    
    pub fn extend(&mut self, len: usize) {
        self.par.extend(self.size.len()..len);
        self.size.resize(len, 1);
        self.diff.resize(len, Op::e());
    }
    
    pub fn len(&self) -> usize { self.par.len() }
    
    pub fn leader(&mut self, i: usize) -> usize {
        let p = self.par[i];
        if self.par[p] == p { return p; }
        let u = self.leader(p);
        self.diff[i] = Op::add(&self.diff[i], &self.diff[p]);
        self.par[i] = u;
        u
    }
    
    pub fn size(&mut self, mut i: usize) -> usize { i = self.leader(i); self.size[i] }
    pub fn is_same(&mut self, i: usize, j: usize) -> bool { self.leader(i) == self.leader(j) }
    
    /// `potential[i] - potential[j]` を返す。
    pub fn diff(&mut self, i: usize, j: usize) -> Option<Op::T> {
        if self.is_same(i, j) { Some(Op::sub(&self.diff[i], &self.diff[j])) } else { None }
    }
    
    /// `potential[i] - potential[j] = w` となるよう情報を追加する。
    /// 整合性を保てないとき `None` を返す。変更がないとき `Some(None)` を返す。変更があるとき、`Some(Some(new_leader, old_leader))` を返す。
    pub fn merge(&mut self, i: usize, j: usize, mut w: Op::T) -> Option<Option<(usize, usize)>> {
        let (mut u, mut v) = (self.leader(i), self.leader(j));
        // assert!([self.par[i], self.par[j], self.par[u], self.par[v]] == [u, v, u, v]);
        // p(i) = p(u) + diff[i], p(j) = p(v) + diff[j]
        // p(i) - p(j) = w => p(u) - p(v) = w - di + dj
        w = Op::sub(&Op::add(&w, &self.diff[j]), &self.diff[i]);
        if u == v { return if w == Op::e() { Some(None) } else { None } }
        if self.size[u] > self.size[v] { (u, v) = (v, u); w = Op::inv(&w); }
        self.par[u] = v;
        self.diff[u] = w;
        self.size[v] += self.size[u];
        Some(Some((v, u)))
    }
    
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = crate::nest![void; self.len()];
        for i in 0..self.len() { res[self.leader(i)].push(i); }
        res.retain(|v| v.len() != 0);
        res
    }
    
    pub fn leaders(&self) -> Vec<usize> {
        (0..self.len()).filter(|&i| self.par[i] == i).collect()
    }
}

impl<Op: Group> std::fmt::Debug for UnionFind<Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::mylib::util::func::join;
        let mut uf = self.clone();
        let g = uf.groups().into_iter().map(|s| {
            join(s.into_iter().map(|i| format!("{i}({:?})", uf.diff[i]).trim_end_matches("(())").into())).unwrap()
        });
        write!(f, "[{}]", join(g.into_iter().map(|s| format!("{{{s}}}"))).unwrap_or(String::new()))
    }
}
