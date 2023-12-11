use {
    std::ops::{Deref, Index, BitAnd, BitOr, BitXor, Not},
    itertools::Itertools
};

/// size, idx < 64
#[derive(Clone, Copy)]
pub struct BoolSet {
    value: usize, size: usize
}

impl BoolSet {
    fn sup(size: usize) -> usize { assert!(size < 64); 1<<size }
    pub fn max(size: usize) -> usize { Self::sup(size)-1 }
    pub fn gen(size: usize) -> impl Iterator<Item = Self> { (0..Self::sup(size)).map(move |i| BoolSet { value: i, size }) }
    pub fn get(&self, idx: usize) -> bool { assert!(idx < self.size); self.value>>idx & 1 == 1 }
    pub fn set(&mut self, idx: usize, value: bool) {
        assert!(idx < self.size);
        if value { self.value |= 1<<idx; } else { self.value &= !(1<<idx); }
    }
    pub fn count_true(&self) -> usize { self.value.count_ones() as usize }
    pub fn count_false(&self) -> usize { self.size - self.count_true() }
    pub fn is_empty(&self) -> bool { self.value == 0 }
}

impl BitAnd for BoolSet {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output { self.value &= rhs.value; self }
}

impl BitOr for BoolSet {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output { self.value |= rhs.value; self }
}

impl BitXor for BoolSet {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output { self.value ^= rhs.value; self }
}

impl Not for BoolSet {
    type Output = Self;
    fn not(mut self) -> Self::Output { self.value = !self.value; self }
}

impl Deref for BoolSet {
    type Target = usize;
    fn deref(&self) -> &Self::Target { &self.value }
}


#[derive(Clone)]
pub struct BitSet<'a> {
    bits: &'a [usize], data: Vec<usize>, value: usize
}

impl<'a> BitSet<'a> {
    pub fn min(bits: &'a [usize]) -> Self {
        Self { bits, data: vec![0; bits.len()], value: 0 }
    }
    
    pub fn max(bits: &'a [usize]) -> Self {
        Self { bits, data: bits.iter().map(|&v| v-1).collect_vec(), value: bits.iter().product::<usize>()-1 }
    }
    
    pub fn increment(mut self) -> Option<Self> {
        for (i, v) in self.data.iter_mut().enumerate() {
            if *v != self.bits[i]-1 {
                *v += 1;
                return Some(self)
            } else {
                *v = 0;
            }
        }
        None
    }
    
    pub fn decrement(mut self) -> Option<Self> {
        for (i, v) in self.data.iter_mut().enumerate() {
            if *v != 0 {
                *v -= 1;
                return Some(self);
            } else {
                *v = self.bits[i]-1;
            }
        }
        None
    }
}

impl<'a> Index<usize> for BitSet<'a> {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output { &self.data[index] }
}

impl<'a> Deref for BitSet<'a> {
    type Target = usize;
    fn deref(&self) -> &Self::Target { &self.value }
}
