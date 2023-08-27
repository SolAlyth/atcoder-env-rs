mod print; pub use print::*;
mod solver; pub use solver::*;

mod modulo; pub use modulo::*;
mod bit; pub use bit::*;

// for debug
pub use proconio::source::once::OnceSource;

pub use {
    proconio::{input, marker::{Chars, Usize1}},
    std::cmp::{min, max, Reverse as Rev},
    std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    itertools::Itertools,
    superslice::Ext
};


pub fn yesno(b: bool) -> &'static str {if b{"Yes"}else{"No"}}

#[macro_export] macro_rules! nest {(void;$n:expr)=>{vec![vec![];$n]};(void;$n:expr$(;$m:expr)+)=>{vec![nest![void$(;$m)+];$n]};($e:expr;$n:expr)=>{vec![$e;$n]};($e:expr;$n:expr$(;$m:expr)+)=>{vec![nest![$e$(;$m)+];$n]};}
