use std::{collections::VecDeque, fmt::Debug};

pub use crate::mylib::abstracts::Monoid;

/// Foldable Deque
/// 
/// `push_front/back` を `amotized O(1) time` でしつつ、モノイドの総積を `O(1)` で取得できるデータ構造。
/// 
/// # 搭載機能
/// 
/// - `Clone`, `Debug`, `FromIterator`
#[derive(Clone)]
pub struct FoldableDeque<Op: Monoid> {
    deque: VecDeque<Op::T>,
    front: Vec<Op::T>,
    back: Vec<Op::T>,
    e: Op::T
}

impl<Op: Monoid> FoldableDeque<Op> {
    pub fn new() -> Self {
        Self { deque: VecDeque::new(), front: vec![], back: vec![], e: Op::e() }
    }
    
    pub fn deque(&self) -> &VecDeque<Op::T> { &self.deque }
    pub fn len(&self) -> usize { self.deque.len() }
    
    fn fold_front(&self) -> &Op::T { self.front.last().unwrap_or(&self.e) }
    fn fold_back(&self) -> &Op::T { self.back.last().unwrap_or(&self.e) }
    pub fn fold(&self) -> Op::T { Op::prod(&self.fold_front(), &self.fold_back()) }
    
    pub fn push_front(&mut self, v: Op::T) {
        self.front.push(Op::prod(&v, self.fold_front()));
        self.deque.push_front(v);
    }
    
    pub fn push_back(&mut self, v: Op::T) {
        self.back.push(Op::prod(self.fold_back(), &v));
        self.deque.push_back(v);
    }
    
    pub fn pop_front(&mut self) -> Option<Op::T> {
        let res = self.deque.pop_front();
        if self.front.pop().is_none() { self.rebuild(); }
        res
    }
    
    pub fn pop_back(&mut self) -> Option<Op::T> {
        let res = self.deque.pop_back();
        if self.back.pop().is_none() { self.rebuild(); }
        res
    }
    
    fn rebuild(&mut self) {
        self.front.clear(); self.back.clear();
        let len = self.deque.len();
        for i in (0..len/2).rev() { self.front.push(Op::prod(&self.deque[i], self.fold_front())); }
        for i in len/2..len { self.back.push(Op::prod(self.fold_back(), &self.deque[i])); }
    }
}

impl<Op: Monoid> FromIterator<Op::T> for FoldableDeque<Op> {
    fn from_iter<T: IntoIterator<Item = Op::T>>(iter: T) -> Self {
        let mut deq = Self::new();
        for v in iter { deq.push_back(v); }
        deq
    }
}

impl<Op: Monoid> Debug for FoldableDeque<Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ fold: {:?}, deque: {:?} }}", self.fold(), self.deque)
    }
}
