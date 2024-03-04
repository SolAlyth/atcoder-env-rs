use std::{mem::replace, ops::{Bound, RangeBounds}};

/// `len <= 2^n` を満たす最小の正整数 `n` を返す。
fn ceil_log(n: usize) -> u32 { (n.max(2)-1).ilog2() + 1 }

pub enum Monoid<S: Clone> { Nop, Op(S, Box<dyn Fn(&S, &S) -> S>) }
pub enum Map<S: Clone, F: Clone> { Nop, Op(F, Box<dyn Fn(&F, &mut F)>, Box<dyn Fn(&F, &mut S)>) }

pub struct SegtreeBuilder<S: Clone, F: Clone> {
    
}

pub struct Segtree<S: Clone, F: Clone> {
    tree: Vec<S>,
    log: u32,
    monoid: Monoid<S>,
    map: Map<S, F>
}

// pub struct EntryAll<'a, S: Clone> { seg: &'a mut Segtree<S>, changed: bool }

pub struct Entry<'a, S: Clone, F: Clone> { seg: &'a mut Segtree<S, F>, idx: usize, changed: bool }



/* impl<S: Clone, F: Clone> SegtreeBuilder<S, F> {
    fn e(impl Fn(&S, &S) -> S + 'static) {
        
    }
} */



impl<S: Clone, F: Clone> Segtree<S, F> {
    pub fn new(len: usize, e: S, op: impl Fn(&S, &S) -> S + 'static, id: F, comp: impl Fn(&F, &mut F) + 'static, map: impl Fn(&F, &mut S) + 'static) -> Self {
        let log = ceil_log(len) + 1;
        let len = 1 << log;
        Segtree { tree: vec![e.clone(); len], log, monoid: Monoid::Op(e, Box::new(op)), map: Map::Op(id, Box::new(comp), Box::new(map)) }
    }
    
    fn len(&self) -> usize { 1 << self.log-1 }
    
    fn interpret(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let l = match range.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => v+1,
            Bound::Unbounded => 0
        };
        let r = match range.end_bound() {
            Bound::Included(v) => v+1,
            Bound::Excluded(v) => *v,
            Bound::Unbounded => self.len()
        };
        
        (l+self.len(), r+self.len())
    }
    
    pub fn calc(&mut self, idx: usize) {
        let Monoid::Op(_, op) = &self.monoid else { panic!(); };
        self.tree[idx] = (*op)(&self.tree[idx*2], &self.tree[idx*2+1]);
    }
    
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> S {
        let Monoid::Op(e, op) = &self.monoid else { panic!(); };
        
        let (mut l, mut r) = self.interpret(range);
        let (mut outl, mut outr) = (e.clone(), e.clone());
        
        while l < r {
            if l&1 == 1 { outl = (*op)(&outl, &self.tree[l]); l += 1; }
            if r&1 == 1 { r -= 1; outr = (*op)(&self.tree[r], &outr); }
            l >>= 1; r >>= 1;
        }
        
        (*op)(&outl, &outr)
    }
    
    pub fn entry<'a>(&'a mut self, mut idx: usize) -> Entry<'a, S, F> {
        assert!(idx < self.len());
        idx += self.len();
        Entry { seg: self, idx, changed: false }
    }
}



impl<'a, S: Clone, F: Clone> Entry<'a, S, F> {
    pub fn get_mut(&mut self) -> &mut S {
        self.changed = true; &mut self.seg.tree[self.idx]
    }
    
    pub fn replace(&mut self, value: S) -> S {
        self.changed = true; replace(&mut self.seg.tree[self.idx], value)
    }
}

impl<'a, S: Clone, F: Clone> Drop for Entry<'a, S, F> {
    fn drop(&mut self) {
        if self.changed { for j in 1..self.seg.log { self.seg.calc(self.idx >> j); } }
    }
}

/* impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Drop for Entry<'a, Mn, Mp> {
    fn drop(&mut self) { if self.changed && !Mn::NOP { for j in 1..self.seg.log { self.seg.calc(self.i >> j); } } }
} */
