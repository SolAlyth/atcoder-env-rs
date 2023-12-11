#![allow(non_snake_case)]

use lib::*;

const INTERACTIVE: bool = false;
const INPUT: &str = "";

#[allow(unused_must_use)]
fn solve<_T: BufRead>(#[allow(unused)] out: &Printer, mut stdin: impl Source<_T>) {
    macro_rules! input {($($t:tt)*)=>{lib::input!(from &mut stdin, $($t)*);}}
    input! {
        /* $1 */
    }
    
    /* $0 */
}



fn main() {
    let out = Printer::new(INTERACTIVE);
    if !lib::SUBMISSION {
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
            unionfind::UnionFind,
            compress::{Numbering, Compress}
        },
        math::modulo::{i998, us998, Mod, ModCalc},
        
        util::{
            printer::{Printer, end},
            traits::{Update, CharFn}
        }
    };
    
    pub const SUBMISSION: bool = true;
    
    pub mod data_struct {
        pub mod bitset {
            use {std::ops::{Deref, Index, BitAnd, BitOr, BitXor, Not}, itertools::Itertools};
            #[derive(Clone, Copy)] pub struct BoolSet { pub value: usize, pub size: usize }
            impl BoolSet { fn sup(size: usize) -> usize { assert!(size < 64); 1<<size } pub fn max(size: usize) -> usize { Self::sup(size)-1 } pub fn gen(size: usize) -> impl Iterator<Item = Self> { (0..Self::sup(size)).map(move |i| BoolSet { value: i, size }) } pub fn get(&self, idx: usize) -> bool { assert!(idx < self.size); self.value>>idx & 1 == 1 } pub fn set(&mut self, idx: usize, value: bool) { assert!(idx < self.size); if value { self.value |= 1<<idx; } else { self.value &= !(1<<idx); } } pub fn count_true(&self) -> usize { self.value.count_ones() as usize } pub fn count_false(&self) -> usize { self.size - self.count_true() } pub fn is_empty(&self) -> bool { self.value == 0 } }
            impl BitAnd for BoolSet { type Output = Self; fn bitand(mut self, rhs: Self) -> Self::Output { self.value &= rhs.value; self } }
            impl BitOr for BoolSet { type Output = Self; fn bitor(mut self, rhs: Self) -> Self::Output { self.value |= rhs.value; self } }
            impl BitXor for BoolSet { type Output = Self; fn bitxor(mut self, rhs: Self) -> Self::Output { self.value ^= rhs.value; self } }
            impl Not for BoolSet { type Output = Self; fn not(mut self) -> Self::Output { self.value = !self.value; self } }
            impl Deref for BoolSet { type Target = usize; fn deref(&self) -> &Self::Target { &self.value } }
        }
        pub mod compress {
            use std::{collections::HashMap, hash::Hash, ops::Index};
            pub struct Numbering<T: Eq + Hash + Clone> { map: HashMap<T, usize>, vec: Vec<T> }
            impl<T: Eq + Hash + Clone> Numbering<T> { pub fn new() -> Self { Numbering { map: HashMap::new(), vec: vec![] } } pub fn entry(&mut self, key: &T) -> usize { if self.map.contains_key(key) { self.map.insert(key.clone(), self.vec.len()); self.vec.push(key.clone()); } self.map[key] } pub fn get(&self, index: usize) -> &T { &self.vec[index] } }
            impl<T: Eq + Hash + Clone> Index<usize> for Numbering<T> { type Output = T; fn index(&self, index: usize) -> &Self::Output { &self.get(index) } }
            impl<T: Eq + Hash + Clone> Index<&T> for Numbering<T> { type Output = usize; fn index(&self, key: &T) -> &Self::Output { &self.map[key] } }
            pub struct Compress<T: PartialOrd + Clone> { vec: Vec<T>, sorted: bool }
            impl<T: PartialOrd + Clone> Compress<T> { pub fn new() -> Self { Compress { vec: vec![], sorted: false } } pub fn insert(&mut self, key: &T) { assert!(!self.sorted); self.vec.push(key.clone()); } pub fn calc(&mut self) { assert!(!self.sorted); self.sorted = true; self.vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); self.vec.dedup(); } pub fn get(&self, key: &T) -> usize { assert!(self.sorted); self.vec.binary_search_by(|v| v.partial_cmp(key).unwrap()).unwrap() } }
            impl<T: PartialOrd + Clone> Index<usize> for Compress<T> { type Output = T; fn index(&self, index: usize) -> &Self::Output { &self.vec[index] } }
        }
        pub mod unionfind {
            use std::mem::swap;
            #[derive(Clone, Copy)] pub enum Item { Leader(usize), Child(usize) }
            pub struct UnionFindCore<T, U> { data: Vec<Item>, pub value: Vec<T>, cmpf: fn(&mut [T], usize, usize, usize), mgf: fn(&mut [T], usize, usize, usize, usize, U) -> bool, invf: fn(&mut U) }
            impl<T, U> UnionFindCore<T, U> { fn compress(&mut self, child: usize, parent_new: usize) { let Item::Child(parent_old) = self.data[child] else { assert_eq!(child, parent_new); return; }; if parent_old == parent_new { return; } self.compress(parent_old, parent_new); self.data[child] = Item::Child(parent_new); (self.cmpf)(&mut self.value, child, parent_old, parent_new); } fn leader_and_size(&mut self, u: usize) -> (usize, usize) { match self.data[u] { Item::Leader(size) => { (u, size) } Item::Child(par) => { let (leader, size) = self.leader_and_size(par); self.compress(u, leader); (leader, size) } } } pub fn merge(&mut self, mut child: usize, mut parent: usize, mut arg: U) -> Result<bool, ()> { if self.size(parent) < self.size(child) { swap(&mut parent, &mut child); (self.invf)(&mut arg); } let ((cl, cs), (pl, ps)) = (self.leader_and_size(child), self.leader_and_size(parent)); let res = (self.mgf)(&mut self.value, child, cl, parent, pl, arg); if res { if cl != pl { self.data[pl] = Item::Leader(ps+cs); self.data[cl] = Item::Child(pl); Ok(true) } else { Ok(false) } } else { Err(()) } } pub fn leader(&mut self, u: usize) -> usize { self.leader_and_size(u).0 } pub fn size(&mut self, u: usize) -> usize { self.leader_and_size(u).1 } pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) } pub fn len(&self) -> usize { self.data.len() } pub fn group(&mut self, u: usize) -> Vec<usize> { (0..self.len()).filter(|&v| self.is_same(u, v)).collect() } pub fn groups(&mut self) -> Vec<Vec<usize>> { let mut out = vec![vec![]; self.len()]; for u in 0..self.len() { out[self.leader(u)].push(u); } out.dedup(); out } }
            pub struct UnionFind { inner: UnionFindCore<(), ()> }
            impl UnionFind { pub fn new(size: usize) -> Self { UnionFind { inner: UnionFindCore { data: vec![Item::Leader(1); size], value: vec![(); size], cmpf: |_: &mut _, _, _, _| {}, mgf: |_: &mut _, _, _, _, _, _| true, invf: |_: &mut _| {} } } } pub fn leader(&mut self, u: usize) -> usize { self.inner.leader(u) } pub fn size(&mut self, u: usize) -> usize { self.inner.size(u) } pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.inner.is_same(u, v) } pub fn len(&self) -> usize { self.inner.len() } pub fn group(&mut self, u: usize) -> Vec<usize> { self.inner.group(u) } pub fn groups(&mut self) -> Vec<Vec<usize>> { self.inner.groups() } pub fn merge(&mut self, u: usize, v: usize) -> bool { self.inner.merge(u, v, ()).unwrap() } }
            type WeightType = i128;
            pub struct WeightedUnionFind { inner: UnionFindCore<WeightType, WeightType> }
            impl WeightedUnionFind { pub fn new(size: usize) -> Self { WeightedUnionFind { inner: UnionFindCore { data: vec![Item::Leader(1); size], value: vec![0; size], cmpf: |value: &mut [WeightType], child, parent_old, _| { value[child] += value[parent_old]; }, mgf: |value: &mut [WeightType], c, cl, p, pl, arg| { if cl != pl { value[cl] = arg + value[p] - value[c]; true } else { value[p] - value[c] == arg } }, invf: |arg: &mut _| { *arg *= -1; } } } } pub fn leader(&mut self, u: usize) -> usize { self.inner.leader(u) } pub fn size(&mut self, u: usize) -> usize { self.inner.size(u) } pub fn is_same(&mut self, u: usize, v: usize) -> bool { self.inner.is_same(u, v) } pub fn len(&self) -> usize { self.inner.len() } pub fn group(&mut self, u: usize) -> Vec<usize> { self.inner.group(u) } pub fn groups(&mut self) -> Vec<Vec<usize>> { self.inner.groups() } pub fn merge(&mut self, child: usize, parent: usize, arg: WeightType) -> Result<bool, ()> { self.inner.merge(child, parent, arg) } pub fn dist(&mut self, child: usize, parent: usize) -> Option<WeightType> { if self.is_same(child, parent) { Some(self.inner.value[child] - self.inner.value[parent]) } else { None } } }
        }
    }
    
    pub mod math {
        pub mod modulo {
            #![allow(non_upper_case_globals)]
            pub const i998: i128 = 998244353;
            pub const us998: usize = 998244353;
            pub trait Mod: Copy { fn normalize(self) -> Self; fn mpow(self, p: usize) -> Self; fn update(&mut self) -> Self { *self = self.normalize(); *self } fn inv(self) -> Self { self.mpow(998244353-2) } }
            impl Mod for i128 { fn normalize(mut self) -> Self { if self < 0 || i998 <= self { self %= i998; if self < 0 { self += i998; } } self } fn mpow(mut self, mut p: usize) -> Self { let mut out = 1; while p != 0 { if p&1 == 1 { out *= self; out.update(); } self = self.pow(2).normalize(); p >>= 1; } out } }
            impl Mod for usize { fn normalize(mut self) -> Self { if us998 <= self { self %= us998; } self } fn mpow(mut self, mut p: usize) -> Self { let mut out = 1; while p != 0 { if p&1 == 1 { out *= self; out.update(); } self = (self.pow(2)).normalize(); p >>= 1; } out } }
            #[derive(Default)] pub struct ModCalc { fact: Vec<i128>, fact_inv: Vec<i128>, inv: Vec<i128> }
            impl ModCalc { pub fn new() -> Self { ModCalc { fact: vec![1], fact_inv: vec![1], inv: vec![0, 1] } } fn update_factrial(&mut self, n: usize) { if n < self.fact.len() { return; } self.fact.reserve(n+1); for i in self.fact.len()..=n { self.fact[i] = (self.fact[i-1] * i as i128).normalize(); } } fn update_factorial_inv(&mut self, mut n: usize) { if n < self.fact_inv.len() { return; } self.fact_inv.resize(n+1, 0); self.fact_inv[n] = self.factorial(n).inv(); while self.fact_inv[n-1] == 0 { self.fact_inv[n-1] = (self.fact_inv[n] * n as i128).normalize(); n -= 1; } } fn update_inv_linear(&mut self, n: usize) { if n < self.inv.len() { return; } for i in self.inv.len()..=n { self.inv[i] = (i998 - (i998/i as i128) * self.inv[us998%i]).normalize(); } } pub fn factorial(&mut self, n: usize) -> i128 { self.update_factrial(n); self.fact[n] } pub fn factorial_inv(&mut self, n: usize) -> i128 { self.update_factorial_inv(n); self.fact_inv[n] } pub fn combination(&mut self, n: usize, k: usize) -> i128 { self.factorial(n) * self.factorial_inv(k) * self.factorial_inv(n-k) } pub fn parmutation(&mut self, n: usize, k: usize) -> i128 { self.factorial(n) * self.factorial_inv(n-k) } pub fn inv_linear(&mut self, mut value: i128) -> i128 { assert!(value != 0); self.update_inv_linear(value.update() as usize); self.inv[value as usize] } }
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
        pub mod traits {
            pub trait Update: Sized + PartialOrd { fn update_max(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_lt() { *self = value; } } fn update_min(&mut self, value: Self) { if (self as &Self).partial_cmp(&value).unwrap().is_gt() { *self = value; } } }
            macro_rules! impl_update { ($($t:ty);+) => { $( impl Update for $t {} )+ }; }
            impl_update!(u8; u16; u32; u64; u128; i8; i16; i32; i64; i128; f32; f64);
            pub trait CharFn: Copy { fn add(self, v: isize) -> Self; fn to_lower(self) -> Self; fn to_upper(self) -> Self; fn lower_to_us(self) -> usize; fn upper_to_us(self) -> usize; fn num_to_us(self) -> usize; }
            impl CharFn for char { fn add(self, v: isize) -> Self { (self as isize + v) as u8 as char } fn to_lower(self) -> Self { self.add(32) } fn to_upper(self) -> Self { self.add(-32) } fn lower_to_us(self) -> usize { self as usize - 97 } fn upper_to_us(self) -> usize { self as usize - 65 } fn num_to_us(self) -> usize { self as usize - 48 } }
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
