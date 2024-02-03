#![allow(non_snake_case)]

use mylib::*;

const INTERACTIVE: bool = false;

#[allow(unused_must_use)]
fn solve<_T: BufRead>(#[allow(unused)] out: &Printer, mut stdin: impl Source<_T>) {
    macro_rules! input {($($t:tt)*)=>{mylib::input!(from &mut stdin, $($t)*);}}
    input! {
        /* $1 */
    }
    
    /* $0 */
}


const INPUT: &str = "";


fn main() {
    let out = Printer::new(INTERACTIVE || !mylib::SUBMISSION);
    if !mylib::SUBMISSION {
        if INPUT != "" {
            solve(&out, OnceSource::from(INPUT));
        } else {
            solve(&out, LineSource::new(BufReader::new(stdin())));
        }
    } else {
        if !INTERACTIVE {
            solve(&out, OnceSource::new(BufReader::new(stdin())));
        } else {
            solve(&out, LineSource::new(BufReader::new(stdin())));
        }
    }
    out.print();
}

// You can see my library at https://github.com/SolAlyth/atcoder-env-rs
#[cfg(not(debug_assertions))] #[allow(unused)]
mod mylib {
    #![allow(non_upper_case_globals)]
    
    pub const SUBMISSION: bool = true;
    
    pub const us998: usize = 998244353;
    pub const i998: i128 = 998244353;
    pub const us107: usize = 1000000007;
    pub const i107: i128 = 1000000007;
    
    const usmod: usize = us998;
    const imod: i128 = i998;
    
