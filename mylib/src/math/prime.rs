use std::collections::BTreeMap;
use crate::mylib::math::barrett::Barrett128;


/// Miller-Rabin 法による素数判定。`O(n^1/4)`
pub fn miller_rabin(n: u64) -> bool {
    let n = n as u128;
    if n < 67 || n % 2 == 0 { return [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61].contains(&n); }
    
    let l: &[u128] = if n < 4759123141 { &[2, 7, 61] } else { &[2, 325, 9375, 28178, 450775, 9780504, 1795265022] };
    let s = (n-1).trailing_zeros();
    let d = (n-1) >> s;
    let brt = Barrett128::new(n);
    
    'a: for &a in l {
        let mut v = brt.pow(a, d as u128);
        if v == 1 || v == n-1 { continue; }
        for _ in 0..s-1 {
            v = brt.reduce(v*v);
            if v == n-1 { continue 'a; }
        }
        return false;
    }
    
    true
}



pub struct PrimeTable {
    max: usize,
    primes: Vec<usize>,
    /// `lpf[i]`: `i` が持つ最小の素因数
    lpf: Vec<usize>
}

impl PrimeTable {
    /// `max` までのテーブルを作成する。`O(N)`
    pub fn new(mut max: usize) -> Self {
        max = max.max(3);
        let mut primes = vec![];
        let mut lpf = vec![0; max+1];
        
        for i in 2..=max {
            if lpf[i] == 0 { primes.push(i); lpf[i] = i; }
            let lpfi = lpf[i];
            for &p in primes.iter().take_while(move |&&p| p <= lpfi && i*p <= max) {
                lpf[i*p] = p;
            }
        }
        
        PrimeTable { max, primes, lpf }
    }
    
    /// `v` が素数であるかを判定する。`v <= self.max` なら `O(1)`, そうでないなら Miller-Rabin 法で `O(N^1/4)`
    pub fn is_prime(&self, n: usize) -> bool {
        if n <= self.max { self.lpf[n] == n } else { miller_rabin(n as u64) }
    }
    
    /// `v` を素因数分解する。
    pub fn fact(&self, mut n: usize) -> BTreeMap<usize, usize> {
        assert_ne!(n, 0);
        let mut out = BTreeMap::new();
        
        for p in Iterator::chain(self.primes.iter().cloned(), self.max+1..) {
            if n <= self.max {
                while n != 1 {
                    *out.entry(self.lpf[n]).or_default() += 1;
                    n /= self.lpf[n];
                }
                break;
            }
            if n < p*p { *out.entry(n).or_default() += 1; break; }
            while n%p == 0 { n /= p; *out.entry(p).or_default() += 1; }
        }
        
        out
    }
    
    pub fn primes(&self) -> &[usize] { &self.primes }
}
