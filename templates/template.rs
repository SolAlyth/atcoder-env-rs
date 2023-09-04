#![allow(non_snake_case)]

#[cfg(debug_assertions)] #[allow(unused)] use lib::{*, eprintln};
#[cfg(not(debug_assertions))] use mylib::*;

#[allow(unused_must_use)]
fn solve(out: &Solver) {
    input! {
        /* $1 */
    }
    
    /* $0 */
}



fn main() {
    let out = Solver::new(false); solve(&out); out.print();
}

#[cfg(not(debug_assertions))] #[allow(dead_code)]
mod mylib {
    pub use {
        proconio::{input, marker::{Chars, Usize1 as usize1, Isize1 as isize1}},
        std::cmp::{min, max, Reverse as Rev},
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
        itertools::Itertools,
        superslice::Ext
    };
    
    pub fn yesno(b: bool) -> &'static str {if b{"Yes"}else{"No"}}
    
    use std::{fmt::Display, ops::{Shl, Shr}, cell::UnsafeCell};
    
    // bit
    #[derive(Clone, Copy)] struct Bit(usize);
    impl Bit { fn get(self, n: usize) -> bool {self.0>>n&1==1} fn set(self, n: usize) -> Bit {Bit(1>>n|self.0)} }
    impl Shl<usize> for Bit { type Output = Bit; fn shl(self, rhs: usize) -> Self::Output {self.set(rhs)} }
    impl Shr<usize> for Bit { type Output = bool; fn shr(self, rhs: usize) -> Self::Output {self.get(rhs)} }
    
    // mod
    pub const MOD: u128 = 998244353;
    pub fn mpow(b: u128, p: u128) -> u128 {if p<=1{b.pow(p as u32)%MOD}else{let sqr=mpow(b.pow(2)%MOD,p/2);if p%2==0{sqr}else{sqr*b%MOD}}}
    
    // others
    #[macro_export] macro_rules! nest {(void;$n:expr)=>{vec![vec![];$n]};(void;$n:expr$(;$m:expr)+)=>{vec![nest![void$(;$m)+];$n]};($e:expr;$n:expr)=>{vec![$e;$n]};($e:expr;$n:expr$(;$m:expr)+)=>{vec![nest![$e$(;$m)+];$n]};}
    #[macro_export] macro_rules! eprintln {($($args:tt)*)=>{}}
    
    // solver
    pub struct Solver { v: UnsafeCell<String>, b: bool }
    impl Solver {pub fn new(b: bool) -> Self {Solver{v:String::new().into(),b}} pub fn print(&self) {unsafe{let s=&mut*self.v.get();println!("{}", s);s.clear();}}}
    impl<T: Display> Shl<T> for &Solver {type Output=Self; fn shl(self,rhs:T)->Self::Output{unsafe{let s=&mut*self.v.get();if s.len()!=0{s.push(' ');}s.push_str(&format!("{}",rhs));}self}}
    #[allow(non_camel_case_types)] pub struct end;
    impl Shl<end> for &Solver {type Output=(); fn shl(self,_:end)->Self::Output{if cfg!(debug_assertions)||self.b{self.print();}()}}
}
