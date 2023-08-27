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
        proconio::{input, marker::{Chars, Usize1}},
        std::cmp::{min, max, Reverse as Rev},
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
        itertools::Itertools,
        superslice::Ext
    };
    
    pub fn yesno(b: bool) -> &'static str {if b{"Yes"}else{"No"}}
    
    // bit
    pub trait Bit { fn nth_bit(self, n: usize) -> Self; fn add_nth_bit(self, n: usize) -> Self; }
    impl Bit for usize {fn nth_bit(self,n:usize)->Self{self>>n&1}fn add_nth_bit(self,n:usize)->Self{self|(1<<n)}}
    
    // mod
    pub const MOD: u128 = 998244353;
    pub fn mpow(b: u128, p: u128) -> u128 {if p<=1{b.pow(p as u32)%MOD}else{let sqr=mpow(b.pow(2)%MOD,p/2);if p%2==0{sqr}else{sqr*b%MOD}}}
    
    // others
    #[macro_export] macro_rules! nest {(void;$n:expr)=>{vec![vec![];$n]};(void;$n:expr$(;$m:expr)+)=>{vec![nest![void$(;$m)+];$n]};($e:expr;$n:expr)=>{vec![$e;$n]};($e:expr;$n:expr$(;$m:expr)+)=>{vec![nest![$e$(;$m)+];$n]};}
    #[macro_export] macro_rules! eprintln {($($args:tt)*)=>{}}
    
    use std::{fmt::Display, ops::Shl, cell::UnsafeCell};
    pub struct Solver { v: UnsafeCell<String>, b: bool }
    impl Solver {pub fn new(b: bool) -> Self {Solver{v:String::new().into(),b}} pub fn print(&self) {unsafe{let s=&mut*self.v.get();println!("{}", s);s.clear();}}}
    impl<T: Display> Shl<T> for &Solver {type Output=Self; fn shl(self,rhs:T)->Self::Output{unsafe{let s=&mut*self.v.get();if s.len()!=0{s.push(' ');}s.push_str(&format!("{}",rhs));}self}}
    #[allow(non_camel_case_types)] pub struct end;
    impl Shl<end> for &Solver {type Output=(); fn shl(self,_:end)->Self::Output{if cfg!(debug_assertions)||self.b{self.print();}()}}
}
