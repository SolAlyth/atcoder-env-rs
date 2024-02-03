use crate::mylib::data_struct::traits::Monoid as MonoidTrait;
use std::{mem::swap, ops::{Index,IndexMut}, slice::SliceIndex};


/// セグメント木。
///
/// 木の値が未定義であることを `None` で表す。子に `None` が含まれる場合、親も `None` になる。
///
/// space `O(2N)`
///
/// # Note
///
/// `Some(Monoid)` であった場所に `None` を再代入しても正しく動くが、そんなに嬉しくなさそう。
/// 悪い点は `calc fn` に `if` が一個挟まる程度なのでいいか…？
pub struct Segtree<Monoid: MonoidTrait> {
    monoid: Monoid,
    tree: Vec<Option<Monoid::Set>>, // tree.len = 2*(2^depth) = 2*st
    depth: u32, // = ceil(log2(data_len))
    st: usize // = 2^depth
}

/// 一斉更新待ちのセグメント木。
pub struct SegtreeLocked<'a, Monoid: MonoidTrait> {
    seg: &'a mut Segtree<Monoid>
}


impl<Monoid: MonoidTrait> Segtree<Monoid> {
    /// 全ての値を `None` で初期化する。`vec![None; N]` の最適化により time `O(1)`
    pub fn new(monoid: Monoid, data_len: usize) -> Self {
        let depth = (data_len-1).ilog2() + 1; // = ceil(log2(data_len))
        let st = 1<<depth;
        Segtree { monoid, tree: vec![None; 2*st], depth, st }
    }
    
    /// 一斉更新するために [SegtreeLocked] を返す。再計算の時間計算量 `O(N)`
    #[must_use]
    pub fn lock(&mut self) -> SegtreeLocked<Monoid> {
        SegtreeLocked { seg: self }
    }
    
    /// `tree[idx]` を計算する。
    fn calc(&mut self, idx: usize) {
        self.tree[idx] = if let (Some(lhs), Some(rhs)) = (&self.tree[2*idx], &self.tree[2*idx+1]) {
            Some(self.monoid.op(lhs, rhs))
        } else {
            None
        };
    }
    
    /// `tree[..]` を一斉計算する。
    fn calc_all(&mut self) {
        for i in (1..1<<self.depth).rev() { self.calc(i); }
    }
    
    /// `Segtree[idx]` に代入し、木を再計算する。
    pub fn set(&mut self, mut idx: usize, mut value: Option<Monoid::Set>) -> Option<Monoid::Set> {
        assert!(idx < 1<<self.depth);
        
        idx += 1 << self.depth;
        swap(&mut self.tree[idx], &mut value);
        for _ in 0..self.depth { idx >>= 1; self.calc(idx); }
        value
    }
    
    /// 区間 `[left, right)` のモノイド積を返す。
    ///
    /// # Panic
    ///
    /// + 区間が不正: not `left <= right`
    /// + 範囲外の区間: not `right <= 2.pow(self.depth)`
    /// + 一部の範囲内の要素が `None` である
    pub fn fold(&mut self, mut left: usize, mut right: usize) -> Monoid::Set {
        assert!(left <= right);
        assert!(right <= 1<<self.depth);
        
        left += 1<<self.depth; right += 1<<self.depth;
        let (mut out_l, mut out_r) = (self.monoid.ident(), self.monoid.ident());
        while left < right {
            if left&1 == 1 {
                out_l = self.monoid.op(&out_l, self.tree[left].as_ref().unwrap());
                left += 1;
            }
            if right&1 == 1 {
                right -= 1;
                out_r = self.monoid.op(self.tree[right].as_ref().unwrap(), &out_r);
            }
            left >>= 1; right >>= 1;
        }
        
        self.monoid.op(&out_l, &out_r)
    }
}

impl<Monoid: MonoidTrait, I: SliceIndex<[Option<Monoid::Set>]>> Index<I> for Segtree<Monoid> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.tree[self.st..], index)
    }
}

impl<'a, Monoid: MonoidTrait> Drop for SegtreeLocked<'a, Monoid> {
    fn drop(&mut self) { self.seg.calc_all(); }
}

impl<'a, Monoid: MonoidTrait, I: SliceIndex<[Option<Monoid::Set>]>> Index<I> for SegtreeLocked<'a, Monoid> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.seg.tree[self.seg.st..], index)
    }
}

impl<'a, Monoid: MonoidTrait, S: SliceIndex<[Option<Monoid::Set>]>> IndexMut<S> for SegtreeLocked<'a, Monoid> {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.seg.tree[self.seg.st..], index)
    }
}
