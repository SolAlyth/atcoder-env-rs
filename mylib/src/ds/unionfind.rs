use std::fmt::Debug;

use crate::nest;

#[derive(Clone, Copy)]
/// `Node::Leader` は集合の要素数を持ち、`Node::Child` は親のインデックスを持つ。
enum Node { Leader(usize), Child(usize) }



/// Union Find (union by size)
#[derive(Clone)]
pub struct UnionFind { nodes: Vec<Node> }

impl UnionFind {
    pub fn new(len: usize) -> Self {
        UnionFind { nodes: vec![Node::Leader(1); len] }
    }
    
    fn leader_and_size(&mut self, mut u: usize) -> (usize, usize) {
        let mut stack = vec![];
        
        loop {
            match self.nodes[u] {
                Node::Child(par) => { stack.push(u); u = par; }
                Node::Leader(size) => {
                    for i in stack { self.nodes[i] = Node::Child(u); }
                    return (u, size);
                }
            }
        }
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
    pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
    
    /// u を含むグループと v を含むグループを結合する。
    /// 結合に成功したら `true` を返し、元々結合されていたら `false` を返す。
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let ((mut u, us), (mut v, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
        
        if u != v {
            if us < vs { (u, v) = (v, u); }
            self.nodes[u] = Node::Leader(us+vs);
            self.nodes[v] = Node::Child(u);
        }
        
        u != v
    }
    
    pub fn group(&mut self, mut u: usize) -> Vec<usize> {
        u = self.leader(u); (0..self.nodes.len()).filter(|&v| self.leader(v) == u).collect()
    }
    
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut out = nest![void; self.nodes.len()];
        for u in 0..self.nodes.len() { out[self.leader(u)].push(u); }
        out.retain(|v| v.len() != 0); out
    }
}

impl Debug for UnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;
        let mut uf = self.clone();
        write!(f, "[{}]", uf.groups().into_iter().map(|v| format!("{{{}}}", v.into_iter().join(", "))).join(", "))
    }
}


type WeightType = i64;

/// Weighted Union Find (union by size)

#[derive(Clone)]
pub struct WeightedUnionFind {
    nodes: Vec<Node>,
    /// `diff[leader] = 0, diff[child] = weight[child] - weight[parent]` を満たす。  
    /// 特に `leader <- parent <- child` から `leader <- {parent, child}` と更新したい場合、`diff[child] += diff[parent]` とすればよい。
    diff: Vec<WeightType>
}

impl WeightedUnionFind {
    pub fn new(len: usize) -> Self {
        WeightedUnionFind { nodes: vec![Node::Leader(1); len], diff: vec![0; len] }
    }
    
    fn leader_and_size(&mut self, mut u: usize) -> (usize, usize) {
        let mut stack = vec![];
        
        loop {
            match self.nodes[u] {
                Node::Child(par) => { stack.push(u); u = par; }
                Node::Leader(size) => {
                    for &child in stack.iter().rev() {
                        let Node::Child(parent) = self.nodes[child] else { unreachable!(); };
                        self.nodes[child] = Node::Child(u);
                        self.diff[child] += self.diff[parent];
                    }
                    return (u, size);
                }
            }
        }
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
    pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
    
    /// 頂点 u に対する頂点 v の辺の重さを返す。
    pub fn weight(&mut self, u: usize, v: usize) -> Result<WeightType, ()> {
        if self.leader(u) == self.leader(v) { Ok(-self.diff[u] + self.diff[v]) } else { Err(()) }
    }
    
    /// `weight[u] + w = weight[v]` が成り立つよう結合する。
    /// 結合に成功したら `Ok(true)` を返し、元々結合されていてかつ操作が矛盾しないなら `Ok(false)` を返す。矛盾する場合は `Err(())` を返す。
    pub fn merge(&mut self, mut u: usize, mut v: usize, mut w: WeightType) -> Result<bool, ()> {
        let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
        
        if ul != vl {
            if !(us >= vs) { (u, v, ul, vl, w) = (v, u, vl, ul, -w); }
            self.nodes[ul] = Node::Leader(us+vs);
            self.nodes[vl] = Node::Child(ul);
            self.diff[vl] = self.diff[u] - self.diff[v] + w;
            Ok(true)
        } else {
            if -self.diff[u] + self.diff[v] == w { Ok(false) } else { Err(()) }
        }
    }
    
    pub fn group(&mut self, mut u: usize) -> Vec<usize> {
        u = self.leader(u); (0..self.nodes.len()).filter(|&v| self.leader(v) == u).collect()
    }
    
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut out = nest![void; self.nodes.len()];
        for u in 0..self.nodes.len() { out[self.leader(u)].push(u); }
        out.retain(|v| v.len() != 0); out
    }
}

impl Debug for WeightedUnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;
        let mut uf = self.clone();
        write!(f, "[{}]", uf.groups().into_iter().map(
            |v| format!("{{{}}}", v.into_iter().map(|i| format!("{i}: {}", uf.diff[i])).join(", "))
        ).join(", "))
    }
}
