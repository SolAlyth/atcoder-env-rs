use crate::mylib::ds::bitset::BitSet;

#[derive(Clone)]
pub struct XorMatrix {
    data: Vec<BitSet>,
    size_h: usize,
    size_w: usize
}

impl XorMatrix {
    pub fn new(h: usize, w: usize, init: bool) -> Self {
        assert!(h <= 63 && w <= 63);
        Self { data: vec![BitSet::new(init, w); h], size_h: h, size_w: w }
    }
    
    pub fn set(&mut self, i: usize, j: usize, value: bool) { self.data[i].set(j, value); }
    
    pub fn gauss_jordan(&mut self, is_extended: bool) -> Option<Vec<BitSet>> {
        let w = self.size_w - if is_extended {1} else {0};
        let mut j = usize::MAX;
        let (mut pivot, mut bases) = (vec![], vec![]);
        
        'main: for i in 0..self.size_h {
            loop {
                j = j.wrapping_add(1);
                if j == w { break 'main; }
                
                if let Some(mut p) = (i..self.size_h).find(|&i| self.data[i][j]) {
                    self.data.swap(i, p);
                    pivot.push(j);
                    p = i;
                    
                    for i in 0..self.size_h {
                        if !self.data[i][j] || i == p { continue; }
                        let tmp = self.data[p];
                        self.data[i] ^= tmp;
                    }
                    
                    break;
                } else {
                    let mut v = BitSet::new(false, self.size_h);
                    v.set(j, true);
                    for (idx, &i) in pivot.iter().enumerate() {
                        if self.data[idx][j] { v.set(i, true); }
                    }
                    bases.push(v);
                }
            }
        }
        
        Some(bases)
    }
}
