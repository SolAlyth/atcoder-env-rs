//! ある始点からの最短/最長経路 `O(EV)`
//! 
//! # Algorithm
//! 
//! 1. 始点以外を最悪コスト(到達不能)で初期化し、辺ごとにコスト更新することを `N` 回行う。
//! 2. もう一度、辺ごとにコスト更新することを `N` 回行うが、更新できた場合は行き先を最良コストにする。
//! 
//! 更新できたときに `prev[v] = Some(u);` とかしてあげると、最短経路が後ろから分かる。

use crate::mylib::util::hyperint::h64;
use crate::{chmin, chmax};

pub struct BellmanFord {
    minimize: bool,
    start: usize,
    
    cost: Vec<h64>,
    prev: Vec<usize>
}

pub enum CostResult {
    Finite(i64),
    Infinite,
    Unreachable
}

impl BellmanFord {
    /// 有向重み付きグラフの、頂点 `start` からある頂点への最小/最大コストを求める。`O(EV)`
    /// 
    /// # Input
    /// 
    /// + `edge: &[(u, v, w)]`: `u -> v` に重み `w` の辺があることを表す。
    pub fn new(minimize: bool, start: usize, len: usize, edge: &[(usize, usize, i64)]) -> Self {
        let mut cost = vec![if minimize {h64::MAX} else {h64::MIN}; len];
        let mut prev = vec![usize::MAX; len];
        cost[start] = h64(0);
        
        for i in 0..2*len {
            for &(u, v, w) in edge {
                let uw = cost[u] + w;
                let upd = if minimize { chmin!(cost[v]; uw) } else { chmax!(cost[v]; uw) };
                if upd {
                    prev[v] = u;
                    if len-1 <= i { cost[v] = if minimize {h64::MIN} else {h64::MAX}; }
                }
            }
        }
        
        BellmanFord { minimize, start, cost, prev }
    }
    
    pub fn cost(&self, u: usize) -> CostResult {
        use CostResult::*;
        if !self.cost[u].is_minmax() {
            Finite(self.cost[u].0)
        } else if self.cost[u].is_min() == self.minimize {
            Infinite
        } else {
            Unreachable
        }
    }
    
    /// `start -> u` への経路 `[start, ..., u]` を返す。
    /// 
    /// # Panic
    /// 
    /// + `cost[u]` が `MIN, MAX` であるとき。
    pub fn route(&self, mut u: usize) -> Vec<usize> {
        assert!(!self.cost[u].is_minmax());
        let mut res = vec![u];
        while u != self.start { u = self.prev[u]; res.push(u); }
        res.reverse(); res
    }
    
    // (cost, prev) を返す。
    pub fn inner(self) -> (Vec<h64>, Vec<usize>) { (self.cost, self.prev) }
}
