#![allow(non_upper_case_globals)]

use std::collections::HashMap;

const psize: usize = 500_000;

pub struct PrimeTable<const size: usize = 1_000_000> { pub prime: [usize; psize], lpf: [usize; size], pl: usize }

impl<const size: usize> PrimeTable<size> {
    pub const fn new() -> Self {
        let mut prime = [0; psize];
        let mut lpf = [0; size];
        let mut i = 2; let mut pl = 0;
        while i < size {
            if lpf[i] == 0 { prime[pl] = i; pl += 1; }
            let mut j = 0;
            while j < pl && i*prime[j] < size {
                lpf[i*prime[j]] = prime[j]; j += 1;
            }
            i += 1;
        }
        
        Self { prime, lpf, pl }
    }
    
    pub fn fact(&self, mut v: usize) -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        
        if size < v {
            'a: for &p in self {
                while Self::trial(p, &mut v, &mut map) {
                    if v <= size { break 'a; }
                }
            }
        }
        
        while 2 <= self.lpf[v] {
            *map.entry(self.lpf[v]).or_insert(0) += 1;
            v /= self.lpf[v];
        }
        
        map
    }
    
    pub fn fact_spec(prime: &[usize], mut v: usize) -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        for &p in prime {
            while Self::trial(p, &mut v, &mut map) { if v == 1 { return map; } }
        }
        map.insert(v, 1); map
    }
    
    fn trial(p: usize, v: &mut usize, map: &mut HashMap<usize, usize>) -> bool {
        *v % p == 0 && { *v /= p; *map.entry(p).or_insert(0) += 1; true }
    }
}

impl<'a, const size: usize> IntoIterator for &'a PrimeTable<size> {
    type Item = &'a usize; type IntoIter = std::slice::Iter<'a, usize>;
    fn into_iter(self) -> Self::IntoIter { self.prime[..self.pl].iter() }
}


/* impl std::ops::Index<usize> for PrimeTable {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {&self.l[index]}
} */