    pub use {
        math::modulo::Modulo,
        util::{
            printer::{Printer, end},
            traits::{AssignMinMax, CharFn}
        },
        
        proconio::{input, marker::{Chars as chars, Usize1 as usize1, Isize1 as isize1}, source::{Source, line::LineSource, once::OnceSource}},
        std::io::{BufReader, BufRead, stdin},
        std::cmp::{min, max, Reverse as Rev},
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
        std::mem::swap,
        itertools::Itertools,
        superslice::Ext,
        num_integer::{gcd, lcm, Roots}
    };
    
    
    pub mod data_struct {
        pub mod bitset {
            use std::{ops::{BitAnd, BitOr, BitXor, Deref, Index, Not}, fmt::Debug};
            use crate::mylib::util::iter::*;
            #[derive(Clone, Copy)] pub struct BitSet { value: usize, len: usize }
            impl BitSet { pub fn new(value: bool, len: usize) -> Self { BitSet { value: if value {!0} else {0}, len }.masked() } fn masked(mut self) -> Self { self.value &= BitSet::max(self.len); self } pub const fn sup(len: usize) -> usize { 1<<len } pub const fn max(len: usize) -> usize { Self::sup(len)-1 } pub fn generate(len: usize) -> impl DoubleEndedIterator<Item = Self> { (0..Self::sup(len)).map(move |i| BitSet { value: i, len }) } fn get_raw(&self, idx: usize) -> usize { assert!(idx < self.len); self.value>>idx & 1 } pub fn set(&mut self, idx: usize, value: bool) { assert!(idx < self.len); if value { self.value |= 1<<idx; } else { self.value &= !(1<<idx); } } pub fn count_true(&self) -> usize { self.value.count_ones() as usize } pub fn count_false(&self) -> usize { self.len - self.count_true() } pub fn is_full(&self) -> bool { self.value == BitSet::max(self.len) } pub fn is_empty(&self) -> bool { self.value == 0 } }
            impl BitAnd for BitSet { type Output = Self; fn bitand(mut self, rhs: Self) -> Self::Output { assert_eq!(self.len, rhs.len); self.value &= rhs.value; self } }
            impl BitOr for BitSet { type Output = Self; fn bitor(mut self, rhs: Self) -> Self::Output { assert_eq!(self.len, rhs.len); self.value |= rhs.value; self } }
            impl BitXor for BitSet { type Output = Self; fn bitxor(mut self, rhs: Self) -> Self::Output { assert_eq!(self.len, rhs.len); self.value ^= rhs.value; self } }
            impl Not for BitSet { type Output = Self; fn not(mut self) -> Self::Output { self.value = !self.value; self.masked() } }
            impl Deref for BitSet { type Target = usize; fn deref(&self) -> &Self::Target { &self.value } }
            impl Debug for BitSet { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?} ({})", self.into_iter().collect::<Vec<_>>(), self.value) } }
            impl Index<usize> for BitSet { type Output = bool; fn index(&self, index: usize) -> &Self::Output { static A: [bool; 2] = [false, true]; &A[self.get_raw(index)] } }
            impl GenericIterable for BitSet { type Item = bool; fn giter_next(&mut self, i: usize) -> Option<Self::Item> { if i < self.len { Some(self[i]) } else { None } } }
            impl IntoIterator for BitSet { type Item = (usize, <Self as GenericIterable>::Item); type IntoIter = GenericIter<Self>; fn into_iter(self) -> Self::IntoIter { self.into() } }
        }
        pub mod compress {
            use std::{collections::HashMap, hash::Hash};
            pub struct Numbering<T: Eq + Hash + Clone> { map: HashMap<T, usize>, vec: Vec<T> }
            impl<T: Eq + Hash + Clone> Numbering<T> { pub fn new() -> Self { Numbering { map: HashMap::new(), vec: vec![] } } pub fn entry(&mut self, key: &T) -> usize { if !self.map.contains_key(key) { self.map.insert(key.clone(), self.vec.len()); self.vec.push(key.clone()); } self.map[key] } pub fn get(&self, index: usize) -> &T { &self.vec[index] } }
            pub struct Compress<T: Ord> (Vec<T>);
            impl<T: Ord> Compress<T> { pub fn new() -> Self { Compress(vec![]) } pub fn insert(&mut self, key: T) { self.0.push(key); } pub fn insert_with(&mut self, iter: impl Iterator<Item = T>) { for key in iter { self.insert(key); } } pub fn calc(mut self) -> Compressed<T> { self.0.sort_unstable(); self.0.dedup(); Compressed(self.0) } }
            pub struct Compressed<T: Ord> (Vec<T>);
            impl<T: Ord> Compressed<T> { pub fn entry(&self, key: &T) -> usize { self.0.binary_search(key).unwrap() } pub fn get(&self, idx: usize) -> &T { &self.0[idx] } }
        }
        pub mod unionfind {
            use crate::nest;
            #[derive(Clone, Copy)] enum Node { Leader(usize), Child(usize) }
            #[derive(Clone)] pub struct UnionFind { nodes: Vec<Node> }
            impl UnionFind { pub fn new(len: usize) -> Self { UnionFind { nodes: vec![Node::Leader(1); len] } } fn leader_and_size(&mut self, u: usize) -> (usize, usize) { let mut now = u; let mut stack = vec![]; let (leader, size) = loop { match self.nodes[now] { Node::Leader(size) => { break (now, size); } Node::Child(par) => { stack.push(now); now = par; } } }; for &i in stack.iter().rev() { self.nodes[i] = Node::Child(leader); } (leader, size) } pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 } pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 } pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) } pub fn merge(&mut self, u: usize, v: usize) -> bool { let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v)); if us < vs { std::mem::swap(&mut ul, &mut vl); } if ul != vl { self.nodes[ul] = Node::Leader(us+vs); self.nodes[vl] = Node::Child(ul); } ul != vl } pub fn group(&mut self, mut u: usize) -> Vec<usize> { u = self.leader(u); (0..self.nodes.len()).filter(|&v| self.leader(v) == u).collect() } pub fn groups(&mut self) -> Vec<Vec<usize>> { let mut out = nest![void; self.nodes.len()]; for u in 0..self.nodes.len() { out[self.leader(u)].push(u); } out.retain(|v| v.len() != 0); out } }
            type WeightType = i128;
            #[derive(Clone)] pub struct WeightedUnionFind { nodes: Vec<Node>, diff: Vec<WeightType> }
            impl WeightedUnionFind { pub fn new(len: usize) -> Self { WeightedUnionFind { nodes: vec![Node::Leader(1); len], diff: vec![0; len] } } fn leader_and_size(&mut self, u: usize) -> (usize, usize) { let mut now = u; let mut stack = vec![]; let (leader, size) = loop { match self.nodes[now] { Node::Leader(size) => { break (now, size); } Node::Child(par) => { stack.push(now); now = par; } } }; for &child in stack.iter().rev() { let Node::Child(parent) = self.nodes[child] else { unreachable!(); }; self.nodes[child] = Node::Child(leader); self.diff[child] += self.diff[parent]; } (leader, size) } pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 } pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 } pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) } pub fn weight(&mut self, u: usize, v: usize) -> Result<WeightType, ()> { if self.leader(u) == self.leader(v) { Ok(-self.diff[u] + self.diff[v]) } else { Err(()) } } pub fn merge(&mut self, u: usize, v: usize, mut w: WeightType) -> Result<bool, ()> { let ((mut ul, us), (mut vl, vs)) = (self.leader_and_size(u), self.leader_and_size(v)); if us < vs { std::mem::swap(&mut ul, &mut vl); w = -w; } if ul != vl { self.nodes[ul] = Node::Leader(us+vs); self.nodes[vl] = Node::Child(ul); Ok(true) } else { if self.weight(u, v).unwrap() == w { Ok(false) } else { Err(()) } } } pub fn group(&mut self, mut u: usize) -> Vec<usize> { u = self.leader(u); (0..self.nodes.len()).filter(|&v| self.leader(v) == u).collect() } pub fn groups(&mut self) -> Vec<Vec<usize>> { let mut out = nest![void; self.nodes.len()]; for u in 0..self.nodes.len() { out[self.leader(u)].push(u); } out.retain(|v| v.len() != 0); out } }
        }
    }
    
    pub mod math {
        pub mod modulo {
            use super::super::{imod, usmod};
            pub trait Modulo: Copy { fn simplify(self) -> Self; fn mpow(self, a: usize) -> Self; fn minv_fermat(self) -> Self { self.mpow(usmod-2) } }
            impl Modulo for i128 { fn simplify(mut self) -> Self { if !(0..imod).contains(&self) { self %= imod; if self < 0 { self += imod; } } self } fn mpow(mut self, mut a: usize) -> Self { let mut out = 1; while a != 0 { if a&1 == 1 { out = (out * self).simplify(); } self = self.pow(2).simplify(); a >>= 1; } out } }
        }
    }
    
    pub mod util {
        pub mod printer {
            #![allow(non_camel_case_types, non_upper_case_globals)]
            use { std::{ops::{Shl, Not}, cell::{UnsafeCell, Cell}, mem::transmute}, itertools::Itertools };
            #[macro_export] macro_rules! pr { ($($args:tt)*) => { println!($($args)*); } }
            #[macro_export] macro_rules! epr { ($($args:tt)*) => { } }
            pub struct Printer<const sp: bool = true> { out: UnsafeCell<String>, endf: bool, bsp: Cell<bool> }
            impl Printer { pub fn new(endf: bool) -> Self { Printer { out: String::new().into(), endf, bsp: true.into() } } }
            impl<const sp: bool> Printer<sp> { fn push(&self, v: &str) { unsafe { let s = &mut *self.out.get(); if (self.bsp.replace(sp) || sp) && !s.is_empty() { *s += " "; } *s += v; } } pub fn print(&self) { unsafe { let s = &mut *self.out.get(); if !s.is_empty() { pr!("{}", s); s.clear(); } } } }
            impl<T: PrinterDisplay, const sp: bool> Shl<T> for &Printer<sp> { type Output = Self; fn shl(self, rhs: T) -> Self::Output { self.push(&rhs.pdisp(sp)); self } }
            impl<'a> Not for &'a Printer<true> { type Output = &'a Printer<false>; fn not(self) -> Self::Output { unsafe { transmute(self) } } }
            pub struct end;
            impl<const sp: bool> Shl<end> for &Printer<sp> { type Output = (); fn shl(self, _: end) -> Self::Output { self.bsp.replace(true); if self.endf { self.print(); } } }
            trait PrinterDisplay { fn pdisp(&self, sp: bool) -> String; }
            trait PrimitivePrinterDisplay: PrinterDisplay {}
            macro_rules! fall { ($($t:ty);+) => { $( impl PrinterDisplay for $t { fn pdisp(&self, _: bool) -> String { format!("{}", self) } } impl PrimitivePrinterDisplay for $t {} )+ }; }
            fall!( u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char; &str; &String; String );
            impl PrinterDisplay for bool { fn pdisp(&self, _: bool) -> String { String::from(if *self {"Yes"} else {"No"}) } }
            impl PrimitivePrinterDisplay for bool {}
            impl<T: PrimitivePrinterDisplay> PrinterDisplay for Vec<T> { fn pdisp(&self, sp: bool) -> String { self.iter().map(|v| v.pdisp(sp)).join(if sp {" "} else {""}) } }
            impl<T: PrimitivePrinterDisplay> PrinterDisplay for &[T] { fn pdisp(&self, sp: bool) -> String { self.iter().map(|v| v.pdisp(sp)).join(if sp {" "} else {""}) } }
        }
        pub mod traits {
            pub trait AssignMinMax: Sized + PartialOrd { fn assign_max(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_lt() { *self = value; } } fn assign_min(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_gt() { *self = value; } } }
            macro_rules! impl_update { ($($t:ty);+) => { $( impl AssignMinMax for $t {} )+ }; }
            impl_update!(u8; u16; u32; u64; u128; i8; i16; i32; i64; i128; f32; f64);
            pub trait CharFn: Copy { fn add(self, v: isize) -> Self; fn to_lower(self) -> Self; fn to_upper(self) -> Self; fn lower_to_us(self) -> usize; fn upper_to_us(self) -> usize; fn num_to_us(self) -> usize; fn into_lower(v: usize) -> Self; fn into_upper(v: usize) -> Self; }
            impl CharFn for char { fn add(self, v: isize) -> Self { (self as isize + v) as u8 as char } fn to_lower(self) -> Self { self.add(32) } fn to_upper(self) -> Self { self.add(-32) } fn lower_to_us(self) -> usize { self as usize - 97 } fn upper_to_us(self) -> usize { self as usize - 65 } fn num_to_us(self) -> usize { self as usize - 48 } fn into_lower(v: usize) -> Self { (v+97) as u8 as char } fn into_upper(v: usize) -> Self { (v+65) as u8 as char } }
        }
        pub mod macros {
            #[macro_export] macro_rules! nest { (void; $n:expr) => { vec![vec![];$n] }; (void; $n:expr $(;$m:expr)+) => { vec![nest![void$(;$m)+]; $n] }; ($e:expr; $n:expr) => { vec![$e; $n] }; ($e:expr; $n:expr $(;$m:expr)+) => { vec![nest![$e$(;$m)+]; $n] }; }
        }
        pub mod iter {
            pub struct GenericIter<T: GenericIterable>(T, usize);
            pub trait GenericIterable: Sized { type Item; fn giter_next(&mut self, i: usize) -> Option<Self::Item>; }
            impl<T: GenericIterable> From<T> for GenericIter<T> { fn from(value: T) -> Self { GenericIter(value, 0) } }
            impl<T: GenericIterable> Iterator for GenericIter<T> { type Item = (usize, T::Item); fn next(&mut self) -> Option<Self::Item> { let tmp = self.0.giter_next(self.1); self.1 += 1; tmp.map(|v| (self.1, v)) } }
        }
    }
}
