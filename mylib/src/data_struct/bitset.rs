//! BitSet

use std::{fmt::Debug, ops::{BitAnd, BitOr, BitXor, Index, Not}};

/// 最大 `usize::BITS-1 = 63` 個の `bool` 値を持てる構造体。
/// 
/// 各 `bool` 値に対して `BitAnd, BitOr, BitXor, Not` が可能。
/// `Index` で特定インデックスの `bool` 値、`Deref` で内部値を取得できる。
/// 
/// # Guarantee
/// 
/// + 使われない bit は 0 (`count_true` のため)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitSet { len: usize, value: usize }

impl BitSet {
    /// `[value; len]` を表す `BitSet` を返す。
    pub fn new(value: bool, len: usize) -> Self { BitSet { value: if value {!0} else {0}, len }.masked() }
    
    /// 内部値から `BitSet` を作成する。自動的に `masked` される。
    pub fn from(value: usize, len: usize) -> Self { BitSet { value, len }.masked() }
    
    /// 使われない bit を 0 にしたものを返す。
    fn masked(mut self) -> Self { self.value &= BitSet::max(self.len); self }
    
    /// `BitSet.value` が取りうる範囲 `0..sup` の上限値を返す。
    ///
    /// 例えば、`len = 3` のとき `sup = 8 = 2.pow(3) = 0b1000` である。
    pub fn sup(len: usize) -> usize { 1<<len }
    
    /// `BitSet.value` が取りうる範囲 `0..=max` の最大値を返す。
    ///
    /// 例えば、`len = 3` のとき `max = 7 = 2.pow(3) - 1 = 0b111` である。
    pub fn max(len: usize) -> usize { (1<<len)-1 }
    
    /// `BitSet.value` が小さい順に `BitSet` を返す `DoubleEndedIterator` を作る。
    pub fn generate(len: usize) -> impl DoubleEndedIterator<Item = Self> { (0..Self::sup(len)).map(move |i| BitSet { value: i, len }) }
    
    /// 内部値を返す。
    pub fn raw(&self) -> usize { self.value }
    
    /// `self[idx] = value;` を行う。
    pub fn set(&mut self, idx: usize, value: bool) {
        assert!(idx < self.len); if value { self.value |= 1<<idx; } else { self.value &= !(1<<idx); }
    }
    
    /// `true` である bit の数を返す。
    pub fn count_true(&self) -> usize { self.value.count_ones() as usize }
    /// `false` である bit の数を返す。
    pub fn count_false(&self) -> usize { self.len - self.count_true() }
    
    /// 全ての bit が `true` か判定する。
    pub fn is_full(&self) -> bool { self.value == BitSet::max(self.len) }
    /// 全ての bit が `false` か判定する。
    pub fn is_empty(&self) -> bool { self.value == 0 }
    
    /// `self[0]` から `self[len-1]` を `DoubleEndedIterator` で返す。
    pub fn iter(self) -> impl DoubleEndedIterator<Item = bool> { (0..self.len).map(move |i| self[i]) }
}

impl BitAnd for BitSet { type Output = Self; fn bitand(mut self, rhs: Self) -> Self::Output { assert_eq!(self.len, rhs.len); self.value &= rhs.value; self } }
impl BitOr for BitSet { type Output = Self; fn bitor(mut self, rhs: Self) -> Self::Output { assert_eq!(self.len, rhs.len); self.value |= rhs.value; self } }
impl BitXor for BitSet { type Output = Self; fn bitxor(mut self, rhs: Self) -> Self::Output { assert_eq!(self.len, rhs.len); self.value ^= rhs.value; self } }
impl Not for BitSet { type Output = Self; fn not(mut self) -> Self::Output { self.value = !self.value; self.masked() } }
impl Debug for BitSet { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}({})", self.iter().collect::<Vec<_>>(), self.value) } }
impl Index<usize> for BitSet { type Output = bool; fn index(&self, index: usize) -> &Self::Output { assert!(index < self.len); static A: [bool; 2] = [false, true]; &A[self.value>>index & 1] } }
