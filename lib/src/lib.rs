pub mod print;
mod solver; pub use solver::*;

mod modulo; pub use modulo::*;
mod bit; pub use bit::*;
mod nest; pub use nest::*;
mod prime; pub use prime::*;
// mod unionfind; pub use unionfind::*;
mod others; pub use others::*;

// for debug
pub use proconio::source::once::OnceSource;

pub use {
    proconio::{input, marker::{Chars, Usize1 as usize1, Isize1 as isize1}},
    std::cmp::{min, max, Reverse as Rev},
    std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    std::mem::swap,
    itertools::Itertools,
    superslice::Ext,
    num_integer::{gcd, lcm}
};
