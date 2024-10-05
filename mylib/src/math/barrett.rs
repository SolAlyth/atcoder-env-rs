use crate::elsedef;

/// Burrett for `u64`.
/// 
/// # Constraints
/// 
/// + `2 <= modulo < 2^32`
#[derive(Clone, Copy)]
pub struct Barrett64 {
    m: u64,
    minv: u64
}

impl Barrett64 {
    pub fn new(modulo: u64) -> Self {
        assert!(modulo < 1<<32);
        Self { m: modulo, minv: !0 / modulo }
    }
    
    pub fn reduce(self, value: u64) -> u64 {
        let tmp = ((value as u128 * self.minv as u128 >> 64) as u64 + 1) * self.m;
        elsedef!(value < tmp; self.m) + value - tmp
    }
}



/// Burrett for `u128`.
/// 
/// # Constraints
/// 
/// + `2 <= modulo < 2^64`
#[derive(Clone, Copy)]
pub struct Barrett128 {
    m: u128,
    minv: u128
}

impl Barrett128 {
    pub fn new(modulo: u128) -> Self {
        assert!(modulo < 1<<64);
        Self { m: modulo, minv: !0 / modulo }
    }
    
    pub fn reduce(self, value: u128) -> u128 {
        let (lu, ll) = (value >> 64, value & (1<<64)-1);
        let (ru, rl) = (self.minv >> 64, self.minv & (1<<64)-1);
        let mut tmp = (lu*ru + (ll*ru + lu*rl >> 64) + 2) * self.m;
        if value < tmp { tmp -= self.m; }
        if value < tmp { tmp -= self.m; }
        value - tmp
    }
    
    pub fn pow(self, mut a: u128, mut b: u128) -> u128 {
        let mut res = 1;
        while b != 0 {
            if b & 1 == 1 { res = self.reduce(res * a); }
            a = self.reduce(a*a);
            b >>= 1;
        }
        res
    }
}
