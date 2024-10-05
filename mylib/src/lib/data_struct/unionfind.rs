use crate::nest;

#[derive(Clone, Copy)]
enum Node { Leader(usize), Child(usize) }



/// Union Find a.k.a. Disjoint Set.
///
/// # Complexity
///
/// 空間計算量は O(n)
///
/// ||未経路圧縮時|経路圧縮時|
/// |-|-|-|
/// |leader, size, merge| amortized O(α(n) | O(1) |
///
/// ||時間計算量|
/// |-|-|
/// |group, groups| O(n) |

#[derive(Clone)]
pub struct UnionFind {
    nodes: Vec<Node>
}

impl UnionFind {
    pub fn new(len: usize) -> Self {
        UnionFind { nodes: vec![Node::Leader(1); len] }
    }
    
    /// Leader と size を返す。同時に経路圧縮を行う。
    fn leader_and_size(&mut self, u: usize) -> (usize, usize) {
        let mut now = u;
        let mut stack = vec![];
        
        let (leader, size) = loop {
            match self.nodes[now] {
                Node::Leader(size) => { break (now, size); }
                Node::Child(par) => { stack.push(now); now = par; }
            }
        };
        
        for i in stack { self.nodes[i] = Node::Child(leader); }
        
        (leader, size)
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
    pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
    
    /// u を含むグループと v を含むグループを結合する。
    /// 結合に成功したら `true` を返し、元々結合されていたら `false` を返す。
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
        if us < vs { std::mem::swap(&mut ul, &mut vl); } // us と vs は和しか見ないので入れ替えなくてもいい
        
        if ul != vl {
            self.nodes[ul] = Node::Leader(us+vs);
            self.nodes[vl] = Node::Child(ul);
        }
        
        ul != vl
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


type WeightType = i128;

/// Weighted Union Find.
///
/// `diff[parent] = 0, diff[child] = weight[parent] - weight[child]` を持ち、経路圧縮や merge 時に適切に処理する。
///
/// # Complexity
///
/// [`UnionFind`] と同じ。

#[derive(Clone)]
pub struct WeightedUnionFind {
    nodes: Vec<Node>, diff: Vec<WeightType>
}

impl WeightedUnionFind {
    pub fn new(len: usize) -> Self {
        WeightedUnionFind { nodes: vec![Node::Leader(1); len], diff: vec![0; len] }
    }
    
    /// `Leader` と `size` を返す。同時に経路圧縮を行う。
    fn leader_and_size(&mut self, u: usize) -> (usize, usize) {
        let mut now = u;
        let mut stack = vec![];
        
        let (leader, size) = loop {
            match self.nodes[now] {
                Node::Leader(size) => { break (now, size); }
                Node::Child(par) => { stack.push(now); now = par; }
            }
        };
        
        for &child in stack.iter().rev() {
            let Node::Child(parent) = self.nodes[child] else { unreachable!(); };
            self.nodes[child] = Node::Child(leader);
            self.diff[child] += self.diff[parent];
        }
        
        (leader, size)
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
    pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
    
    /// 頂点 u から頂点 v への辺の重さを返す。同じグループに属さない場合、`Err(())` を返す。
    pub fn weight(&mut self, u: usize, v: usize) -> Result<WeightType, ()> {
        if self.leader(u) == self.leader(v) { Ok(-self.diff[u] + self.diff[v]) } else { Err(()) }
    }
    
    /// 頂点 u から頂点 v に重さ w の辺ができるよう結合する。
    /// 結合に成功したら `Ok(true)` を返し、元々結合されていたら `Ok(false)` を返す。前の情報と矛盾する場合は `Err(())` を返す。
    pub fn merge(&mut self, u: usize, v: usize, mut w: WeightType) -> Result<bool, ()> {
        let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
        if us < vs { std::mem::swap(&mut ul, &mut vl); w = -w; } // us と vs は和しか見ないので入れ替えなくてもいい
        
        if ul != vl {
            self.nodes[ul] = Node::Leader(us+vs);
            self.nodes[vl] = Node::Child(ul);
            Ok(true)
        } else {
            if self.weight(u, v).unwrap() == w { Ok(false) } else { Err(()) }
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
