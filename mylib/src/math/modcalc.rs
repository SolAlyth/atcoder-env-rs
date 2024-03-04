use crate::mylib::{imod, usmod, math::modulo::Modulo};

/// 色々計算できるやつ。
pub struct ModCalc {
    /// `[n!]`
    fact: Vec<i128>,
    /// `[inv(n!)]`
    fact_inv: Vec<i128>,
    /// `[inv(n)]`
    minv: Vec<i128>
}

impl ModCalc {
    pub fn new() -> Self {
        ModCalc { fact: vec![1], fact_inv: vec![1], minv: vec![0, 1] }
    }
    
    /// n までの階乗・階乗逆数・逆数テーブルを計算する。計算量 <O(n)>.
    ///
    /// O(n) で逆数テーブルを作る ... ref. https://drken1215.hatenablog.com/entry/2018/06/08/210000
    pub fn calc_tables(&mut self, n: usize) {
        // 階乗
        for i in self.fact.len()..=n {
            // i! = (i-1)! * i
            self.fact.push((self.fact[i-1] * i as i128).simplify());
        }
        
        // 逆数
        for i in self.minv.len()..=n {
            // M/i * i + M%i = 0 より inv(i) = M - M/i * inv(M%i)
            self.minv.push(imod - (imod/i as i128 * self.minv[usmod%i]) % imod);
        }
        
        // 階乗逆数
        for i in self.fact_inv.len()..=n {
            // inv(i!) = inv((i-1)!) * inv(i)
            self.fact_inv.push((self.fact_inv[i-1] * self.minv[i]).simplify());
        }
    }
    
    /// `n! mod m` を返す。計算量 <O(n), O(1)>.
    pub fn factorial(&mut self, n: usize) -> i128 {
        if self.fact.len()-1 < n { self.calc_tables(n); }
        self.fact[n]
    }
    
    /// `inv(n!) mod m` を返す。計算量 <O(n), O(1)>.
    pub fn factorial_inv(&mut self, n: usize) -> i128 {
        if self.fact_inv.len()-1 < n { self.calc_tables(n); }
        self.fact_inv[n]
    }
    
    /// `inv(n)` を返す。計算量 <O(n), O(1)>.
    pub fn minv(&mut self, n: usize) -> i128 {
        if self.minv.len()-1 < n { self.calc_tables(n); }
        self.minv[n]
    }
    
    /// `nCk mod m` を返す。計算量 <O(n), O(1)>.
    pub fn combination(&mut self, n: usize, k: usize) -> i128 {
        (self.fact[n] * self.fact_inv[k] * self.fact_inv[n-k]).simplify()
    }
    
    /// `nPk mod m` を返す。計算量 <O(n), O(1)>.
    pub fn permutation(&mut self, n: usize, k: usize) -> i128 {
        (self.factorial(n) * self.factorial_inv(n-k)).simplify()
    }
}
