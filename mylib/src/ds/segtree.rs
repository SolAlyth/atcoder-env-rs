use std::{fmt::Debug, mem, ops::{Index, IndexMut, RangeBounds}, slice::SliceIndex};
use crate::mylib::util::traits::AsBounds;

#[derive(Clone)]
pub struct SegTree<Op: SegTreeOp> {
    tree: Vec<Op::Value>,
    lazy: Vec<Option<Op::Lazy>>,
    depth: u32
}

pub trait SegTreeOp: Clone {
    type Value: Clone + Debug;
    type Lazy: Clone;
    
    /// `Value` の単位元を返す。
    fn id_value() -> Self::Value;
    
    /// `Value` の積を返す。
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    
    /// `value` に `lazy` を作用させる。
    fn act_value(value: &mut Self::Value, lazy: &Self::Lazy);
    
    /// `lazy` の上から `ad` を合成させる。
    fn comp_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy);
}

pub struct EntryAll<'a, Op: SegTreeOp> {
    seg: &'a mut SegTree<Op>,
    changed: bool
}




impl<Op: SegTreeOp> SegTree<Op> {
    pub fn new(len: usize) -> Self {
        let depth = (len.max(2)-1).ilog2() + 2;
        SegTree { tree: vec![Op::id_value(); 1<<depth], lazy: vec![None; 1<<depth], depth }
    }
    
    pub fn len(&self) -> usize { 1 << self.depth-1 }
    
    pub fn get_ref(&mut self, mut i: usize) -> &Op::Value {
        i += self.len();
        for j in (0..self.depth).rev() { self.push(i >> j); }
        &self.tree[i]
    }
    
    pub fn set(&mut self, mut i: usize, value: Op::Value) -> Op::Value {
        i += self.len();
        for j in (0..self.depth).rev() { self.push(i >> j); }
        let res = mem::replace(&mut self.tree[i], value);
        for j in 1..self.depth { self.update(i >> j); }
        res
    }
    
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> Op::Value {
        let [mut l, mut r] = range.as_bounds(self.len()).map(|v| v + self.len());
        let (mut resl, mut resr) = (Op::id_value(), Op::id_value());
        
        for i in 1..self.depth { self.push(l >> i); self.push(r-1 >> i); }
        
        while l < r {
            if l&1 == 1 {
                resl = Op::prod_value(&resl, self.push(l));
                l += 1;
            }
            if r&1 == 1 {
                r -= 1;
                resr = Op::prod_value(self.push(r), &resr);
            }
            l >>= 1; r >>= 1;
        }
        
        Op::prod_value(&resl, &resr)
    }
    
    pub fn apply_lazy(&mut self, range: impl RangeBounds<usize>, lazy: Op::Lazy) {
        let [l, r] = range.as_bounds(self.len()).map(|v| v + self.len());
        if l == r { return; }
        
        for i in 1..self.depth { self.push(l >> i); self.push(r-1 >> i); }
        
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l&1 == 1 {
                    self.comp_lazy(l, &lazy);
                    self.push(l);
                    l += 1;
                }
                if r&1 == 1 {
                    r -= 1;
                    self.comp_lazy(r, &lazy);
                    self.push(r);
                }
                l >>= 1; r >>= 1;
            }
        }
        
        for i in 1..self.depth { self.update(l >> i); self.update(r-1 >> i); }
    }
    
    /// `f(l..r) == true` かつ `f(l..r+1) == false` である `r` を一つ返す。  
    /// ただし `f(l..l) == true` かつ `f(l..len) == false` であるとする。
    pub fn max_right(&mut self, l: usize, f: impl Fn(&Op::Value) -> bool) -> usize {
        let (mut r, mut val) = (l + self.len(), Op::id_value());
        
        loop {
            r >>= r.trailing_zeros();
            let tmp = Op::prod_value(&val, self.push(r));
            if f(&tmp) {
                if r.count_ones() == 1 { return self.len(); }
                val = tmp;
                r += 1;
            } else {
                while r < self.len() {
                    r <<= 1;
                    let tmp = Op::prod_value(&val, self.push(r));
                    if f(&tmp) { val = tmp; r += 1; }
                }
                return r - self.len();
            }
        }
    }
    
    /// `f(l..r) == true` かつ `f(l-1..r) == false` である `l` を一つ返す。
    /// ただし `f(r..r) == true` かつ `f(-1..r) == false` とする。
    pub fn max_left(&mut self, r: usize, f: impl Fn(&Op::Value) -> bool) -> usize {
        let (mut l, mut val) = (r + self.len(), Op::id_value());
        
        loop {
            l -= 1;
            l >>= l.trailing_zeros();
            
            let tmp = Op::prod_value(self.push(l), &val);
            if f(&tmp) {
                if l.count_ones() == 1 { return 0; }
                val = tmp;
            } else {
                while l < self.len() {
                    l = 2*l+1;
                    let tmp = Op::prod_value(self.push(l), &val);
                    if f(&tmp) { val = tmp; l -= 1; }
                }
                return l+1 - self.len();
            }
        }
    }
    
    pub fn entry_all<'a>(&'a mut self) -> EntryAll<'a, Op> {
        for i in 1..self.tree.len() { self.push(i); }
        EntryAll { seg: self, changed: false }
    }
    
    
    
    /// `lazy[i]` を作用させ、子に伝搬する。
    fn push(&mut self, i: usize) -> &Op::Value {
        let Some(lazy) = mem::replace(&mut self.lazy[i], None) else { return &self.tree[i]; };
        if i < self.len() {
            self.comp_lazy(2*i, &lazy);
            self.comp_lazy(2*i+1, &lazy);
        }
        Op::act_value(&mut self.tree[i], &lazy);
        &self.tree[i]
    }
    
    /// `tree[i]` を子から更新する。
    fn update(&mut self, i: usize) {
        self.tree[i] = Op::prod_value(&self.tree[2*i], &self.tree[2*i+1]);
    }
    
    fn comp_lazy(&mut self, i: usize, ad: &Op::Lazy) {
        if let Some(lazy) = &mut self.lazy[i] { Op::comp_lazy(lazy, ad); } else { self.lazy[i] = Some(ad.clone()); }
    }
}

impl<Op: SegTreeOp> Debug for SegTree<Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut seg = self.clone();
        for i in 1..seg.tree.len() { seg.push(i); }
        f.debug_list().entries((0..self.depth).map(|i| self.tree[1<<i..1<<i+1].to_vec())).finish()
    }
}

impl<Op: SegTreeOp> FromIterator<Op::Value> for SegTree<Op> {
    fn from_iter<T: IntoIterator<Item = Op::Value>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        let mut seg = Self::new(v.len());
        {
            let mut seg = seg.entry_all();
            for (i, v) in v.into_iter().enumerate() { seg[i] = v; }
        }
        seg
    }
}



impl<'a, Op: SegTreeOp, I: SliceIndex<[Op::Value]>> Index<I> for EntryAll<'a, Op> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.seg.tree[self.seg.len()..], index)
    }
}

impl<'a, Op: SegTreeOp, I: SliceIndex<[Op::Value]>> IndexMut<I> for EntryAll<'a, Op> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.changed = true;
        let len = self.seg.len();
        IndexMut::index_mut(&mut self.seg.tree[len..], index)
    }
}

impl<'a, Op: SegTreeOp> Drop for EntryAll<'a, Op> {
    fn drop(&mut self) {
        if self.changed {
            for i in (1..self.seg.len()).rev() { self.seg.update(i); }
        }
    }
}
