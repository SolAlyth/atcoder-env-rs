use crate::nest;

#[derive(Clone, Copy)]
/// `Node::Leader` は集合の要素数を持ち、`Node::Child` は親のインデックスを持つ。
enum Node { Leader(usize), Child(usize) }



/// Union Find (Disjoint Set)
/// 
/// `Node::Leader` は集合の要素数を持つ実装。
#[derive(Clone)]
pub struct UnionFind { nodes: Vec<Node> }

impl UnionFind {
    pub fn new(len: usize) -> Self {
        UnionFind { nodes: vec![Node::Leader(1); len] }
    }
    
    /// Leader と size を返す。同時に経路圧縮を行う。
    fn leader_and_size(&mut self, u: usize) -> (usize, usize) {
        let mut now = u;
        let mut stack = vec![];
        
        loop {
            match self.nodes[now] {
                Node::Child(par) => { stack.push(now); now = par; }
                Node::Leader(size) => {
                    for i in stack { self.nodes[i] = Node::Child(now); }
                    return (now, size);
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
        let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
        
        if ul != vl {
            if us < vs { (ul, vl) = (vl, ul); }
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

/// Weighted Union Find
///
/// `diff[leader] = 0, diff[child] = weight[child] - weight[parent]` を持ち、経路圧縮や merge 時に適切に処理する。
/// 
/// 「頂点 `u` から頂点 `v` に重さ `w` の辺を作る」とは、`weight[v] = weight[u] + w` が成り立つように `diff` を変更すること。  
/// ( `u -> v` ベクトルなイメージ )
///
/// # Complexity
///
/// [`UnionFind`] と同じ。

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
    
    /// `Leader` と `size` を返す。同時に経路圧縮を行う。
    fn leader_and_size(&mut self, u: usize) -> (usize, usize) {
        let mut now = u;
        let mut stack = vec![];
        
        loop {
            match self.nodes[now] {
                Node::Child(par) => { stack.push(now); now = par; }
                Node::Leader(size) => {
                    for &child in stack.iter().rev() {
                        let Node::Child(parent) = self.nodes[child] else { unreachable!(); };
                        self.nodes[child] = Node::Child(now);
                        self.diff[child] += self.diff[parent];
                    }
                    
                    return (now, size);
                }
            }
        }
    }
    
    pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
    pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
    
    /// 頂点 u から頂点 v への辺の重さを返す。同じグループに属さない場合、`Err` を返す。
    pub fn weight(&mut self, u: usize, v: usize) -> Result<WeightType, ()> {
        if self.leader(u) == self.leader(v) { Ok(-self.diff[u] + self.diff[v]) } else { Err(()) }
    }
    
    /// 頂点 u から頂点 v に重さ w の辺ができるよう結合する。
    /// 結合に成功したら `Ok(true)` を返し、元々結合されていたら `Ok(false)` を返す。前の情報と矛盾する場合は `Err` を返す。
    pub fn merge(&mut self, u: usize, v: usize, mut w: WeightType) -> Result<bool, ()> {
        let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
        
        if ul != vl {
            if us < vs { (ul, vl) = (vl, ul); w = -w; }
            self.nodes[ul] = Node::Leader(us+vs);
            self.nodes[vl] = Node::Child(ul);
            // 頂点 u, v は経路圧縮されていることに注意。
            // w = weight[v] - weight[u] = (diff[vl] + diff[v]) - (diff[ul] + diff[u]) と diff[ul] = 0 を diff[vl] について解く
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
