#![allow(non_snake_case)]

#[cfg(debug_assertions)] #[allow(unused)] use lib::{*, eprintln};
#[cfg(not(debug_assertions))] use mylib::*;

#[allow(unused_must_use)]
fn solve(out: &Solver<true>) {
    input! {
        /* $1 */
    }
    
    /* $0 */
}



fn main() {
    let out = Solver::<true>::new(false); solve(&out); out.print();
}

#[cfg(not(debug_assertions))]
mod mylib {
    #![allow(dead_code, non_upper_case_globals)]
    pub use {
        proconio::{input, marker::{Chars, Usize1 as usize1, Isize1 as isize1}},
        std::cmp::{min, max, Reverse as Rev},
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
        std::mem::swap,
        itertools::Itertools,
        superslice::Ext,
        num_integer::{gcd, lcm}
    };
    
    pub fn yesno(b: bool) -> &'static str {if b{"Yes"}else{"No"}}
    
    use std::{fmt::Display, ops::{Shl, Not}, cell::UnsafeCell, mem::transmute};
    
    // bit
    trait Bit {
        fn get(&self, n: usize) -> bool;
        fn set_true(&mut self, n: usize);
        fn set_false(&mut self, n: usize);
    }
    impl Bit for usize {
        fn get(&self, n: usize) -> bool {self>>n&1==1}
        fn set_true(&mut self, n: usize) {*self|=1<<n;}
        fn set_false(&mut self, n: usize) {*self&=!(1<<n);}
    }
    
    // modulo
    pub const i998: i128 = 998244353;
    pub const u998: u128 = 998244353;

    pub trait Mod {
        fn normalize(&self) -> Self;
        fn inv(&self) -> Self;
        fn mpow(self, p: Self) -> Self;
    }
    impl Mod for i128 {
        fn normalize(&self) -> Self {let a=self%i998;if a<0{a+i998}else{a}}
        fn inv(&self) -> Self {self.mpow(i998-2)}
        fn mpow(mut self, mut p: Self) -> Self {let mut a=1;while p!=0{if p&1==1{a=a*self%i998;}self=self*self%i998;p>>=1;}a}
    }
    
    // prime
    const lsize: usize = 1_000_000;
    const plsize: usize = 200_000;

    pub struct PrimeTable { pub prime: Vec<usize>, lpf: Vec<usize> }
    impl PrimeTable {
        pub fn new() -> Self {let mut prime=vec![0;plsize];let mut lpf=vec![0;lsize];for i in 2..lsize{if lpf[0]==0{prime.push(i);lpf[i]=i;}for &j in &prime{lpf[i*j]=j;}}Self{prime,lpf}}
        pub fn fact(&self, mut v: usize) -> HashMap<usize, usize> {let mut map=HashMap::new();if lsize<v{'a: for &p in self{while Self::trial(p,&mut v,&mut map){if v<=lsize{break 'a;}}}map.insert(v,1);return map;}while v!=1{*map.entry(self.lpf[v]).or_insert(0)+=1;v/=self.lpf[v];}map}
        pub fn fact_spec(prime: &[usize], mut v: usize) -> HashMap<usize, usize> {let mut map = HashMap::new();for &p in prime{while Self::trial(p,&mut v,&mut map){if v==1{return map;}}}map.insert(v,1);map}
        pub fn fact_trial(mut v: usize) -> HashMap<usize, usize> {let mut map=HashMap::new();if v==1{return map;}for p in 2..{while Self::trial(p,&mut v,&mut map){if v==1{return map;}}}map}
        pub fn is_prime(&self, v: usize) -> bool {self.lpf[v]==v}
        fn trial(p: usize, v: &mut usize, map: &mut HashMap<usize, usize>) -> bool {*v%p==0&&{*v/=p;*map.entry(p).or_insert(0)+=1;true}}
    }
    impl<'a> IntoIterator for &'a PrimeTable {type Item=&'a usize;type IntoIter=std::slice::Iter<'a,usize>;fn into_iter(self)->Self::IntoIter{self.prime.iter()}}

    // others
    #[macro_export] macro_rules! nest {(void;$n:expr)=>{vec![vec![];$n]};(void;$n:expr$(;$m:expr)+)=>{vec![nest![void$(;$m)+];$n]};($e:expr;$n:expr)=>{vec![$e;$n]};($e:expr;$n:expr$(;$m:expr)+)=>{vec![nest![$e$(;$m)+];$n]};}
    #[macro_export] macro_rules! eprintln {($($args:tt)*)=>{}}
    
    // solver
    pub struct Solver<const sp: bool> { v: UnsafeCell<String>, b: bool, bf: UnsafeCell<bool> }
    impl<const sp: bool> Solver<sp> {
        pub fn new(b: bool) -> Solver<true> {Solver::<true>{v:String::new().into(),b,bf:true.into()}}
        pub fn swapbf(&self, mut v: bool) -> bool {unsafe{swap(&mut*self.bf.get(),&mut v)}v}
        pub fn push(&self, v: &str) {unsafe{let s=&mut*self.v.get();if sp||self.swapbf(sp)&&s.len()!=0{s.push(' ');}s.push_str(v);}}
        pub fn print(&self) {unsafe{let s=&mut*self.v.get();if s.len()!=0{println!("{}", s);s.clear();}}}
    }
    impl<T: Display, const sp: bool> Shl<T> for &Solver<sp> {type Output=Self;fn shl(self,rhs:T)->Self::Output{self.push(&format!("{}",rhs)); self}}
    #[allow(non_camel_case_types)] pub struct end;
    impl<const sp: bool> Shl<end> for &Solver<sp> {type Output=();fn shl(self,_:end)->Self::Output{self.swapbf(true);if cfg!(debug_assertions)||self.b{self.print();}()}}
    impl<'a> Not for &'a Solver<true> {type Output=&'a Solver<false>;fn not(self)->Self::Output{unsafe{transmute(self)}}}
}
