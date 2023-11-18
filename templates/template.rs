#![allow(non_snake_case)]

use lib::*;

const INTERACTIVE: bool = false;
const INPUT: &str = "";

#[allow(unused_must_use)]
fn solve<_T: BufRead>(out: &Printer, mut stdin: impl Source<_T>) {
    macro_rules! input {($($t:tt)*)=>{lib::input!(from &mut stdin, $($t)*);}}
    input! {
        /* $1 */
    }
    
    /* $0 */
}



fn main() {
    let out = Printer::new(INTERACTIVE);
    if cfg!(debug_assertions) {
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
#[cfg(not(debug_assertions))]
mod lib {
    pub use {
        proconio::{input, marker::{Chars as chars, Usize1 as usize1, Isize1 as isize1}, source::{Source, line::LineSource, once::OnceSource}},
        std::io::{BufReader, BufRead, stdin},
        std::cmp::{min, max, Reverse as Rev},
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
        std::mem::swap,
        itertools::Itertools,
        superslice::Ext,
        num_integer::{gcd, lcm}
    };
    
    pub use {
        data_struct::{
            bitset::BoolSet,
            unionfind::UnionFind
        },
        util::{
            printer::{Printer, end},
            macros
        }
    };
    
    pub mod data_struct {
        pub mod bitset {
            use {std::ops::{Deref, Index, IndexMut, BitAnd, BitOr, BitXor, Not}, itertools::Itertools};
            #[derive(Clone, Copy)] pub struct BoolSet { pub value: usize, pub size: usize }
            impl BoolSet { fn sup(size: usize) -> usize { assert!(size < 64); 1<<size } pub fn len(size: usize) -> usize { Self::sup(size) } pub fn max(size: usize) -> usize { Self::sup(size)-1 } pub fn gen(size: usize) -> impl Iterator<Item = Self> { let f = move |i| BoolSet { value: i, size }; (0..Self::sup(size)).map(f) } pub fn get(&self, idx: usize) -> bool { assert!(idx < 64); self.value>>idx & 1 == 1 } pub fn set(&mut self, idx: usize, value: bool) { if value { self.set_true(idx); } else { self.set_false(idx); } } pub fn set_true(&mut self, idx: usize) { assert!(idx < 64); self.value |= 1<<idx; } pub fn set_false(&mut self, idx: usize) { assert!(idx < 64); self.value &= !(1<<idx); } pub fn count_true(&self) -> usize { self.value.count_ones() as usize } pub fn count_false(&self) -> usize { self.size - self.count_true() } pub fn increment(mut self) -> Option<Self> { if self.value != Self::max(self.size) { self.value += 1; Some(self) } else { None } } pub fn is_empty(&self) -> bool { self.value == 0 } }
            impl BitAnd for BoolSet { type Output = Self; fn bitand(mut self, rhs: Self) -> Self::Output { self.value &= rhs.value; self } }
            impl BitOr for BoolSet { type Output = Self; fn bitor(mut self, rhs: Self) -> Self::Output { self.value |= rhs.value; self } }
            impl BitXor for BoolSet { type Output = Self; fn bitxor(mut self, rhs: Self) -> Self::Output { self.value ^= rhs.value; self } }
            impl Not for BoolSet { type Output = Self; fn not(mut self) -> Self::Output { self.value = !self.value; self } }
            impl Deref for BoolSet { type Target = usize; fn deref(&self) -> &Self::Target { &self.value } }
            #[derive(Clone)] pub struct BitSet<'a> { bits: &'a [usize], data: Vec<usize> }
            impl<'a> BitSet<'a> { pub fn min(bits: &'a [usize]) -> Self { Self { bits, data: vec![0; bits.len()] } } pub fn max(bits: &'a [usize]) -> Self { Self { bits, data: bits.into_iter().map(|&v| v-1).collect_vec() } } pub fn increment(mut self) -> Option<Self> { for (i, v) in self.data.iter_mut().enumerate() { if *v != self.bits[i]-1 { *v += 1; return Some(self) } else { *v = 0; } } None } pub fn decrement(mut self) -> Option<Self> { for (i, v) in self.data.iter_mut().enumerate() { if *v != 0 { *v -= 1; return Some(self); } else { *v = self.bits[i]-1; } } None } }
            impl<'a> Index<usize> for BitSet<'a> { type Output = usize; fn index(&self, index: usize) -> &Self::Output { &self.data[index] } }
            impl<'a> IndexMut<usize> for BitSet<'a> { fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.data[index] } }
        }
        pub mod unionfind {
            use {std::mem::swap, itertools::Itertools};
            #[derive(Clone, Copy)] pub(crate) enum Item { Parent(usize), Size(usize) }
            pub struct UnionFind { pub(crate) data: Vec<Item> }
            impl UnionFind { pub fn new(size: usize) -> Self { Self { data: vec![Item::Size(1); size] } } pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.parent_and_size(u).0 == self.parent_and_size(v).0 } fn parent_and_size(&mut self, i: usize) -> (usize, usize) { match self.data[i] { Item::Size(size) => { (i, size) } Item::Parent(par) => { let (parpar, pars) = self.parent_and_size(par); if par != parpar { self.data[i] = Item::Parent(parpar); } (parpar, pars) } } } pub(crate) fn parent_and_size_with(&mut self, i: usize, mergef: &mut impl FnMut(usize, usize)) -> (usize, usize) { match self.data[i] { Item::Size(size) => { (i, size) } Item::Parent(par) => { let (parpar, pars) = self.parent_and_size_with(par, mergef); if par != parpar { self.data[i] = Item::Parent(parpar); mergef(i, par); } (parpar, pars) } } } pub fn merge(&mut self, u: usize, v: usize) -> bool { let (u, us) = self.parent_and_size(u); let (v, vs) = self.parent_and_size(v); if u == v { return false; } let (child, par) = if us <= vs { (u, v) } else { (v, u) }; self.data[par] = Item::Size(us + vs); self.data[child] = Item::Parent(par); true } pub fn size(&mut self, i: usize) -> usize { self.parent_and_size(i).1 } pub fn subset(&mut self, i: usize) -> Vec<usize> { (0..self.data.len()).filter(|&v| self.is_same(i, v)).collect_vec() } pub fn partition(&mut self) -> Vec<Vec<usize>> { let mut out = crate::nest!(void; self.data.len()); for i in 0..self.data.len() { out[self.parent_and_size(i).0].push(i); } out.retain(|v| v.len() != 0); out } }
            type WeightType = i128;
            pub struct WeightedUnionFind { uf: UnionFind, diff: Vec<WeightType> }
            impl WeightedUnionFind { pub fn new(size: usize) -> Self { Self { uf: UnionFind::new(size), diff: vec![0; size] } } fn parent_and_size(&mut self, i: usize) -> (usize, usize) { let mut weight_merge = |child, par| { self.diff[child] += self.diff[par]; }; self.uf.parent_and_size_with(i, &mut weight_merge) } pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.parent_and_size(u).0 == self.parent_and_size(v).0 } pub fn merge(&mut self, u: usize, v: usize, mut weight: WeightType) -> Result<bool, ()> { let (mut child, childs) = self.parent_and_size(u); let (mut par, pars) = self.parent_and_size(v); if child == par { return if self.diff[v] - self.diff[u] == weight { Ok(false) } else { Err(()) } } weight = -self.diff[u] + weight + self.diff[v]; if childs > pars { swap(&mut child, &mut par); weight *= -1; } self.uf.data[child] = Item::Parent(par); self.uf.data[par] = Item::Size(childs + pars); self.diff[child] = weight; Ok(true) } pub fn dist(&mut self, u: usize, v: usize) -> Result<WeightType, ()> { if self.is_same(u, v) { Ok(self.diff[u] - self.diff[v]) } else { Err(()) } } pub fn size(&mut self, i: usize) -> usize { self.parent_and_size(i).1 } pub fn subset(&mut self, i: usize) -> Vec<usize> { self.uf.subset(i) } pub fn partition(&mut self) -> Vec<Vec<usize>> { self.uf.partition() } }
        }
    }
    
    pub mod util {
        pub mod printer {
            #![allow(non_camel_case_types, non_upper_case_globals)]
            use { std::{ops::{Shl, Not}, cell::UnsafeCell, mem::{transmute, swap}}, itertools::Itertools };
            pub struct Printer<const sp: bool = true> { v: UnsafeCell<String>, endf: bool, spf: UnsafeCell<bool> }
            impl Printer { pub fn new(endf: bool) -> Self { Printer { v: String::new().into(), endf, spf: true.into() } } }
            impl<const sp: bool> Printer<sp> { fn swap_spf(&self, mut f: bool) -> bool { unsafe { swap(&mut *self.spf.get(), &mut f) } f} fn push(&self, v: &str) { unsafe { let s = &mut *self.v.get(); if (self.swap_spf(sp) || sp) && !s.is_empty() { *s += " "; } *s += v; } } pub fn print(&self) { unsafe { let s = &mut *self.v.get(); if !s.is_empty() { crate::pr!("{}", s); s.clear(); } } } }
            impl<T: PrinterDisplay, const sp: bool> Shl<T> for &Printer<sp> { type Output = Self; fn shl(self, rhs: T) -> Self::Output { self.push(&rhs.pdisp(sp)); self } }
            impl<'a> Not for &'a Printer<true> { type Output = &'a Printer<false>; fn not(self) -> Self::Output { unsafe { transmute(self) } } }
            pub struct end;
            impl<const sp: bool> Shl<end> for &Printer<sp> { type Output=(); fn shl(self, _:end) -> Self::Output { self.swap_spf(true); if self.endf { self.print(); } } }
            trait PrinterDisplay { fn pdisp(&self, sp: bool) -> String; }
            trait PrimitivePrinterDisplay: PrinterDisplay {}
            macro_rules! fall { ($($t:ty);+) => { $( impl PrinterDisplay for $t { fn pdisp(&self, _: bool) -> String { format!("{}", self) } } impl PrimitivePrinterDisplay for $t {} )+ }; }
            fall!( u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char; &str; String );
            impl PrinterDisplay for bool { fn pdisp(&self, _: bool) -> String { String::from(if *self {"Yes"} else {"No"}) } }
            impl PrimitivePrinterDisplay for bool {}
            impl<T: PrimitivePrinterDisplay> PrinterDisplay for Vec<T> { fn pdisp(&self, sp: bool) -> String { self.iter().map(|v| v.pdisp(sp)).join(if sp {" "} else {""}) } }
            impl<T: PrimitivePrinterDisplay> PrinterDisplay for &[T] { fn pdisp(&self, sp: bool) -> String { self.iter().map(|v| v.pdisp(sp)).join(if sp {" "} else {""}) } }
        }
        pub mod color_print {
            #[macro_export] macro_rules! pr { ($($args:tt)*) => { println!($($args)*); } }
            #[macro_export] macro_rules! epr { ($($args:tt)*) => { } }
        }
        pub mod macros {
            #[macro_export] macro_rules! nest { (void; $n:expr) => { vec![vec![];$n] }; (void; $n:expr $(;$m:expr)+) => { vec![nest![void$(;$m)+]; $n] }; ($e:expr; $n:expr) => { vec![$e; $n] }; ($e:expr; $n:expr $(;$m:expr)+) => { vec![nest![$e$(;$m)+]; $n] }; }
        }
    }
}
