// #![allow(non_upper_case_globals)]

use std::collections::BTreeMap;

pub struct PrimeTable { max: usize, _primes: Vec<usize>, lpf: Vec<usize> }

impl PrimeTable {
    /// `[0, max]` のテーブルを作成する。`O(N)`
    /// 
    /// # Panic
    /// 
    /// `3 <= max` でないとき。(壊れたら怖いので)
    pub fn new(max: usize) -> Self {
        assert!(3 <= max);
        let mut primes = vec![];
        let mut lpf = vec![0; max+1];
        
        for i in 2..=max {
            if lpf[i] == 0 { primes.push(i); lpf[i] = i; }
            let lpfi = lpf[i];
            for &p in primes.iter().take_while(|&&p| p <= lpfi && i*p <= max) {
                lpf[i*p] = p;
            }
        }
        
        PrimeTable { max, _primes: primes, lpf }
    }
    
    /// `v` が素数であるかを判定する。`v <= max` なら `O(1)`, そうでないなら試し割りで `O(√N)`
    pub fn is_prime(&self, v: usize) -> bool {
        if v <= self.max {
            self.lpf[v] == v
        } else {
            Iterator::chain(self._primes.iter().cloned(), self.max+1..).take_while(|&p| p.pow(2) <= v).all(|p| v%p != 0)
        }
    }
    
    /// `v` を素因数分解する。
    pub fn fact(&self, mut v: usize) -> BTreeMap<usize, usize> {
        assert_ne!(v, 0);
        let mut out = BTreeMap::new();
        
        for p in Iterator::chain(self._primes.iter().cloned(), self.max+1..) {
            if v <= self.max {
                while v != 1 {
                    *out.entry(self.lpf[v]).or_default() += 1;
                    v /= self.lpf[v];
                }
                break;
            }
            if v < p*p { *out.entry(v).or_default() += 1; break; }
            while v%p == 0 { v /= p; *out.entry(p).or_default() += 1; }
        }
        
        out
    }
    
    pub fn primes(&self) -> &[usize] { &self._primes }
}
