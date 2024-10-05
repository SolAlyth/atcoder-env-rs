#[cfg(not(debug_assertions))] #[allow(unused)]
mod mylib {
    #![allow(non_upper_case_globals)]
    
    pub const SUBMISSION: bool = true;
    
    pub use {
        util::{
            printer::{out, end, EndFlag}, traits::*, hyperint::h64, func::binary_search
        },
        
        proconio::{input, input_interactive, marker::{Bytes as bytes, Chars as chars, Usize1 as usize1, Isize1 as isize1}},
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet},
        std::mem::{swap, replace},
        itertools::{Itertools, iproduct, izip},
        superslice::Ext,
        num_integer::{gcd, lcm, Roots},
        rand,
        ac_library
    };
    
    
    pub mod ds {
        pub mod bitset {
            use std::{fmt::Debug, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign, Shr, ShrAssign}}; #[derive(Clone, Copy, PartialEq, Eq)] pub struct BitSet { len: usize, value: usize } impl BitSet { pub fn new(value: bool, len: usize) -> Self { BitSet { value: if value {!0} else {0}, len }.masked() } pub fn from(value: usize, len: usize) -> Self { BitSet { value, len }.masked() } fn masked(mut self) -> Self { self.value &= BitSet::max(self.len); self } pub fn sup(len: usize) -> usize { 1<<len } pub fn max(len: usize) -> usize { (1<<len)-1 } pub fn generate(len: usize) -> impl DoubleEndedIterator<Item = Self> { (0..Self::sup(len)).map(move |i| BitSet { value: i, len }) } pub fn raw(&self) -> usize { self.value } pub fn set(&mut self, idx: usize, value: bool) { assert!(idx < self.len); if value { self.value |= 1<<idx; } else { self.value &= !(1<<idx); } } pub fn count_true(&self) -> usize { self.value.count_ones() as usize } pub fn count_false(&self) -> usize { self.len - self.count_true() } pub fn is_full(&self) -> bool { self.value == BitSet::max(self.len) } pub fn is_empty(&self) -> bool { self.value == 0 } pub fn iter(self) -> impl DoubleEndedIterator<Item = (usize, bool)> { (0..self.len).map(move |i| (i, self[i])) } } impl BitAnd for BitSet { type Output = Self; fn bitand(mut self, rhs: Self) -> Self::Output { self &= rhs; self } } impl BitOr for BitSet { type Output = Self; fn bitor(mut self, rhs: Self) -> Self::Output { self |= rhs; self } } impl BitXor for BitSet { type Output = Self; fn bitxor(mut self, rhs: Self) -> Self::Output { self ^= rhs; self } } impl BitAndAssign for BitSet { fn bitand_assign(&mut self, rhs: Self) { debug_assert_eq!(self.len, rhs.len); self.value &= rhs.value; } } impl BitOrAssign for BitSet { fn bitor_assign(&mut self, rhs: Self) { debug_assert_eq!(self.len, rhs.len); self.value |= rhs.value; } } impl BitXorAssign for BitSet { fn bitxor_assign(&mut self, rhs: Self) { debug_assert_eq!(self.len, rhs.len); self.value ^= rhs.value; } } impl Shl<usize> for BitSet { type Output = Self; fn shl(mut self, rhs: usize) -> Self::Output { self <<= rhs; self.masked() } } impl Shr<usize> for BitSet { type Output = Self; fn shr(mut self, rhs: usize) -> Self::Output { self >>= rhs; self } } impl ShlAssign<usize> for BitSet { fn shl_assign(&mut self, rhs: usize) { self.value <<= rhs; *self = self.masked(); } } impl ShrAssign<usize> for BitSet { fn shr_assign(&mut self, rhs: usize) { self.value >>= rhs; } } impl Not for BitSet { type Output = Self; fn not(mut self) -> Self::Output { self.value = !self.value; self.masked() } } impl Debug for BitSet { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}({})", (0..self.len).map(|i| self[i]).collect::<Vec<_>>(), self.value) } } impl Index<usize> for BitSet { type Output = bool; fn index(&self, index: usize) -> &Self::Output { assert!(index < self.len); &[false, true][self.value>>index & 1] } }
        }
        pub mod compress {
            pub struct Compress<T: Ord> (Vec<T>); impl<T: Ord> Compress<T> { pub fn new() -> Self { Compress(vec![]) } pub fn entry(&mut self, value: T) { self.0.push(value); } pub fn calc(mut self) -> Compressed<T> { self.0.sort_unstable(); self.0.dedup(); Compressed(self.0) } } pub struct Compressed<T: Ord> (Vec<T>); impl<T: Ord> Compressed<T> { pub fn len(&self) -> usize { self.0.len() } pub fn key(&self, value: &T) -> usize { self.0.binary_search(value).unwrap() } pub fn restore(&self, key: usize) -> &T { &self.0[key] } pub fn inner(&self) -> &Vec<T> { &self.0 } }
        }
        pub mod unionfind {
            use std::fmt::Debug;

            use crate::nest;

            #[derive(Clone, Copy)]
            /// `Node::Leader` は集合の要素数を持ち、`Node::Child` は親のインデックスを持つ。
            enum Node { Leader(usize), Child(usize) }



            /// Union Find (union by size)
            #[derive(Clone)]
            pub struct UnionFind { nodes: Vec<Node> }

            impl UnionFind {
                pub fn new(len: usize) -> Self {
                    UnionFind { nodes: vec![Node::Leader(1); len] }
                }
                
                fn leader_and_size(&mut self, mut u: usize) -> (usize, usize) {
                    let mut stack = vec![];
                    
                    loop {
                        match self.nodes[u] {
                            Node::Child(par) => { stack.push(u); u = par; }
                            Node::Leader(size) => {
                                for i in stack { self.nodes[i] = Node::Child(u); }
                                return (u, size);
                            }
                        }
                    }
                }
                
                pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
                pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
                pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
                
                /// u を含むグループと v を含むグループを結合する。
                /// 結合に成功したら `true` を返し、元々結合されていたら `false` を返す。
                pub fn merge(&mut self, u: usize, v: usize) -> bool {
                    let ((mut u, us), (mut v, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
                    
                    if u != v {
                        if us < vs { (u, v) = (v, u); }
                        self.nodes[u] = Node::Leader(us+vs);
                        self.nodes[v] = Node::Child(u);
                    }
                    
                    u != v
                }
                
                pub fn group(&mut self, mut u: usize) -> Vec<usize> {
                    u = self.leader(u); (0..self.nodes.len()).filter(|&v| self.leader(v) == u).collect()
                }
                
                pub fn groups(&mut self) -> Vec<Vec<usize>> {
                    let mut out = nest![void; self.nodes.len()];
                    for u in 0..self.nodes.len() { out[self.leader(u)].push(u); }
                    out.retain(|v| v.len() != 0); out
                }
            }

            impl Debug for UnionFind {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    use itertools::Itertools;
                    let mut uf = self.clone();
                    write!(f, "[{}]", uf.groups().into_iter().map(|v| format!("{{{}}}", v.into_iter().join(", "))).join(", "))
                }
            }


            type WeightType = i64;

            /// Weighted Union Find (union by size)

            #[derive(Clone)]
            pub struct WeightedUnionFind {
                nodes: Vec<Node>,
                /// `diff[leader] = 0, diff[child] = weight[child] - weight[parent]` を満たす。  
                /// 特に `leader <- parent <- child` から `leader <- {parent, child}` と更新したい場合、`diff[child] += diff[parent]` とすればよい。
                diff: Vec<WeightType>
            }

            impl WeightedUnionFind {
                pub fn new(len: usize) -> Self {
                    WeightedUnionFind { nodes: vec![Node::Leader(1); len], diff: vec![0; len] }
                }
                
                fn leader_and_size(&mut self, mut u: usize) -> (usize, usize) {
                    let mut stack = vec![];
                    
                    loop {
                        match self.nodes[u] {
                            Node::Child(par) => { stack.push(u); u = par; }
                            Node::Leader(size) => {
                                for &child in stack.iter().rev() {
                                    let Node::Child(parent) = self.nodes[child] else { unreachable!(); };
                                    self.nodes[child] = Node::Child(u);
                                    self.diff[child] += self.diff[parent];
                                }
                                return (u, size);
                            }
                        }
                    }
                }
                
                pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 }
                pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 }
                pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }
                
                /// 頂点 u に対する頂点 v の辺の重さを返す。
                pub fn weight(&mut self, u: usize, v: usize) -> Result<WeightType, ()> {
                    if self.leader(u) == self.leader(v) { Ok(-self.diff[u] + self.diff[v]) } else { Err(()) }
                }
                
                /// `weight[u] + w = weight[v]` が成り立つよう結合する。
                /// 結合に成功したら `Ok(true)` を返し、元々結合されていてかつ操作が矛盾しないなら `Ok(false)` を返す。矛盾する場合は `Err(())` を返す。
                pub fn merge(&mut self, mut u: usize, mut v: usize, mut w: WeightType) -> Result<bool, ()> {
                    let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v));
                    
                    if ul != vl {
                        if !(us >= vs) { (u, v, ul, vl, w) = (v, u, vl, ul, -w); }
                        self.nodes[ul] = Node::Leader(us+vs);
                        self.nodes[vl] = Node::Child(ul);
                        self.diff[vl] = self.diff[u] - self.diff[v] + w;
                        Ok(true)
                    } else {
                        if -self.diff[u] + self.diff[v] == w { Ok(false) } else { Err(()) }
                    }
                }
                
                pub fn group(&mut self, mut u: usize) -> Vec<usize> {
                    u = self.leader(u); (0..self.nodes.len()).filter(|&v| self.leader(v) == u).collect()
                }
                
                pub fn groups(&mut self) -> Vec<Vec<usize>> {
                    let mut out = nest![void; self.nodes.len()];
                    for u in 0..self.nodes.len() { out[self.leader(u)].push(u); }
                    out.retain(|v| v.len() != 0); out
                }
            }

            impl Debug for WeightedUnionFind {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    use itertools::Itertools;
                    let mut uf = self.clone();
                    write!(f, "[{}]", uf.groups().into_iter().map(
                        |v| format!("{{{}}}", v.into_iter().map(|i| format!("{i}: {}", uf.diff[i])).join(", "))
                    ).join(", "))
                }
            }

        }
        pub mod segtree {
            pub use crate::mylib::traits::abstracts::{Map, Monoid}; use std::marker::PhantomData; use std::ops::{Bound, RangeBounds}; use std::{fmt::Debug, mem::replace, ops::{Deref, DerefMut, Index, IndexMut}, slice::SliceIndex}; fn ceil_log(len: usize) -> u32 { (len.max(2)-1).ilog2() + 1 } pub struct Segtree<Mn: Monoid, Mp: Map<Mn::Set>> { tree: Vec<Mn::Set>, map_tree: Vec<Mp::F>, log: u32, } pub struct EntryAll<'a, Mn: Monoid, Mp: Map<Mn::Set>> { seg: &'a mut Segtree<Mn, Mp>, changed: bool } pub struct Entry<'a, Mn: Monoid, Mp: Map<Mn::Set>> { seg: &'a mut Segtree<Mn, Mp>, i: usize, changed: bool } impl Segtree<Nop<()>, Nop<()>> { pub fn new_no_map<Mn: Monoid>(len: usize) -> Segtree<Mn, Nop<Mn::Set>> { let log = ceil_log(len) + 1; let size = 1 << log; Segtree { tree: vec![Mn::e(); size], map_tree: vec![(); size], log } } pub fn new_no_monoid<T: Clone + Debug, Mp: Map<T>>(init: T, len: usize) -> Segtree<Nop<T>, Mp> { let log = ceil_log(len) + 1; let size = 1 << log; Segtree { tree: vec![init; size], map_tree: vec![Mp::id(); size], log } } pub fn new<Mn: Monoid, Mp: Map<Mn::Set>>(len: usize) -> Segtree<Mn, Mp> { let log = ceil_log(len) + 1; let size = 1 << log; Segtree { tree: vec![Mn::e(); size], map_tree: vec![Mp::id(); size], log } } } impl<Mn: Monoid, Mp: Map<Mn::Set>> Segtree<Mn, Mp> { pub fn hint_monoid(&self, _: Mn) {} pub fn hint_map(&self, _: Mp) {} fn interpret(&self, range: impl RangeBounds<usize>) -> (usize, usize) { let l = match range.start_bound() { Bound::Included(v) => *v, Bound::Excluded(v) => v+1, Bound::Unbounded => 0 }; let r = match range.end_bound() { Bound::Included(v) => v+1, Bound::Excluded(v) => *v, Bound::Unbounded => self.len() }; assert!(l <= r && r <= self.len(), "specified: [{l}, {r}), valid: [0, {})", self.len()); (l+self.len(), r+self.len()) } pub fn len(&self) -> usize { self.tree.len()/2 } fn act(&mut self, i: usize) { let f = replace(&mut self.map_tree[i], Mp::id()); let is_leaf = self.map_tree.len() <= 2*i; if !is_leaf { Mp::comp(&f, &mut self.map_tree[2*i]); Mp::comp(&f, &mut self.map_tree[2*i+1]); } if !Mn::NOP || (Mn::NOP && is_leaf) { Mp::map(&f, &mut self.tree[i]); } } fn act_range(&mut self, l: usize, r: usize) { for i in (1..self.log).rev() { if (l >> i) << i != l { self.act(l >> i); } if (r >> i) << i != r { self.act(r >> i); } } } fn calc(&mut self, i: usize) { self.tree[i] = Mn::op(&self.tree[2*i], &self.tree[2*i+1]); } pub fn fold(&mut self, range: impl RangeBounds<usize>) -> Mn::Set { assert!(!Mn::NOP); let (mut l, mut r) = self.interpret(range); if !Mp::NOP { self.act_range(l, r); } let (mut outl, mut outr) = (Mn::e(), Mn::e()); while l < r { if l&1 == 1 { if !Mp::NOP { self.act(l); } outl = Mn::op(&outl, &self.tree[l]); l += 1; } if r&1 == 1 { if !Mp::NOP { self.act(r-1); } outr = Mn::op(&self.tree[r-1], &outr); r -= 1; } l >>= 1; r >>= 1; } Mn::op(&outl, &outr) } pub fn map(&mut self, range: impl RangeBounds<usize>, map: Mp::F) { assert!(!Mp::NOP); let (l, r) = self.interpret(range); self.act_range(l, r); { let (mut l, mut r) = (l, r); let (mut lf, mut rf) = (true, true); while l < r { if l&1 != 0 { Mp::comp(&map, &mut self.map_tree[l]); self.act(l); if lf { self.act(l-1); lf = false; } l += 1; } if r&1 != 0 { Mp::comp(&map, &mut self.map_tree[r-1]); self.act(r-1); if rf { self.act(r); rf = false; } r -= 1; } l >>= 1; r >>= 1; } } if !Mn::NOP { for i in 1..self.log { if (l >> i) << i != l { self.calc(l >> i); } if (r >> i) << i != r { self.calc(r >> i); } } } } pub fn max_right(&mut self, mut l: usize, pred: impl Fn(&Mn::Set) -> bool) -> usize { assert!(l <= self.len()); assert!(pred(&Mn::e())); if l == self.len() { return self.len(); } l += self.len(); if !Mp::NOP { for i in (1..self.log).rev() { self.act(l >> i); } } let mut res = Mn::e(); loop { while l&1 == 0 { l >>= 1; } if !Mp::NOP { self.act(l); } let tmp = Mn::op(&res, &self.tree[l]); if !pred(&tmp) { while l < self.len() { l <<= 1; if !Mp::NOP { self.act(l); } let tmp = Mn::op(&res, &self.tree[l]); if pred(&tmp) { res = tmp; l += 1; } } break l - self.len(); } res = tmp; l += 1; let l = l as isize; if (l & -l) != l { break self.len(); } } } pub fn max_left(&mut self, mut r: usize, pred: impl Fn(&Mn::Set) -> bool) -> usize { assert!(r <= self.len()); assert!(pred(&Mn::e())); if r == 0 { return 0; } r += self.len(); if !Mp::NOP { for i in (1..self.log).rev() { self.act((r-1) >> i); } } let mut res = Mn::e(); loop { r -= 1; while 1 < r && r&1 == 0 { r >>= 1; } if !Mp::NOP { self.act(r); } let tmp = Mn::op(&self.tree[r], &res); if !pred(&tmp) { while r < self.len() { r = 2*r + 1; if !Mp::NOP { self.act(r); } let tmp = Mn::op(&self.tree[r], &res); if pred(&tmp) { res = tmp; r -= 1; } } return r + 1 - self.len(); } res = tmp; let r = r as isize; if (r & -r) != r { return 0; } } } pub fn entry<'a>(&'a mut self, idx: usize) -> Entry<'a, Mn, Mp> { assert!(idx < self.len()); let i = idx + self.len(); if !Mp::NOP { for j in (0..self.log).rev() { self.act(i >> j); } } Entry { seg: self, i, changed: false } } pub fn entry_all<'a>(&'a mut self) -> EntryAll<'a, Mn, Mp> { if !Mp::NOP { for i in 1..self.map_tree.len() { self.act(i); } } EntryAll { seg: self, changed: false } } } impl<Mn: Monoid, Mp: Map<Mn::Set>> Debug for Segtree<Mn, Mp> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { let mut a = vec![String::new(); self.tree.len()]; for i in (if !Mn::NOP {0} else {self.len()})..self.tree.len() { a[i] += &format!("[{}]", Mn::debug(&self.tree[i])); } if !Mp::NOP { for i in 0..self.map_tree.len() { if !a[i].is_empty() { a[i] += " "; } a[i] += &format!("({})", &Mp::debug(&self.map_tree[i])); } } let max_len = a.iter().map(|s| s.len()).max().unwrap(); let mut out = String::new(); for i in 0..self.log { out += "\n"; for j in (1 << i)..(1 << i+1) { out += &format!("{: <width$}", a[j], width = max_len*(1 << self.log-i-1)); } } write!(f, "{}", &out[1..]) } } impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Drop for EntryAll<'a, Mn, Mp> { fn drop(&mut self) { if self.changed && !Mn::NOP { for i in (1..self.seg.tree.len()/2).rev() { self.seg.calc(i); } } } } impl<'a, Mn: Monoid, Mp: Map<Mn::Set>, I: SliceIndex<[Mn::Set]>> Index<I> for EntryAll<'a, Mn, Mp> { type Output = I::Output; fn index(&self, index: I) -> &Self::Output { Index::index(&self.seg.tree[self.seg.len()..], index) } } impl<'a, Mn: Monoid, Mp: Map<Mn::Set>, I: SliceIndex<[Mn::Set]>> IndexMut<I> for EntryAll<'a, Mn, Mp> { fn index_mut(&mut self, index: I) -> &mut Self::Output { self.changed = true; let len = self.seg.len(); IndexMut::index_mut(&mut self.seg.tree[len..], index) } } impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Entry<'a, Mn, Mp> { pub fn map(&mut self, map: &Mp::F) { self.changed = true; Mp::map(map, &mut self.seg.tree[self.i]); } pub fn set(&mut self, value: Mn::Set) -> Mn::Set { self.changed = true; replace(&mut self.seg.tree[self.i], value) } pub fn modify<T>(&mut self, f: impl FnOnce(&mut Mn::Set) -> T) -> T { self.changed = true; f(&mut self.seg.tree[self.i]) } } impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Drop for Entry<'a, Mn, Mp> { fn drop(&mut self) { if self.changed && !Mn::NOP { for j in 1..self.seg.log { self.seg.calc(self.i >> j); } } } } impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Deref for Entry<'a, Mn, Mp> { type Target = Mn::Set; fn deref(&self) -> &Self::Target { &self.seg.tree[self.i] } } impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> DerefMut for Entry<'a, Mn, Mp> { fn deref_mut(&mut self) -> &mut Self::Target { self.changed = true; &mut self.seg.tree[self.i] } } pub struct Nop<T: Clone> (PhantomData<T>); impl<T: Clone + Debug> Monoid for Nop<T> { const NOP: bool = true; type Set = T; fn e() -> T { panic!() } fn op(_: &T, _: &T) -> T { panic!() } fn debug(v: &Self::Set) -> String { format!("{v:?}") } } impl<T: Clone> Map<T> for Nop<T> { const NOP: bool = true; type F = (); fn id() -> () { panic!() } fn comp(_: &(), _: &mut ()) { panic!() } fn map(_: &(), _: &mut T) { panic!() } fn debug(_: &Self::F) -> String { "".into() } }
        }
        pub mod bijection {
            #[derive(Clone, Debug)] pub struct Bijection { v: Vec<Option<usize>>, k: Vec<Option<usize>> } impl Bijection { pub fn new(len: usize) -> Self { Bijection { v: vec![None; len], k: vec![None; len] } } pub fn insert(&mut self, key: usize, value: usize) -> bool { if self.v[key].is_some() || self.k[value].is_some() { return self.v[key] == Some(value); } self.v[key] = Some(value); self.k[value] = Some(key); true } pub fn remove_by_key(&mut self, key: usize) -> bool { let Some(value) = self.v[key] else { return false; }; self.v[key] = None; self.k[value] = None; true } pub fn remove_by_value(&mut self, value: usize) -> bool { let Some(key) = self.k[value] else { return false; }; self.v[key] = None; self.k[value] = None; true } pub fn value(&self, key: usize) -> Option<usize> { self.v[key] } pub fn key(&self, value: usize) -> Option<usize> { self.k[value] } pub fn get_inner_kv(&self) -> &Vec<Option<usize>> { &self.v } pub fn get_inner_vk(&self) -> &Vec<Option<usize>> { &self.k } }
        }
        pub mod multiset {
            use std::{collections::{BTreeMap, HashMap}, hash::Hash, ops::RangeBounds, ptr::eq as ptr_eq}; pub use { btree_multi_set::BTreeMultiSet, hash_multi_set::HashMultiSet }; pub struct BlockItem<'a, V> { pub value: &'a V, pub len: usize, pub idx: usize, }
            mod btree_multi_set {
                use super::*; use std::collections::btree_map::Iter as BIter; #[derive(Clone, Debug, PartialEq, Eq)] pub struct BTreeMultiSet<V: Ord + Clone> { inner: BTreeMap<V, usize>, len: usize } impl<V: Ord + Clone> BTreeMultiSet<V> { pub fn clear(&mut self) { self.inner.clear(); self.len = 0; } pub fn contains(&self, value: &V) -> bool { self.inner.contains_key(&value) } pub fn insert(&mut self, value: &V, n: usize) { self.modify(value, |befn| befn+n); } pub fn is_empty(&self) -> bool { self.len == 0 } pub fn len(&self) -> usize { self.len } pub fn remove(&mut self, value: &V, n: usize, strict: bool) -> bool { let mut ret = true; self.modify(value, |befn| { if strict && befn < n { ret = false; befn } else { befn.saturating_sub(n) } }); ret } pub fn remove_block(&mut self, value: &V) -> Option<usize> { let mut ret = None; self.modify(value, |n| { if n != 0 { ret = Some(n); } 0 }); ret } pub fn len_blocks(&self) -> usize { self.inner.len() } pub fn len_block(&self, value: &V) -> usize { *self.inner.get(value).unwrap_or(&0) } pub fn modify(&mut self, value: &V, f: impl FnOnce(usize) -> usize) { if let Some(n) = self.inner.get_mut(value) { self.len -= *n; *n = f(*n); self.len += *n; if *n == 0 { self.inner.remove(value); } } else { let n = f(0); if n != 0 { self.inner.insert(value.clone(), n); self.len += n; } } } pub fn iter(&self) -> Iter<V> { Iter::new(self) } pub fn iter_blocks(&self) -> impl Iterator<Item = (&V, usize)> + DoubleEndedIterator { self.inner.iter().map(|(v, &n)| (v, n)) } pub fn new() -> Self { Self { inner: BTreeMap::new(), len: 0 } } pub fn first(&self) -> Option<(&V, usize)> { self.inner.first_key_value().map(|v| (v.0, *v.1)) } pub fn last(&self) -> Option<(&V, usize)> { self.inner.last_key_value().map(|v| (v.0, *v.1)) } pub fn pop_first(&mut self) -> Option<V> { let Some(mut entry) = self.inner.first_entry() else { return None; }; let (v, &n) = (entry.key().clone(), entry.get()); if n == 1 { entry.remove(); } else { entry.insert(n-1); } self.len -= 1; Some(v) } pub fn pop_last(&mut self) -> Option<V> { let Some(mut entry) = self.inner.last_entry() else { return None; }; let (v, &n) = (entry.key().clone(), entry.get()); if n == 1 { entry.remove(); } else { entry.insert(n-1); } self.len -= 1; Some(v) } pub fn range_blocks(&self, range: impl RangeBounds<V>) -> impl Iterator<Item = (&V, usize)> + DoubleEndedIterator { self.inner.range(range).map(|(v, &n)| (v, n)) } pub fn pop_first_block(&mut self) -> Option<(V, usize)> { if let Some(v) = self.inner.pop_first() { self.len -= v.1; Some(v) } else { None } } pub fn pop_last_block(&mut self) -> Option<(V, usize)> { if let Some(v) = self.inner.pop_last() { self.len -= v.1; Some(v) } else { None } } } impl<V: Ord + Clone> FromIterator<V> for BTreeMultiSet<V> { fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self { let mut res = Self::new(); for v in iter { res.insert(&v, 1); } res } } pub enum Iter<'a, V> { Empty, Some { src: BIter<'a, V, usize>, f: (&'a V, &'a usize), b: (&'a V, &'a usize), fidx: usize, bidx: usize } } impl<'a, V: Ord + Clone> Iter<'a, V> { fn new(src: &'a BTreeMultiSet<V>) -> Self { if src.is_empty() { return Self::Empty; } let mut src = src.inner.iter(); let f = src.next().unwrap(); let b = src.next_back().unwrap_or(f); Self::Some { src, f, b, fidx: 0, bidx: *b.1 } } } impl<'a, V> Iterator for Iter<'a, V> { type Item = BlockItem<'a, V>; fn next(&mut self) -> Option<Self::Item> { let Self::Some { src, f, b, fidx, bidx } = self else { return None; }; let res = BlockItem { value: f.0, len: *f.1, idx: *fidx }; *fidx += 1; if ptr_eq(f.0, b.0) && fidx == bidx { *self = Self::Empty; return Some(res); } if fidx == f.1 { *f = src.next().unwrap_or(*b); *fidx = 0; } Some(res) } } impl<'a, V> DoubleEndedIterator for Iter<'a, V> { fn next_back(&mut self) -> Option<Self::Item> { let Self::Some { src, f, b, fidx, bidx } = self else { return None; }; *bidx -= 1; let res = BlockItem { value: b.0, len: *b.1, idx: *bidx }; if ptr_eq(f.0, b.0) && fidx == bidx { *self = Self::Empty; return Some(res); } if *bidx == 0 { *b = src.next().unwrap_or(*f); *bidx = *b.1; } Some(res) } }
            }
            mod hash_multi_set {
                use super::*; use std::collections::hash_map::Iter as HIter; pub struct HashMultiSet<V: Clone + Hash + Eq> { inner: HashMap<V, usize>, len: usize } impl<V: Clone + Hash + Eq> HashMultiSet<V> { pub fn clear(&mut self) { self.inner.clear(); self.len = 0; } pub fn contains(&self, value: &V) -> bool { self.inner.contains_key(&value) } pub fn insert(&mut self, value: &V, n: usize) { self.modify(value, |befn| befn+n); } pub fn is_empty(&self) -> bool { self.len == 0 } pub fn len(&self) -> usize { self.len } pub fn remove(&mut self, value: &V, n: usize, strict: bool) -> bool { let mut ret = true; self.modify(value, |befn| { if strict && befn < n { ret = false; befn } else { befn.saturating_sub(n) } }); ret } pub fn remove_block(&mut self, value: &V) -> Option<usize> { let mut ret = None; self.modify(value, |n| { if n != 0 { ret = Some(n); } 0 }); ret } pub fn len_blocks(&self) -> usize { self.inner.len() } pub fn len_block(&self, value: &V) -> usize { *self.inner.get(value).unwrap_or(&0) } pub fn modify(&mut self, value: &V, f: impl FnOnce(usize) -> usize) { if let Some(n) = self.inner.get_mut(value) { self.len -= *n; *n = f(*n); self.len += *n; if *n == 0 { self.inner.remove(value); } } else { let n = f(0); if n != 0 { self.inner.insert(value.clone(), n); self.len += n; } } } pub fn iter(&self) -> Iter<V> { Iter::new(self) } pub fn iter_blocks(&self) -> impl Iterator<Item = (&V, usize)> { self.inner.iter().map(|(v, &n)| (v, n)) } pub fn new() -> Self { Self { inner: HashMap::new(), len: 0} } } impl<V: Clone + Hash + Eq> FromIterator<V> for HashMultiSet<V> { fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self { let mut res = Self::new(); for v in iter { res.insert(&v, 1); } res } } pub enum Iter<'a, V> { Empty, Some { src: HIter<'a, V, usize>, f: (&'a V, &'a usize), fidx: usize, } } impl<'a, V: Clone + Hash + Eq> Iter<'a, V> { fn new(src: &'a HashMultiSet<V>) -> Self { if src.is_empty() { return Self::Empty; } let mut src = src.inner.iter(); let f = src.next().unwrap(); Self::Some { src, f, fidx: 0 } } } impl<'a, V> Iterator for Iter<'a, V> { type Item = BlockItem<'a, V>; fn next(&mut self) -> Option<Self::Item> { let Self::Some { src, f, fidx } = self else { return None; }; let res = BlockItem { value: f.0, len: *f.1, idx: *fidx }; *fidx += 1; if fidx == f.1 { if let Some(tmp) = src.next() { *f = tmp; *fidx = 0; } else { *self = Self::Empty; return Some(res); } } Some(res) } }
            }
        }
        pub mod foldable_deque {
            use std::{collections::VecDeque, fmt::Debug}; pub struct FoldableDeque<'a, T: Clone> { elem: VecDeque<T>, front: Vec<T>, back: Vec<T>, e: T, f: Box<dyn Fn(&T, &T) -> T + 'a> } impl<'a, T: Clone> FoldableDeque<'a, T> { pub fn new(e: T, f: impl Fn(&T, &T) -> T + 'a) -> Self { Self { elem: VecDeque::new(), front: vec![], back: vec![], e, f: Box::from(f) } } pub fn len(&self) -> usize { self.elem.len() } fn acc_front(&self) -> &T { self.front.last().unwrap_or(&self.e) } fn acc_back(&self) -> &T { self.back.last().unwrap_or(&self.e) } pub fn fold(&self) -> T { (self.f)(self.acc_front(), self.acc_back()) } pub fn push_front(&mut self, v: T) { self.front.push((self.f)(&v, self.acc_front())); self.elem.push_front(v); } pub fn push_back(&mut self, v: T) { self.back.push((self.f)(self.acc_back(), &v)); self.elem.push_back(v); } pub fn pop_front(&mut self) -> Option<T> { let res = self.elem.pop_front(); if self.front.pop().is_none() { self.recalc_acc(); } res } pub fn pop_back(&mut self) -> Option<T> { let res = self.elem.pop_back(); if self.back.pop().is_none() { self.recalc_acc(); } res } fn recalc_acc(&mut self) { self.front.clear(); self.back.clear(); let len = self.elem.len(); for i in (0..len/2).rev() { self.front.push((self.f)(&self.elem[i], self.acc_front())); } for i in len/2..len { self.back.push((self.f)(self.acc_back(), &self.elem[i])); } } } impl<'a, T: Clone + Debug> Debug for FoldableDeque<'a, T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { let mut front = self.front.clone(); front.reverse(); write!(f, "elem = {:?}\nfold = {:?} {:?}", self.elem, front, self.back) } }
        }
        
        pub mod splay_tree {
            use crate::mylib::util::traits::AsBounds;

            use std::cell::Cell;
            use std::fmt::Debug;
            use std::ops::{Deref, DerefMut, Index, RangeBounds};
            use std::ptr::{eq as ptr_eq, null_mut};
            use std::mem::replace;



            pub trait SplayOps {
                type Value: Sized + Clone;
                type Acc: Sized + Clone;
                type Lazy: Sized + Clone;
                
                /// 単頂点 `Value` を `Acc` 化する関数。
                fn to_acc(value: &Self::Value) -> Self::Acc;
                /// `Acc` のマージを行う関数。
                fn merge_acc(lhs: &Self::Acc, rhs: &Self::Acc) -> Self::Acc;
                /// 遅延作用 `Lazy` の反映を行う関数。`len` は `Acc` の対象となっている頂点数。
                fn act_lazy(value: &mut Self::Value, acc: &mut Self::Acc, lazy: &Self::Lazy, len: usize);
                /// 遅延作用 `Lazy` の合成を行う関数。
                fn comp_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy);
            }



            /// Splay 木
            /// 
            /// # 搭載トレイト
            /// 
            /// + `Clone`
            /// + `Debug where Ops::Value: Debug`
            /// + `FromIterator<Value>`
            /// + `IntoIterator for &SplayTree`
            /// + `Index<usize>`: `splay.get(i).unwrap()` と等価。
            /// 
            /// # 実装のメモ
            /// 
            /// `insert/delete` 系の破壊的な関数は `&mut self` で、`splay` しかしない関数は `&self` にしてある。
            pub struct SplayTree<Ops: SplayOps> (Cell<*mut Node<Ops>>);

            impl<Ops: SplayOps> SplayTree<Ops> {
                fn root(&self) -> *mut Node<Ops> { self.0.get() }
                
                pub fn new() -> Self { Self(Cell::new(null_mut())) }
                pub fn is_empty(&self) -> bool { self.root().is_null() }
                pub fn len(&self) -> usize { Node::get(self.root()).map_or(0, |root| root.len) }
                
                /// 指定した位置に挿入する。一番右なら `at = self.len` とすればよい。
                /// 
                /// # Panics
                /// 
                /// `!(at <= self.len)`
                pub fn insert(&mut self, at: usize, value: Ops::Value) {
                    assert!(at <= self.len());
                    let [l, r] = Node::split_at(self.root(), at);
                    let node = Node::new(value);
                    self.0.set(Node::merge_3(l, node, r));
                }
                
                /// 指定した位置を削除する。
                /// 
                /// # Panics
                /// 
                /// `!(at < self.len)`
                pub fn delete(&mut self, at: usize) -> Ops::Value {
                    assert!(at < self.len());
                    let (l, c, r) = Node::split_at_3(self.root(), at, at+1).unwrap();
                    self.0.set(Node::merge(l, r));
                    unsafe { Box::from_raw(c) }.value
                }
                
                /// `tree[range]` を逆順にする。
                pub fn reverse(&mut self, range: impl RangeBounds<usize>) {
                    let [st, ed] = range.as_bounds(self.len());
                    let Some((l, c, r)) = Node::split_at_3(self.root(), st, ed) else { return; };
                    c.rev ^= true;
                    c.push();
                    self.0.set(Node::merge_3(l, c, r));
                }
                
                /// `tree[range]` を畳み込んだ値を返す。区間幅が `0` のとき `None` を返す。
                pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<Ops::Acc> {
                    let [st, ed] = range.as_bounds(self.len());
                    let Some((l, c, r)) = Node::split_at_3(self.root(), st, ed) else { return None; };
                    c.update();
                    let res = c.acc.clone();
                    self.0.set(Node::merge_3(l, c, r));
                    Some(res)
                }
                
                /// `tree[range]` に `lazy` を作用させる。
                pub fn act(&mut self, range: impl RangeBounds<usize>, lazy: Ops::Lazy) {
                    let [st, ed] = range.as_bounds(self.len());
                    let Some((l, c, r)) = Node::split_at_3(self.root(), st, ed) else { return; };
                    c.lazy = Some(lazy);
                    c.push();
                    self.0.set(Node::merge_3(l, c, r));
                }
                
                /// 指定した位置の参照を返す。
                pub fn get(&self, i: usize) -> Option<&Ops::Value> {
                    if self.len() <= i { return None; }
                    let root = Node::splay_index(Node::unwrap(self.root()), i);
                    self.0.set(root);
                    Some(&root.value)
                }
                
                /// 指定した位置の可変参照を返す。
                pub fn get_mut(&mut self, i: usize) -> Option<RefMut<'_, Ops>> {
                    if self.len() <= i { return None; }
                    let root = Node::splay_index(Node::unwrap(self.root()), i);
                    self.0.set(root);
                    Some(RefMut(root))
                }
                
                /// `[self, ret] = [tree[..at], tree[at..]]; return ret;` をする。
                /// 
                /// # Panic
                /// 
                /// !(at <= self.len)
                pub fn split_off(&mut self, at: usize) -> Self {
                    assert!(at <= self.len());
                    let [left, right] = Node::split_at(self.root(), at);
                    self.0.set(left);
                    Self(Cell::new(right))
                }
                
                /// 木を右からマージする。
                pub fn merge(&mut self, right: Self) {
                    let tmp = Node::merge(self.root(), right.root());
                    self.0.set(tmp);
                }
                
                pub fn iter(&self) -> Iter<'_, Ops> {
                    Iter { splay: self, st: 0, ed: self.len() }
                }
                
                pub fn range(&self, range: impl RangeBounds<usize>) -> Iter<'_, Ops> {
                    let [st, ed] = range.as_bounds(self.len());
                    Iter { splay: self, st, ed }
                }
            }

            impl<Ops: SplayOps> FromIterator<Ops::Value> for SplayTree<Ops> {
                fn from_iter<T: IntoIterator<Item = Ops::Value>>(iter: T) -> Self {
                    let mut iter = iter.into_iter();
                    let mut root = if let Some(v) = iter.next() { Node::new(v) } else { return Self::new(); };
                    for v in iter {
                        let node = Node::new(v);
                        root.parent = node;
                        node.child[0] = root;
                        node.update();
                        root = node;
                    }
                    Self(Cell::new(root))
                }
            }

            impl<'a, Ops: SplayOps> IntoIterator for &'a SplayTree<Ops> {
                type Item = &'a Ops::Value;
                type IntoIter = Iter<'a, Ops>;
                fn into_iter(self) -> Self::IntoIter { self.iter() }
            }

            impl<Ops: SplayOps> Clone for SplayTree<Ops> {
                fn clone(&self) -> Self { self.iter().map(|v| v.clone()).collect() }
            }

            impl<Ops: SplayOps> Index<usize> for SplayTree<Ops> {
                type Output = Ops::Value;
                fn index(&self, index: usize) -> &Self::Output { assert!(index < self.len()); self.get(index).unwrap() }
            }

            impl<Ops: SplayOps> Debug for SplayTree<Ops> where Ops::Value: Debug {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_list().entries(self.iter()).finish()
                }
            }

            impl<Ops: SplayOps> Drop for SplayTree<Ops> {
                fn drop(&mut self) {}
            }



            /// [`SplayTree::entry`] の返り値型。
            /// 
            /// # Constraints
            /// 
            /// `entry` 対象である `self.0` が根であること。
            pub struct RefMut<'a, Ops: SplayOps>(&'a mut Node<Ops>);

            impl<Ops: SplayOps> Deref for RefMut<'_, Ops> {
                type Target = Ops::Value;
                fn deref(&self) -> &Self::Target { &self.0.value }
            }

            impl<Ops: SplayOps> DerefMut for RefMut<'_, Ops> {
                fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0.value }
            }

            impl<Ops: SplayOps> Drop for RefMut<'_, Ops> {
                fn drop(&mut self) { self.0.update(); }
            }



            /// [`SplayTree::iter`], [`SplayTree::range`] の返り値型。保持する区間は `st..ed` で表される。
            /// 
            /// [`Iter`] 存在中に根が変わったりするのは OK だけど、新たに値を追加するとかは NG なことに注意。
            pub struct Iter<'a, Ops: SplayOps> {
                splay: &'a SplayTree<Ops>,
                st: usize,
                ed: usize
            }

            impl<'a, Ops: SplayOps> Iterator for Iter<'a, Ops> {
                type Item = &'a Ops::Value;
                
                fn next(&mut self) -> Option<Self::Item> {
                    if self.st == self.ed { return None; }
                    self.st += 1;
                    Some(&self.splay[self.st-1])
                }
            }

            impl<'a, Ops: SplayOps> DoubleEndedIterator for Iter<'a, Ops> {
                fn next_back(&mut self) -> Option<Self::Item> {
                    if self.st == self.ed { return None; }
                    self.ed -= 1;
                    Some(&self.splay[self.ed])
                }
            }





            struct Node<Ops: SplayOps> {
                pub child: [*mut Self; 2],
                pub parent: *mut Self,
                pub len: usize,
                pub rev: bool,
                pub value: Ops::Value,
                pub acc: Ops::Acc,
                pub lazy: Option<Ops::Lazy>
            }

            impl<Ops: SplayOps> Node<Ops> {
                /// leak した `Node` を返す。
                fn new<'a>(value: Ops::Value) -> &'a mut Self {
                    let tmp = Self { child: [null_mut(); 2], parent: null_mut(), len: 1, rev: false, acc: Ops::to_acc(&value), value, lazy: None };
                    Box::leak(Box::new(tmp))
                }
                
                fn get<'a>(ptr: *mut Self) -> Option<&'a mut Self> { unsafe { ptr.as_mut() } }
                fn unwrap<'a>(ptr: *mut Self) -> &'a mut Self {
                    debug_assert!(!ptr.is_null());
                    unsafe { &mut *ptr }
                }
                
                /// 親 `p` に対する `self` の位置を返す。
                fn pos(&self, p: &Self) -> usize {
                    debug_assert!(ptr_eq(p.child[0], self) || ptr_eq(p.child[1], self));
                    ptr_eq(p.child[1], self) as usize
                }
                
                /// `self` が根になるよう、適切に回転させる。
                fn splay(&mut self) {
                    while let Some(p) = Node::get(self.parent) {
                        let pos = self.pos(p);
                        
                        if let Some(g) = Node::get(p.parent) {
                            if pos == p.pos(g) {
                                p.rotate(g, pos); self.rotate(p, pos);
                            } else {
                                self.rotate(p, pos); self.rotate(g, pos^1);
                            }
                        } else {
                            self.rotate(p, pos);
                        }
                    }
                }
                
                /// `self` が自分自身の親の位置に来るように回転させる。
                fn rotate(&mut self, p: &mut Self, pos: usize) {
                    self.push();
                    
                    // connect g --[pos]--> self
                    self.parent = p.parent;
                    if let Some(g) = Node::get(self.parent) { g.child[p.pos(g)] = self; }
                    
                    // connect p --[pos]--> self.child[pos^1]
                    p.child[pos] = self.child[pos^1];
                    if let Some(c) = Node::get(p.child[pos]) { c.parent = p; }
                    
                    // connect self --[pos^1]--> p
                    p.parent = self;
                    self.child[pos^1] = p;
                    
                    p.update(); self.update();
                }
                
                /// 遅延作用と反転を行い、子に伝搬させる。
                fn push(&mut self) {
                    if let Some(lazy) = self.lazy.take() {
                        Ops::act_lazy(&mut self.value, &mut self.acc, &lazy, self.len);
                        for c in self.child { Self::apply_lazy(c, &lazy); }
                    }
                    
                    if replace(&mut self.rev, false) {
                        self.child.swap(0, 1);
                        for c  in self.child { if let Some(c) = Node::get(c) { c.rev ^= true; } }
                    }
                }
                
                /// `dir` に遅延作用 `ad` を与える。
                fn apply_lazy(dir: *mut Self, ad: &Ops::Lazy) {
                    let Some(Node { lazy, .. }) = Node::get(dir) else { return; };
                    if let Some(lazy) = lazy {
                        Ops::comp_lazy(lazy, ad);
                    } else {
                        *lazy = Some(ad.clone());
                    }
                }
                
                /// `self.len, self.acc` を子の情報から更新する。
                fn update(&mut self) {
                    self.len = 1;
                    self.acc = Ops::to_acc(&self.value);
                    for pos in [0, 1] {
                        if let Some(c) = Node::get(self.child[pos]) {
                            c.push();
                            self.len += c.len;
                            self.acc = if pos == 0 { Ops::merge_acc(&c.acc, &self.acc) } else { Ops::merge_acc(&self.acc, &c.acc) };
                        }
                    }
                }
                
                /// 頂点 `i` を見つけて `splay` し、根である頂点 `i` を返す。
                /// 
                /// # Constraints
                /// 
                /// `i < root.len`
                fn splay_index(mut root: &mut Self, mut i: usize) -> &mut Self {
                    assert!(i < root.len);
                    loop {
                        root.push();
                        for c in root.child { if let Some(c) = Node::get(c) { c.push(); } }
                        let ls = Node::get(root.child[0]).map_or(0, |left| left.len);
                        
                        use std::cmp::Ordering::*;
                        root = match i.cmp(&ls) {
                            Less => { Node::unwrap(root.child[0]) }
                            Equal => { root.splay(); return root; }
                            Greater => { i -= ls + 1; Node::unwrap(root.child[1]) }
                        }
                    }
                }
                
                /// マージした木を返す。
                /// 
                /// `l` の一番右の頂点が根になるよう `splay` した後に `r` と繋げる。
                fn merge(l: *mut Self, r: *mut Self) -> *mut Self {
                    let Some(mut left) = Node::get(l) else { return r; };
                    let Some(right) = Node::get(r) else { return left; };
                    left = Self::splay_index(left, left.len-1);
                    left.push();
                    left.child[1] = right; right.parent = left;
                    left.update();
                    left
                }
                
                /// `tree[..at], tree[at..]` で構成される 2 つの木を返す。
                /// 
                /// # Constraints
                /// 
                /// `at <= root.len`
                fn split_at(root: *mut Self, at: usize) -> [*mut Self; 2] {
                    let Some(mut root) = Node::get(root) else { return [null_mut(), null_mut()]; };
                    if at == 0 { return [null_mut(), root]; }
                    if at == root.len { return [root, null_mut()]; }
                    root = Self::splay_index(root, at);
                    root.push();
                    let left = replace(&mut root.child[0], null_mut());
                    if let Some(left) = Node::get(left) {
                        left.parent = null_mut();
                        root.update();
                    }
                    [left, root]
                }
                
                /// `tree[..at1], tree[at1..at2], tree[at2..]` で構成される 2 つの木を返す。
                /// 
                /// # Constraints
                /// 
                /// `at1 <= at2 <= root.len`
                fn split_at_3<'a>(root: *mut Self, at1: usize, at2: usize) -> Option<(*mut Self, &'a mut Self, *mut Self)> {
                    if at1 == at2 { return None; }
                    let [c, r] = Self::split_at(root, at2);
                    let [l, c] = Self::split_at(c, at1);
                    Some((l, Node::unwrap(c), r))
                }
                
                /// マージした木を返す。
                fn merge_3(l: *mut Self, c: *mut Self, r: *mut Self) -> *mut Self {
                    Self::merge(Self::merge(l, c), r)
                }
            }
        }
        
        
    }
    
    pub mod algo {
        pub mod bellman_ford {
            use itertools::iproduct;
            use crate::{chmin, mylib::h64, nest};
            pub struct WarshallFloyd {
                d: Vec<Vec<h64>>,
                aft: Vec<Vec<usize>>
            }
            pub enum CostResult {
                Finite(i64),
                Unreachable
            }
            impl WarshallFloyd {
                pub fn new(len: usize, edge: &[(usize, usize, i64)]) -> Option<Self> {
                    let mut d = nest![h64::MAX; len; len];
                    let mut aft = nest![0; len; len];
                    for i in 0..len { d[i][i] = h64(0); }
                    for &(u, v, w) in edge { chmin!(d[u][v]; h64(w)); }
                    for (i, j) in iproduct!(0..len, 0..len) { aft[i][j] = j; }
                    for (k, i, j) in iproduct!(0..len, 0..len, 0..len) {
                        if chmin!(d[i][j]; d[i][k] + d[k][j]) { aft[i][j] = aft[i][k]; }
                        if i == j && d[i][j] < h64(0) { return None; }
                    }
                    Some(WarshallFloyd { d, aft })
                }
                pub fn cost(&self, u: usize, v: usize) -> CostResult {
                    if self.d[u][v] == h64::MAX { CostResult::Unreachable } else { CostResult::Finite(self.d[u][v].0) }
                }
                pub fn route(&self, mut u: usize, v: usize) -> Vec<usize> {
                    let mut ret = vec![];
                    loop {
                        ret.push(u); if u == v { break ret; } else { u = self.aft[u][v]; }
                    }
                }
            }
        }
        pub mod seq {
            use ac_library::string::{z_algorithm_arbitrary, suffix_array_arbitrary, lcp_array_arbitrary};
            pub fn z_algorithm<T: Ord>(v: &[T]) -> Vec<usize> { z_algorithm_arbitrary(v) }
            pub struct SuffixLcp<T: Ord> {
                s: Vec<T>,
                suffix: Vec<usize>,
                lcp: Vec<usize>
            }
            impl<T: Ord> SuffixLcp<T> {
                pub fn new(v: Vec<T>) -> Self {
                    let suffix = suffix_array_arbitrary(&v);
                    let lcp = lcp_array_arbitrary(&v, &suffix);
                    Self { s: v, suffix, lcp }
                }
                pub fn inner(&self) -> (&[usize], &[usize]) { (&self.suffix, &self.lcp) }
                pub fn lower_bound(&self, v: &[T]) -> usize {
                    let (mut ng, mut ok) = (usize::MAX, self.s.len()-1);
                    while let Some(mid) = crate::mylib::binary_search(ng, ok) {
                        *(if &self.s[mid..] < v { &mut ok } else { &mut ng }) = mid;
                    }
                    ok
                }
            }

        }
    }
    
    pub mod traits {
        pub mod abstracts {
            pub trait Monoid { const NOP: bool = false; type Set: Clone; fn e() -> Self::Set; fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set; fn debug(_: &Self::Set) -> String { "no info".into() } }
            pub trait Map<MnSet: Clone, const NOP: bool = false> { const NOP: bool = false; type F: Clone; fn id() -> Self::F; fn comp(f: &Self::F, g: &mut Self::F); fn map(f: &Self::F, x: &mut MnSet); fn debug(_: &Self::F) -> String { "no info".into() } }
        }
    }
    
    pub mod math {
        pub mod prime { use std::collections::BTreeMap; use crate::mylib::math::barrett::Barrett128; pub fn miller_rabin(n: u64) -> bool { let n = n as u128; if n < 67 || n % 2 == 0 { return [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61].contains(&n); } let l: &[u128] = if n < 4759123141 { &[2, 7, 61] } else { &[2, 325, 9375, 28178, 450775, 9780504, 1795265022] }; let s = (n-1).trailing_zeros(); let d = (n-1) >> s; let brt = Barrett128::new(n); 'a: for &a in l { let mut v = brt.pow(a, d as u128); if v == 1 || v == n-1 { continue; } for _ in 0..s-1 { v = brt.reduce(v*v); if v == n-1 { continue 'a; } } return false; } true } pub struct PrimeTable { max: usize, primes: Vec<usize>, lpf: Vec<usize> } impl PrimeTable { pub fn new(mut max: usize) -> Self { max = max.max(3); let mut primes = vec![]; let mut lpf = vec![0; max+1]; for i in 2..=max { if lpf[i] == 0 { primes.push(i); lpf[i] = i; } let lpfi = lpf[i]; for &p in primes.iter().take_while(move |&&p| p <= lpfi && i*p <= max) { lpf[i*p] = p; } } PrimeTable { max, primes, lpf } } pub fn is_prime(&self, n: usize) -> bool { if n <= self.max { self.lpf[n] == n } else { miller_rabin(n as u64) } } pub fn fact(&self, mut n: usize) -> BTreeMap<usize, usize> { assert_ne!(n, 0); let mut out = BTreeMap::new(); for p in Iterator::chain(self.primes.iter().cloned(), self.max+1..) { if n <= self.max { while n != 1 { *out.entry(self.lpf[n]).or_default() += 1; n /= self.lpf[n]; } break; } if n < p*p { *out.entry(n).or_default() += 1; break; } while n%p == 0 { n /= p; *out.entry(p).or_default() += 1; } } out } pub fn primes(&self) -> &[usize] { &self.primes } } }
        pub mod barrett {
            use crate::elsedef; #[derive(Clone, Copy)] pub struct Barrett64 { m: u64, minv: u64 } impl Barrett64 { pub fn new(modulo: u64) -> Self { assert!(modulo < 1<<32); Self { m: modulo, minv: !0 / modulo } } pub fn reduce(self, value: u64) -> u64 { let tmp = ((value as u128 * self.minv as u128 >> 64) as u64 + 1) * self.m; elsedef!(value < tmp; self.m) + value - tmp } } #[derive(Clone, Copy)] pub struct Barrett128 { m: u128, minv: u128 } impl Barrett128 { pub fn new(modulo: u128) -> Self { assert!(modulo < 1<<64); Self { m: modulo, minv: !0 / modulo } } pub fn reduce(self, value: u128) -> u128 { let (lu, ll) = (value >> 64, value & (1<<64)-1); let (ru, rl) = (self.minv >> 64, self.minv & (1<<64)-1); let mut tmp = (lu*ru + (ll*ru + lu*rl >> 64) + 2) * self.m; if value < tmp { tmp -= self.m; } if value < tmp { tmp -= self.m; } value - tmp } pub fn pow(self, mut a: u128, mut b: u128) -> u128 { let mut res = 1; while b != 0 { if b & 1 == 1 { res = self.reduce(res * a); } a = self.reduce(a*a); b >>= 1; } res } }
        }
        
        pub mod matrix {
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

        }
    }
    
    pub mod util {
        pub mod printer {
            #![allow(non_camel_case_types)] use std::{mem::{replace, transmute}, ops::{Not, Shl}, sync::{Mutex, MutexGuard, OnceLock}}; static INTERNAL: OnceLock<Mutex<Internal>> = OnceLock::new(); pub static out: Printer = Printer(&INTERNAL); pub struct Internal { buf: String, endf: EndFlag, prvf: PreviousFlag } #[derive(PartialEq, Eq)] pub enum EndFlag { DoNothing, LineFeed, Print } use PreviousFlag::*; #[derive(PartialEq, Eq, Clone, Copy)] enum PreviousFlag { Space, NoSpace, LineHead, } pub struct end; #[derive(Clone, Copy)] pub struct Printer<const sp: bool = true>(&'static OnceLock<Mutex<Internal>>); impl<const sp: bool> Printer<sp> { pub fn init(&self, endf: EndFlag) { let is_err = self.0.set(Mutex::new(Internal { buf: String::new(), endf, prvf: LineHead })).is_err(); if is_err { panic!("[@printer] Error: Second call of Printer::init"); } } fn get(&self) -> MutexGuard<Internal> { self.0.get().unwrap().lock().unwrap() } fn push(&self, v: impl PrinterDisplay) { self.get().push(v, sp); } pub fn print(&self) { self.get().print(); } } impl Internal { fn push(&mut self, v: impl PrinterDisplay, sp: bool) { let prvf = replace(&mut self.prvf, if sp {Space} else {NoSpace}); let buf = &mut self.buf; if prvf != LineHead && (prvf == Space || sp) { *buf += " "; } v.pdisp(sp, buf); } fn print(&mut self) { let prvf = replace(&mut self.prvf, LineHead); let buf = &mut self.buf; if prvf == LineHead { buf.pop(); } if buf.is_empty() { return; } if crate::mylib::SUBMISSION { println!("{buf}"); } else { eprint!("\x1b[32m"); for (i, s) in buf.split('\n').enumerate() { eprint!("{}", if i == 0 {">> "} else {"   "}); println!("{s}"); } eprint!("\x1b[0m"); } buf.clear(); } } impl<T: PrinterDisplay, const sp: bool> Shl<T> for Printer<sp> { type Output = Self; fn shl(self, v: T) -> Self { self.push(v); self } } impl Not for Printer<true> { type Output = Printer<false>; fn not(self) -> Printer<false> { unsafe { transmute(self) } } } impl<const sp: bool> Shl<end> for Printer<sp> { type Output = Self; fn shl(self, _: end) -> Self { let mut itn = self.0.get().unwrap().lock().unwrap(); use EndFlag::*; match itn.endf { Print => { itn.print(); } LineFeed => { itn.buf += "\n"; itn.prvf = LineHead; } DoNothing => {} } self } } trait PrinterDisplay { fn pdisp(&self, sp: bool, buf: &mut String); } macro_rules! fall { ($($t:ty),+) => { $( impl PrinterDisplay for $t { fn pdisp(&self, _: bool, buf: &mut String) { *buf += &format!("{self}"); } } )+ }; } fall!( u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64, ac_library::ModInt998244353, ac_library::ModInt1000000007 ); impl PrinterDisplay for char { fn pdisp(&self, _: bool, buf: &mut String) { buf.push(*self); } } impl PrinterDisplay for &str { fn pdisp(&self, _: bool, buf: &mut String) { buf.push_str(self); } } impl PrinterDisplay for &String { fn pdisp(&self, _: bool, buf: &mut String) { buf.push_str(self); } } impl PrinterDisplay for bool { fn pdisp(&self, _: bool, buf: &mut String) { *buf += if *self {"Yes"} else{ "No" }; } } impl<T: PrinterDisplay> PrinterDisplay for &[T] { fn pdisp(&self, sp: bool, buf: &mut String) { for e in *self { e.pdisp(sp, buf); if sp { *buf += " "; } } if sp && !self.is_empty() { buf.pop(); } } }
        }
        pub mod traits {
            use std::ops::{RangeBounds, Bound::*};
            pub trait RectUtil: Sized + Copy { type Rhs: Copy; const LRUD: [Self::Rhs; 4]; fn wrapping_add_signed(self, rhs: Self::Rhs) -> Self; fn apply_lrud(self) -> [Self; 4]; } impl RectUtil for (usize, usize) { type Rhs = (isize, isize); const LRUD: [Self::Rhs; 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; fn wrapping_add_signed(self, rhs: Self::Rhs) -> Self { (self.0.wrapping_add_signed(rhs.0), self.1.wrapping_add_signed(rhs.1)) } fn apply_lrud(self) -> [Self; 4] { Self::LRUD.map(|d| self.wrapping_add_signed(d)) } } pub trait CharUtil: Clone { const lower: [Self; 26]; const upper: [Self; 26]; fn lower_to_us(self) -> usize; fn upper_to_us(self) -> usize; fn flip(self) -> Self; fn as_lrud(self) -> usize; } impl CharUtil for char { const lower: [char; 26] = { let (mut out, mut i) = (['_'; 26], 0); while i < 26 { out[i] = (i+97) as u8 as char; i += 1; } out }; const upper: [char; 26] = { let (mut out, mut i) = (['_'; 26], 0); while i < 26 { out[i] = (i+65) as u8 as char; i += 1; } out }; fn lower_to_us(self) -> usize { debug_assert!('a' <= self && self <= 'z'); self as usize - 97 } fn upper_to_us(self) -> usize { debug_assert!('A' <= self && self <= 'Z'); self as usize - 65 } fn flip(self) -> Self { (self as u8 ^ 32) as char } fn as_lrud(mut self) -> usize { self = self.to_ascii_uppercase(); ['L', 'R', 'U', 'D'].into_iter().position(|v| v == self).unwrap() } }
            pub trait IntUtil: Copy { fn bit(self, n: usize) -> bool; } impl IntUtil for usize { fn bit(self, n: usize) -> bool { self>>n & 1 == 1 } }
            pub trait AsBounds: RangeBounds<usize> {
                fn as_bounds(&self, sup: usize) -> [usize; 2] {
                    let l = match self.start_bound() {
                        Included(&v) => v,
                        Excluded(&v) => v+1,
                        Unbounded => 0
                    };
                    
                    let r = match self.end_bound() {
                        Included(&v) => v+1,
                        Excluded(&v) => v,
                        Unbounded => sup
                    };
                    
                    assert!(l <= r && r <= sup, "valid: 0..{sup}\ninputed: {l}..{r}");
                    [l, r]
                }
            }
            impl<T: RangeBounds<usize>> AsBounds for T {}
        }
        pub mod macros {
            #[macro_export] macro_rules! epr { ($($args:tt)*) => {} } #[macro_export] macro_rules! nest { (void; $n:expr) => { vec![vec![]; $n] }; (void; $n:expr $(;$m:expr)+) => { vec![nest![void$(;$m)+]; $n] }; () => { vec![] }; ($e:expr; $n:expr) => { vec![$e; $n] }; ($e:expr; $n:expr $(;$m:expr)+) => { vec![nest![$e$(;$m)+]; $n] }; } #[macro_export] macro_rules! min { ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x<y {x} else {y}).unwrap() } } #[macro_export] macro_rules! max { ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x>y {x} else {y}).unwrap() } } #[macro_export] macro_rules! chmin { ($dst:expr; $($vl:expr),+) => { { let v = crate::min!($($vl),+); if v < $dst { $dst = v; true } else { false } } }; } #[macro_export] macro_rules! chmax { ($dst:expr; $($vl:expr),+) => { { let v = crate::max!($($vl),+); if $dst < v { $dst = v; true } else { false } } }; } #[macro_export] macro_rules! elsedef { ($cond:expr; $v:expr) => { if $cond {$v} else {Default::default()} } }
        }
        pub mod func {
            pub fn binary_search(low: usize, high: usize) -> Option<usize> { if 1 < high.wrapping_sub(low) { Some(low.wrapping_add(high)/2) } else { None } }
        }
        pub mod hyperint {
            #![allow(non_camel_case_types)] use std::{fmt::Debug, ops::{Add, Sub, Neg}}; #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)] pub struct h64(pub i64); impl h64 { pub const MIN: h64 = h64(i64::MIN+1); pub const MAX: h64 = h64(i64::MAX); pub fn new(value: i64) -> Self { assert!(i64::MIN+2 <= value && value != i64::MAX); h64(value) } pub fn is_min(self) -> bool { self == h64::MIN } pub fn is_max(self) -> bool { self == h64::MAX } pub fn is_minmax(self) -> bool { self == h64::MIN || self == h64::MAX } } impl Add for h64 { type Output = Self; fn add(self, rhs: Self) -> Self::Output { match (self, rhs) { (h64::MAX, h64::MIN) | (h64::MIN, h64::MAX) => panic!("[@h64] MAX+MIN is undefined."), (h64::MAX, _) | (_, h64::MAX) => h64::MAX, (h64::MIN, _) | (_, h64::MIN) => h64::MIN, (l, r) => h64::new(l.0+r.0) } } } impl Add<i64> for h64 { type Output = Self; fn add(self, rhs: i64) -> Self::Output { if self.is_minmax() { self } else { h64::new(self.0+rhs) } } } impl Neg for h64 { type Output = Self; fn neg(mut self) -> Self::Output { self.0 += -1; self } } impl Sub for h64 { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { self + -rhs } } impl Sub<i64> for h64 { type Output = Self; fn sub(self, rhs: i64) -> Self::Output { self + -rhs } } impl Debug for h64 { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { if *self == h64::MIN { write!(f, "MIN") } else if *self == h64::MAX { write!(f, "MAX") } else { write!(f, "{}", self.0) } } }
        }
    }
}
