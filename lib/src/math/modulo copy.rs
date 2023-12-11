#![allow(non_upper_case_globals)]

pub const i998: i128 = 998244353;
pub const us998: usize = 998244353;

pub trait Mod: Copy {
    fn normalize(self) -> Self;
    fn mpow(self, p: usize) -> Self;
    
    fn update(&mut self) -> Self { *self = self.normalize(); *self }
    fn inv(self) -> Self { self.mpow(998244353-2) }
}

impl Mod for i128 {
    fn normalize(mut self) -> Self {
        if self < 0 || i998 <= self { self %= i998; if self < 0 { self += i998; } } self
    }
    
    fn mpow(mut self, mut p: usize) -> Self {
        let mut out = 1;
        while p != 0 {
            if p&1 == 1 { out *= self; out.update(); }
            self = self.pow(2).normalize();
            p >>= 1;
        }
        out
    }
}

impl Mod for usize {
    fn normalize(mut self) -> Self {
        if us998 <= self { self %= us998; } self
    }
    
    fn mpow(mut self, mut p: usize) -> Self {
        let mut out = 1;
        while p != 0 {
            if p&1 == 1 { out *= self; out.update(); }
            self = (self.pow(2)).normalize();
            p >>= 1;
        }
        out
    }
}



pub struct ModCalc<const FACT_MAX: usize> {
    fact: [i128; FACT_MAX],
    fact_inv: [i128; FACT_MAX],
    inv: Vec<i128>
}

impl<const FACT_MAX: usize> ModCalc<FACT_MAX> {
    pub const fn new() -> Self {
        let (mut fact, mut fact_inv) = ([i128; FACT_MAX], [FACT_MAX]);
        let mut i = 0;
        fact[0] = 1;
        
        ModCalc { fact: vec![1], fact_inv: vec![1], inv: vec![0, 1] }
    }
    
    fn update_factrial(&mut self, n: usize) {
        if n < self.fact.len() { return; }
        self.fact.reserve(n+1);
        for i in self.fact.len()..=n {
            self.fact[i] = (self.fact[i-1] * i as i128).normalize();
        }
    }
    
    fn update_factorial_inv(&mut self, mut n: usize) {
        if n < self.fact_inv.len() { return; }
        self.fact_inv.resize(n+1, 0);
        self.fact_inv[n] = self.factorial(n).inv();
        while self.fact_inv[n-1] == 0 {
            self.fact_inv[n-1] = (self.fact_inv[n] * n as i128).normalize();
            n -= 1;
        }
    }
    
    fn update_inv_linear(&mut self, n: usize) {
        if n < self.inv.len() { return; }
        for i in self.inv.len()..=n {
            self.inv[i] = (i998 - (i998/i as i128) * self.inv[us998%i]).normalize();
        }
    }
    
    pub fn factorial(&mut self, n: usize) -> i128 {
        self.update_factrial(n); self.fact[n]
    }
    
    pub fn factorial_inv(&mut self, n: usize) -> i128 {
        self.update_factorial_inv(n); self.fact_inv[n]
    }
    
    pub fn combination(&mut self, n: usize, k: usize) -> i128 {
        self.factorial(n) * self.fact_inv[k] * self.fact_inv[n-k]
    }
    
    pub fn parmutation(&mut self, n: usize, k: usize) -> i128 {
        self.factorial(n) * self.fac
    }
    
    pub fn inv_linear(&mut self, mut value: i128) -> i128 {
        assert!(value != 0);
        self.update_inv_linear(value.update() as usize);
        self.inv[value as usize]
    }
}
