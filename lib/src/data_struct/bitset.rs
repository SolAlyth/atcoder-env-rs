use {
    std::ops::{Deref, Index, IndexMut, BitAnd, BitOr, BitXor, Not},
    itertools::Itertools
};

/// size, idx < 64
#[derive(Clone, Copy)]
pub struct BoolSet {
    pub value: usize, pub size: usize
}

impl BoolSet {
    fn sup(size: usize) -> usize { assert!(size < 64); 1<<size }
    pub fn len(size: usize) -> usize { Self::sup(size) }
    pub fn max(size: usize) -> usize { Self::sup(size)-1 }
    pub fn gen(size: usize) -> impl Iterator<Item = Self> {
        let f = move |i| BoolSet { value: i, size };
        (0..Self::sup(size)).map(f)
    }
    pub fn get(&self, idx: usize) -> bool { assert!(idx < 64); self.value>>idx & 1 == 1 }
    pub fn set(&mut self, idx: usize, value: bool) {
        if value { self.set_true(idx); } else { self.set_false(idx); }
    }
    pub fn set_true(&mut self, idx: usize) { assert!(idx < 64); self.value |= 1<<idx; }
    pub fn set_false(&mut self, idx: usize) { assert!(idx < 64); self.value &= !(1<<idx); }
    pub fn count_true(&self) -> usize { self.value.count_ones() as usize }
    pub fn count_false(&self) -> usize { self.size - self.count_true() }
    pub fn increment(mut self) -> Option<Self> {
        if self.value != Self::max(self.size) { self.value += 1; Some(self) } else { None }
    }
    pub fn is_empty(&self) -> bool { self.value == 0 }
}

impl BitAnd for BoolSet {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.value &= rhs.value; self
    }
}

impl BitOr for BoolSet {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.value |= rhs.value; self
    }
}

impl BitXor for BoolSet {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self.value ^= rhs.value; self
    }
}

impl Not for BoolSet {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.value = !self.value; self
    }
}

impl Deref for BoolSet {
    type Target = usize;
    fn deref(&self) -> &Self::Target { &self.value }
}


#[derive(Clone)]
pub struct BitSet<'a> {
    bits: &'a [usize], data: Vec<usize>
}

impl<'a> BitSet<'a> {
    pub fn min(bits: &'a [usize]) -> Self {
        Self { bits, data: vec![0; bits.len()] }
    }
    
    pub fn max(bits: &'a [usize]) -> Self {
        Self { bits, data: bits.into_iter().map(|&v| v-1).collect_vec() }
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
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> IndexMut<usize> for BitSet<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
