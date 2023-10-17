#![allow(non_upper_case_globals)]

use std::collections::HashMap;

const lsize: usize = 1_000_000;
const plsize: usize = 200_000;

pub struct PrimeTable { pub prime: Vec<usize>, lpf: Vec<usize> }

impl PrimeTable {
    pub fn new() -> Self {
        let mut prime = vec![0; plsize];
        let mut lpf = vec![0; lsize];
        for i in 2..lsize {
            if lpf[0] == 0 { prime.push(i); lpf[i] = i; }
            for &j in &prime {
                lpf[i*j] = j;
            }
        }
        
        Self { prime, lpf }
    }
    
    pub fn fact(&self, mut v: usize) -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        
        if lsize < v {
            'a: for &p in self {
                while Self::trial(p, &mut v, &mut map) {
                    if v <= lsize { break 'a; }
                }
            }
            map.insert(v, 1); return map;
        }
        
        while v != 1 {
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
    
    pub fn fact_trial(mut v: usize) -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        if v == 1 { return map; }
        for p in 2.. {
            while Self::trial(p, &mut v, &mut map) { if v == 1 { return map; } }
        }
        map
    }
    
    pub fn is_prime(&self, v: usize) -> bool {
        self.lpf[v] == v
    }
    
    fn trial(p: usize, v: &mut usize, map: &mut HashMap<usize, usize>) -> bool {
        *v % p == 0 && { *v /= p; *map.entry(p).or_insert(0) += 1; true }
    }
}

impl<'a> IntoIterator for &'a PrimeTable {
    type Item = &'a usize; type IntoIter = std::slice::Iter<'a, usize>;
    fn into_iter(self) -> Self::IntoIter { self.prime.iter() }
}


/* impl std::ops::Index<usize> for PrimeTable {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {&self.l[index]}
} */
