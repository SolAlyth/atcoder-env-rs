//! 全ての頂点ペアの最小コスト経路 `O(n³)`
//! 
//! 有向重み付きグラフの全頂点ペアの最小コスト経路を返す。
//! 負閉路が存在する場合は適用できない。
//! 
//! 負辺が存在しないときは dijkstra n 回 `O(nm + n²logn)` のほうが早い。
//! 
//! # Algorithm
//! 
//! 1. `d[i][j]`: 「`i -> j` の最小コスト」と定義し、`d[i][i] = 0, d[u][v] = w, others = INF` で初期化。
//! 2. 経由頂点が `0..=k-1` に限定された `d` が求まっているときの、`k` を経由した最小コスト経路で chmin する。これは、`for (k, i, j) { chmin!(d[i][j]; d[i][k] + d[k][j]); }` とすればよい。
//! 3. ただし、負閉路がある場合は `d` がどんどん小さくなるためオーバーフローする。`if i == j && d[i][j] < 0 { return Err; }` とすればよい。

use itertools::iproduct;

use crate::{chmin, mylib::h64, nest};

pub struct WarshallFloyd {
    /// `d[i][j]` = `i -> j` 最小コスト
    d: Vec<Vec<h64>>,
    /// `aft[i][j]` = `i -> j` 最小コスト経路の、`i` の次の頂点
    aft: Vec<Vec<usize>>
}

pub enum CostResult {
    Finite(i64),
    Unreachable
}

impl WarshallFloyd {
    /// 構築する。ただし、負閉路が存在する場合は `None` を返す。
    pub fn new(len: usize, edge: &[(usize, usize, i64)]) -> Option<Self> {
        let mut d = nest![h64::MAX; len; len];
        let mut aft = nest![0; len; len];
        
        for i in 0..len { d[i][i] = h64(0); }
        for &(u, v, w) in edge { chmin!(d[u][v]; h64(w)); }
        for (i, j) in iproduct!(0..len, 0..len) { aft[i][j] = j; }
        
        for (k, i, j) in iproduct!(0..len, 0..len, 0..len) {
            // 経由頂点が 0..=k-1 に限定された最短経路が求まっているときの、k を経由した最短経路を考える
            if chmin!(d[i][j]; d[i][k] + d[k][j]) { aft[i][j] = aft[i][k]; }
            if i == j && d[i][j] < h64(0) { return None; }
        }
        
        Some(WarshallFloyd { d, aft })
    }
    
    pub fn cost(&self, u: usize, v: usize) -> CostResult {
        if self.d[u][v] == h64::MAX { CostResult::Unreachable } else { CostResult::Finite(self.d[u][v].0) }
    }
    
    /// `u -> v` への最小コスト経路 `[u, ..., v]` を返す。
    pub fn route(&self, mut u: usize, v: usize) -> Vec<usize> {
        let mut ret = vec![];
        loop {
            ret.push(u); if u == v { break ret; } else { u = self.aft[u][v]; }
        }
    }
}
