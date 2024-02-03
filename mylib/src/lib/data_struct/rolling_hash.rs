#![allow(unused)]

const MERSENNE_EXP: u32 = 107;
const MOD: i128 = 2i128.pow(MERSENNE_EXP)-1;

/// `l`, `r` は normalize されている必要がある。
const fn mul_by_mod(l: i128, r: i128) -> i128 {
    // const MASK: i128 = 2i128.pow(MERSENNE_EXP)-1;
    
    let lu = l >> MERSENNE_EXP/2;
    todo!()
}

fn normalize(value: &mut i128) -> i128 {
    todo!()
}

#[derive(Clone, Copy)]
pub struct RollingHash {
    value: i128, len: usize
}

impl RollingHash {
    const BASE: [i128; 1] = [100];
    
    pub const fn e() -> Self {
        RollingHash { value: 0, len: 0 }
    }
    
    pub fn concat_back(mut self, rhs: Self) -> Self {
        // self.value = (self.value * self.base.mpow(rhs.len, MOD) + rhs.value) % self.m;
        self.len += rhs.len;
        todo!()
    }
}

pub struct RollingHashBuilder {
    values: Vec<RollingHash>
}
